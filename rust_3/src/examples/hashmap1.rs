// Rust provides a nice way to set default values for keys
let student3 = String::from("Student3");

// Inserts a key only if it doesn't already exist
let new_entry = grades.entry(student3.clone()).or_insert(80);

// This is the same as writing this:
let new_entry = if grades.contains_key(&student3) {
    grades.get_mut(&student3)
} else {
    grades.insert(student3.clone(), 80);
    grades.get_mut(&student3)
};
