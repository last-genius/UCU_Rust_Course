fn result_test() -> Result<&'static str, &'static str> {
    if something {
        Ok("valuable data we can work with")
    } else {
        Err("error commentary")
    }
}

fn main() {
    match result_test() {
        Ok(message) => println!("We received a message: {}", message),
        Err(err_message) => println!("There was an error: {}", err_message),
    }
}
