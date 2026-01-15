use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 使用 Arc (Atomic Reference Counted) 讓多個執行緒可以同時持有 mutex 的所有權
    // Mutex<()> 表示我們只想要鎖的功能，不一定要保護特定的資料
    let x = Arc::new(Mutex::new(()));
    let y = Arc::new(Mutex::new(()));

    // 複製引用給執行緒 A
    let x_a = Arc::clone(&x);
    let y_a = Arc::clone(&y);

    let thread_a = thread::spawn(move || {
        // 嘗試鎖定 x
        let _lock_x = x_a.lock().unwrap();
        println!("A lock x");

        // 故意停頓，確保執行緒 B 有時間鎖定 y
        thread::sleep(Duration::from_secs(1));

        println!("A 嘗試鎖定 y...");
        let _lock_y = y_a.lock().unwrap(); // 這裡會發生死結
        println!("A lock y");

        println!("finished A");
    });

    // 複製引用給執行緒 B
    let x_b = Arc::clone(&x);
    let y_b = Arc::clone(&y);

    let thread_b = thread::spawn(move || {
        // 嘗試鎖定 y (與 A 的順序相反)
        let _lock_y = y_b.lock().unwrap();
        println!("B lock y");

        // 故意停頓，確保執行緒 A 有時間鎖定 x
        thread::sleep(Duration::from_secs(1));

        println!("B 嘗試鎖定 x...");
        let _lock_x = x_b.lock().unwrap(); // 這裡會發生死結
        println!("B lock x");

        println!("finished B");
    });

    // 等待執行緒結束 (在死結發生時，這裡永遠不會結束)
    thread_a.join().unwrap();
    thread_b.join().unwrap();
}