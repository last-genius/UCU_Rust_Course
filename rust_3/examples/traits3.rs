fn get_report(student: &impl Report) {
    println!("{}", student.general_report());
}

// This is equivalent to this, now with generics:
fn get_report<T: Report>(student: &T) {
    println!("{}", student.general_report());
}
