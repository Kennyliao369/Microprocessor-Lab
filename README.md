# Microprocesser System Lab

NTUT 2026 Y3S1 Microprocessor System Lab Class Project

Cross compilation from x86-archlinux to Jetson-TX2

> Not include class PDF and Report

## Environment

- Host: x86-archlinux
- Target: Jetson-TX2
- C++ cross compilation: gcc-linaro-7.3.1-2018.05-x86_64_aarch64-linux-gnu
- Qt: qtcreator
- Rust target: aarch64-unknown-linux-gnu

## 目錄結構

```
.
├── Lab1
│   ├── Lab01_第03組
│   └── docs.txt
├── Lab2
│   ├── Lab02_第03組
│   ├── lab02
│   └── docs.txt
├── Lab3
│   ├── Lab03_第03組
│   └── docs.txt
└── Lab4
    └── lab04
```

> `LabXX_第XX組` 目錄放置課程標準作業
> 
> `labXX` 目錄放置進階功能的 Rust 課外專案
>
> `docs.txt` 為專案筆記

## Lab1: 嵌入式系統編譯

**目標**

- 使用 ssh 與 Jetson Tx2 平臺連線，並撰寫 Makefile
- 跨平臺編譯程式，傳輸到 Tx2 嵌入式系統上執行

**實作內容**

簡單寫了一個 Makefile ，並跨平臺編譯

**建制及執行**

todo!()

## Lab2: 基礎 GPIO 控制

**目標**

- 瞭解 Liunx 檔案系統的讀取、寫入，以及 physical pin 對應的 system file
- 學習 C 語言，透過指令傳入參數、撰寫 Makefile、跨平臺編譯環境
- 最後使用 C/C++ 控制 Tx2 GPIO 上面的 LED 燈珠

**C++ 實作內容**

Makefile 簡單修改延用 Lab1

gpio.cpp 當成 gpio 控制的方便接口。main.cpp 做參數解析、流程控制

**Rust 實作內容**

環境建制沒有遇到什麼重大問題，跨平臺便宜甚至比 C++ 方便

lib.rs 做 gpio 底層控制。main.rs 做參數解析、流程控制

多虧於 Rust 的錯誤處理方式，達成較少的代碼、更安全的錯誤處理

結構簡單明瞭

**建制及執行**

todo!()

## Lab3: 嵌入式 QT 視窗程式開發

**目標**

- QT 應用程式設計，瞭解「事件驅動」、「訊號與槽」機制
- 利用 QT 操作 Tx2 上面的 GPIO
- 完成簡易的 GUI LED 控制系統

**實作內容**

本實驗只能使用 C++ ，因為是 QT

界面拖拽了好久，為了讓 GUI 能自適應

主程式使用 u8 模擬 u4 當作 led 的座標，通過 左移/右移/翻轉 達成 Switch 、Running

還製作了簡單的打地鼠遊戲

自認為程式寫的工整、好讀、高效

Gpio 檔案操作的部分是 Claude 生成的，因為寫太多次，要吐了

感想是，QT 太多專用類型，要是入坑的話，會出不來，更何況我不想碰 C++ 呀

**建制及執行**

todo!()

## Lab4: 構建 web sever

**目標**

- 利用 Node.js 在 TX2 上面假設網頁伺服器
- 在網頁上操作，並控制 TX2 上面的 LED 燈珠
- 包含 LED 點亮、閃爍、流水燈、反向流水

**實作內容**

如果以為我會去寫 Node.js 就太天真了

題外話：執行子行程腳本控制 LED ，在多請求時會衝突

這種底層控制要達到最佳效率還得是 Rust 呀

這東西寫了好久。寫太底層了，用到一堆複雜概念。處理了幾乎所有可能的作物，受益良多

包含的內容：

- http 請求的接受請求、解析參數、回應請求，從底層實作了一次
- Leds 類型統一控制 gpio ，shine, switch, running, stop
- 製作線程池，同時接受多個請求
- 原子引用計數，共享信道到多個線程
- 信道收發，線程中共享信道
- 使用簡易併發，async 、await，防止 switch 、running 阻塞線程
- 使用 `Arc<AtomicBool>` ，原子引用 + 原子布林，在線程中共享可變數值
- 寫 html 。當按下按鈕，主動向服務器發起請求。接受結果，送回界面上

運作原理：

1. main 先開一個 handle_leds 的線程，此線程等待控制 led 的請求
2. main 持續等待 tcp 連接，收到後交給線程池執行 handle_connection 
3. handle_connection 解析 uri 並向 handle_connection 線程發送控制消息，並等待回信
4. handle_leds 線程收到 shine ，開始執行任務，並回覆結果
5. handle_leds 線程收到 switch 或是 running 會異步執行，這時還是可以繼續接受其他訊息，並返回 gpio 忙碌中
6. handle_leds 線程如果收到 stop 會主動打斷目前執行的 switch 或是 running ，並等待下次請求
7. handle_leds 線程如果收到 reverse ，會將 `Arc<AtomicBool>` 反向，而 running 會讀取 AtomicBool 及時判斷方向
8. handle_connection 收到 Leds 線程回信後，整理並向客戶返回訊息

行為模式：

- client 同時發送多個 shine 指令不衝突，會順序執行完
- client 發送 switch 馬上又發送 shine ，回報 gpio 忙碌中
- client 連續發送 running ，回報 gpio 忙碌中
- 亂送請求，回報錯誤

**建制及執行**

todo!()
