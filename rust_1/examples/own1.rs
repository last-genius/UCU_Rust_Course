fn main() {
    let a: u32 = 5;
    let b = Box::new(5i32);

    {
        let a: u32 = 5;
        let b = Box::new(5i32);
    }
}
