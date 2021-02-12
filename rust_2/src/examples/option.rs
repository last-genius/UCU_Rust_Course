fn divide(num: f64, den: f64) -> Option<f64> {
    if denominator == 0.0 { None } 
    else { Some(numerator / denominator) }
}

// The return value of the function is an Option
// Pattern match to retrieve the value
match divide(2.0, 3.0) {
    // The division was valid
    Some(x) => println!("Result: {}", x),

    // The division was invalid
    None    => println!("Cannot divide by 0"),
}
