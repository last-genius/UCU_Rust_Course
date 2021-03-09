impl Report for SchoolStudent {
    fn general_report(&self) -> String {
        format!(
            "Student: {},
        Their teacher: {},
        Takes these classes: {:?},
        Their grades: {:?}",
            &self.name, &self.teacher, &self.classes, &self.grades
        )
    }
}
