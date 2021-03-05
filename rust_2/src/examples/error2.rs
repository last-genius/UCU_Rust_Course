fn result_test() -> Result<&'static str, &'static str> {
    if something {
        Ok("valuable data we can work with")
    } else {
        Err("error commentary")
    }
}
