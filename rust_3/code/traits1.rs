trait Report {
    fn general_report(&self) -> String;
    fn grades(&self) -> &HashMap<String, i32> {
        &self.grades
    };
}

struct SchoolStudent {
    name: String,
    grades: HashMap<String, i32>,
    teacher: String,
    classes: Vec<String>,
}
