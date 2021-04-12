use std::thread;
use std::time::Duration;

fn main() {
    let mut vec = Vec::new();

    for i in 1..6 {
        vec.push(thread::spawn(move || {
            for j in 1..10 {
                println!("thread {} - {}", i, j);
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    for i in 1..5 {
        println!("main thread - {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    for thread in vec {
        thread.join().unwrap();
    }
}
