// Iterating
for book in &books {
    println!("{}", book);
}

// Creation from an iterator
let students = ["Student1", "Student2", "Student3"]
        .iter()
        .cloned()
        .collect::<HashSet<&'static str>>();
