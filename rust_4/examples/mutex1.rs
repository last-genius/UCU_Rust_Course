use std::sync::Mutex;

fn main() {
    let a = Mutex::new(42);

    {
        let mut data = a.lock().unwrap();
        *data += 1;
    }

    println!("{:?}", a);
}
