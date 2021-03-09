fn get_report(student: &(impl Report + Display)) {
    println!("{}", student.general_report());
}

// This is equivalent to this, now with generics:
fn get_report<T: Report + Display>(student: &T) {
    println!("{}", student.general_report());
}
