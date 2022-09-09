use std::io;


use super::subject::Subject;

pub struct UserInput<T>{
    pub inputs: Vec<T>,
}
impl <T> UserInput<T> {
    pub fn new(inputs: Vec<T>)->UserInput<T>{
        UserInput{inputs:inputs}
    }
    pub fn inputs(self)->Vec<T>{
       self.inputs
    }
}
pub struct AdminPages{}

impl AdminPages{
    pub fn show_subjects(subject_list: &Vec<Subject>)->i32{
        if subject_list.is_empty(){
            println!("+++++++++++++++++++++++++++++++");
            println!("The subjects are empty.");
            println!("1. Enter 1 to add subjects")
        }else{
            println!("+++++++++++++++++++++++++++++++");
            println!("The current subjects are:");
            for subjects in subject_list{
                println!("{}", subjects.name())
            }
            println!("+++++++++++++++++++++++++++++++");
            println!("1. Enter 1 to add subjects");
        }
        println!("2. Enter 2 to logout!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let i32_input:i32 = input.trim().parse().unwrap_or_default();
        i32_input
       
    }

    pub fn add_subjects()->Subject{
        println!("name");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        println!("teacher");
        let mut teacher = String::new();
        io::stdin().read_line(&mut teacher).expect("Failed to read line");
        let subject = Subject::new(name,teacher);
        subject
    }
}