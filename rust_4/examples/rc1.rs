let text = "Rc examples".to_string();
{
    // Reference count is 1
    let rc_a: Rc<String> = Rc::new(text);
    {
        // Reference count is 2
        let rc_b: Rc<String> = Rc::clone(&rc_a);
        
        rc_a.len();
        println!("{}", rc_b);
        
        // rc_b is dropped, reference count is 1
    }
    // rc_a is dropped, reference count is 0, the value is dropped too
}
// Error, rc_examples was dropped!
println!("rc_examples: {}", rc_examples);
