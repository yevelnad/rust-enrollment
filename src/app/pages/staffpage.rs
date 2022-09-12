use crate::app::{student::Student, userinput::UserInput};
use std::io;
pub struct StaffPages{}

impl StaffPages{
    pub fn show_students(student_list: &Vec<Student>)->UserInput<i32>{
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
        let i32_input = input.trim().parse().unwrap_or(0);
        let user_input = UserInput::new(vec![i32_input]);
        user_input
    }
    pub fn add_student()->UserInput<String>{
        let mut name = String::new();
        println!("Please add a name.");
        io::stdin().read_line(&mut name ).expect("Cant read the line");
        let mut age = String::new();
        println!("Please add an age.");
        io::stdin().read_line(&mut age).expect("cant read line");
        name = name.trim().to_string();
        let user_input = UserInput::new(vec![name,age]);
        user_input
        
    }
}