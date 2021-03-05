// Panic if the Err() occurs:
let ok_message = result_test().unwrap();

// Panic if the Err() occurs, but add a message:
let ok_message = result_test().expect("message text");
