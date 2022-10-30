// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
use std::collections::HashMap;
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students_grade: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            students_grade: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students_grade
            .entry(grade)
            .and_modify(|students| students.push(student.to_owned()))
            .or_insert(vec![student.to_owned()]);
        // match self.students_grade.get(&grade) {
        //     Some(grades) => {
        //         let mut g = grades.clone();
        //         g.push(student.to_owned());
        //         g.sort();
        //         self.students_grade.insert(grade, g).unwrap();
        //     }
        //     None => {
        //         self.students_grade
        //             .insert(grade, vec![student.to_owned()])
        //             .is_none();
        //     }
        // };
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v = Vec::new();
        self.students_grade.keys().for_each(|g| v.push(*g));
        v.sort();
        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self.students_grade.get(&grade).unwrap_or(&vec![]).clone();
        students.sort();
        students
    }
}
