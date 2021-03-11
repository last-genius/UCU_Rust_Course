fn test_func<A, B, F>(self, f: F) -> Vec<B>
    where F: FnMut(A) -> B;
{}

fn box_up_your_closure_and_move_out() -> Box<Fn(i32) -> i32> {
    // local stuff
    Box::new(move |x| x * local)
}
