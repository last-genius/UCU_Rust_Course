use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new(AtomicUsize::new(1));

    let clone = Arc::clone(&a);
    let thread = thread::spawn(move || {
        clone.fetch_add(1, Ordering::Relaxed);
    });

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
