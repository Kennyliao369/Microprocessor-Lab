use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        // 這邊需要使用 &mut 。因為如果直接使用 Vec 的話，for 預設會使用 into_iter ，這會奪取所有權
        // 而這邊的 self 只有 &mut ，不能直接從結構體裡面偷所有權
        //
        // 但是使用了 &mut 之後還是會有問題
        // 因為 worker 會變成 &mut
        // 而 join 操作會奪取 worker.thread 所有權
        // 同樣會有 &mut struct 不能奪取欄位的問題
        // for worker in &mut self.workers {
        //     println!("Shutdown worker {}", worker.id);
        //     worker.thread.join().unwrap();
        // }

        // 當然也可以使用 Option 將 thread 包起來
        // 但是我們總是知道 Option 會有值
        // 卻還要包起來，所以可以考慮更好的優化方式

        // 透過調用 worker.drain 的方式，指定要從 Vec 移除的值
        // 並返回這些項目的迭代器
        // .. 可以指定移除所有值
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}
