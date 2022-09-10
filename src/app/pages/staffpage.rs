use crate::app::student::Student;
use std::io;
pub struct StaffPages{}

impl StaffPages{
    pub fn show_students(student_list: &Vec<Student>)->i32{
        if student_list.is_empty(){
            println!("Seems you have empty list of students.");
            println!("1. Consider adding one by entering number 1.");
        }
        else{
            println!("These are the current list of students.");
            println!(" ");
            let mut x = 0;
            for student in student_list{
                x = x + 1;
                println!("{}. {}", x, &student.name())
            }
            println!("1. Enter 1 to add a student.")
        }
        println!("2. To logout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Cant read line");
        let user_input = input.trim().parse().unwrap_or(0);
        user_input
    }
    pub fn add_student()->Student{
        let mut input = String::new();
        println!("Please add a name.");
        io::stdin().read_line(&mut input ).expect("Cant read the line");
        let student_name = input;
        println!("Please add an age.");
        let mut input_age = String::new();
        io::stdin().read_line(&mut input_age).expect("cant read line");
        let age = input_age.trim().parse().unwrap_or(1);
        let student = Student::new(student_name,age);
        student
    }
}