struct Student {
    name: String,
    grade: u8,
    subjects: Vec<String>,
}
impl Student {
    fn add_subject(&mut self) {
        self.subjects.push(String::from("Hindi"));
    }

    fn display(&self) {
        println!(
            "{} has grade {} and has Subjects {:?}",
            self.name, self.grade, self.subjects
        )
    }
}

fn main() {
    let mut s1 = Student {
        name: String::from("Harish"),
        grade: 97,
        subjects: vec![String::from("Maths"), String::from("Science")],
    };
    s1.add_subject();
    s1.display();
}
