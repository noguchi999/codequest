use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 0..=5 {
        println!("{}: {}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

pub fn main() {
    let handle1 = thread::spawn(|| sleep_print("thread1"));
    let handle2 = thread::spawn(|| sleep_print("thread2"));
    let handle3 = thread::spawn(|| sleep_print("thread3"));

    // sleep_print("main_1");
    // sleep_print("main_2");

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}