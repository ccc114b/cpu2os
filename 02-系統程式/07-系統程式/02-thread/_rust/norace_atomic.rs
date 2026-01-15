use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

const LOOPS: i32 = 100_000_000;
// 使用原子整數，可以在執行緒間安全共享
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    // 建立第一個執行緒 (inc)
    let t1 = thread::spawn(|| {
        for _ in 0..LOOPS {
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    });

    // 建立第二個執行緒 (dec)
    let t2 = thread::spawn(|| {
        for _ in 0..LOOPS {
            COUNTER.fetch_sub(1, Ordering::Relaxed);
        }
    });

    // 等待執行緒結束 (等同於 pthread_join)
    t1.join().unwrap();
    t2.join().unwrap();

    println!("counter={}", COUNTER.load(Ordering::Relaxed));
}