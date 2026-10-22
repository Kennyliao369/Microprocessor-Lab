use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
    },
    thread,
    time::Duration,
};

mod command;
use command::LedMode;

use led_panel::Leds;
use thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    let (led_panel_tx, led_panel_rx) = trpl::channel::<(LedMode, Sender<Result<String, String>>)>();

    // pin: 7 15 29 33
    let leds = Arc::new(Leds::new(&[396, 255, 398, 389]));
    if let Err(e) = leds.exports() {
        eprintln!("Problem with export gpios: {e}");
    }
    if let Err(e) = leds.set_direction("out") {
        eprintln!("Problem with set gpios direction : {e}");
    }

    thread::spawn(move || {
        trpl::block_on(handle_leds(leds, led_panel_rx));
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let led_panel_tx = led_panel_tx.clone();
        pool.execute(move || handle_connection(stream, led_panel_tx));
    }
}

fn handle_connection(
    mut stream: TcpStream,
    led_panel_tx: trpl::Sender<(LedMode, Sender<Result<String, String>>)>,
) {
    let buf = BufReader::new(&stream);
    let Some(Ok(request_line)) = buf.lines().next() else {
        return;
    };

    let (respons_line, respons_body) = match request_line.as_str() {
        "GET / HTTP/1.1" => {
            let body = fs::read_to_string("index.html").unwrap();
            ("HTTP/1.1 200 OK", body)
        }
        request_line => {
            let Some(uri) = request_line.split_whitespace().nth(1) else {
                return;
            };
            match LedMode::from_uri(uri) {
                Ok(led_mode) => {
                    let (reply_tx, reply_rx) = mpsc::channel();
                    if led_panel_tx.send((led_mode, reply_tx)).is_err() {
                        (
                            "HTTP/1.1 503 SERVICE UNAVAILABLE",
                            String::from("LED controller is down"),
                        )
                    } else {
                        match reply_rx.recv_timeout(Duration::from_millis(500)) {
                            Ok(Ok(respons)) => ("HTTP/1.1 200 OK", respons),
                            Ok(Err(e)) => ("HTTP/1.1 400 BAD REQUEST", e.to_string()),
                            Err(_) => (
                                "HTTP/1.1 503 SERVICE UNAVAILABLE",
                                String::from("LED controller not respond"),
                            ),
                        }
                    }
                }
                Err(e) => ("HTTP/1.1 400 BAD REQUEST", e),
            }
        }
    };

    let len = respons_body.len();
    let respons = format!("{respons_line}\r\nContent-Length:{len}\r\n\r\n{respons_body}");
    stream.write_all(respons.as_bytes()).unwrap();
}

async fn handle_leds(
    leds: Arc<Leds>,
    mut led_panel_rx: trpl::Receiver<(LedMode, Sender<Result<String, String>>)>,
) {
    let mut handle: trpl::JoinHandle<()> = trpl::spawn_task(async {});
    let reverse: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    while let Some((led_mode, reply_tx)) = led_panel_rx.recv().await {
        match led_mode {
            LedMode::Shine { index, power } => {
                if !handle.is_finished() {
                    let _ = reply_tx.send(Err(format!("LED{index} still busy")));
                    continue;
                }
                match leds.shine(index, power) {
                    Ok(()) => {
                        let _ = reply_tx.send(Ok(format!(
                            "LED{index} set to {}",
                            if power { "ON" } else { "OFF" }
                        )));
                    }
                    Err(e) => {
                        let _ = reply_tx.send(Err(format!("LED{index} failed: {e}")));
                    }
                }
            }
            LedMode::Switch { times } => {
                if !handle.is_finished() {
                    let _ = reply_tx.send(Err(String::from("LED still busy")));
                    continue;
                }
                let leds2 = Arc::clone(&leds);
                handle = trpl::spawn_task(async move { leds2.switch(times).await });
                let _ = reply_tx.send(Ok(String::from("LED start switching")));
            }
            LedMode::Running => {
                if !handle.is_finished() {
                    let _ = reply_tx.send(Err(String::from("LED still busy")));
                    continue;
                }
                let leds2 = Arc::clone(&leds);
                let reverse = Arc::clone(&reverse);
                handle = trpl::spawn_task(async move { leds2.running(&reverse).await });
                let _ = reply_tx.send(Ok(String::from("LED start running")));
            }
            LedMode::Reverse => {
                let now = !reverse.fetch_not(Ordering::Relaxed);
                let dir = if now { "reverse" } else { "forward" };
                let _ = reply_tx.send(Ok(format!("LED direction: {dir}")));
            }
            LedMode::Stop => {
                let is_running = !handle.is_finished();
                if is_running {
                    handle.abort();
                    let _ = (&mut handle).await;
                }
                let _ = leds.all_off();

                let msg = if is_running {
                    "LED stopped"
                } else {
                    "All LED off"
                };
                let _ = reply_tx.send(Ok(String::from(msg)));
            }
        }
    }
}
