use std::sync::{Arc, Mutex};
use std::thread;

const LOOPS: i32 = 100_000_000;

fn main() {
    // Arc 讓多個執行緒可以共同擁有這個變數，Mutex 保證一次只有一個執行緒能修改
    let counter = Arc::new(Mutex::new(0));

    let c1 = Arc::clone(&counter);
    let t1 = thread::spawn(move || {
        for _ in 0..LOOPS {
            let mut num = c1.lock().unwrap();
            *num += 1;
        }
    });

    let c2 = Arc::clone(&counter);
    let t2 = thread::spawn(move || {
        for _ in 0..LOOPS {
            let mut num = c2.lock().unwrap();
            *num -= 1;
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("counter={}", *counter.lock().unwrap());
}