match result_test() {
    Ok(message) => {
        println!("We received a message: {}", message);
    }
    Err(err_message) => {
        println!("There was an error: {}", err_message);
    }
}
