use std::thread;

const LOOPS: i32 = 100_000_000;
static mut COUNTER: i32 = 0;

fn main() {
    // Rust 裡面修改全域可變變數是 unsafe 的
    let t1 = thread::spawn(|| {
        for _ in 0..LOOPS {
            unsafe {
                COUNTER += 1;
            }
        }
    });

    let t2 = thread::spawn(|| {
        for _ in 0..LOOPS {
            unsafe {
                COUNTER -= 1;
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        println!("counter={}", COUNTER);
    }
}