use std::io;

use crate::app::subject::Subjects;

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
pub struct Pages{}

impl Pages{
    pub fn login()->UserInput<String>{
        let mut user_input = Vec::new();
        println!("username");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        user_input.push(input);
        println!("username");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        user_input.push(input);
        let inputs = UserInput::new(user_input);
        inputs
        
    }

    pub fn show_subjects(subject_list: &Vec<Subjects>)->UserInput<i32>{
        if subject_list.is_empty(){
            println!("The subjects are empty. Consider adding one by choosing 1");
        }else{
            for subjects in subject_list{
                println!("{}", subjects.name())
            }
        }
        let mut user_input = Vec::new();
        user_input.push(3);

        let input:UserInput<i32> = UserInput::new(user_input);
        input
    }

    pub fn add_subjects()->Subjects{
        println!("name");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let mut teacher = String::new();
        io::stdin().read_line(&mut teacher).expect("Failed to read line");
        let mut time = String::new();
        io::stdin().read_line(&mut time).expect("Failed to read line");
        let subject = Subjects::new(name,teacher);
        subject
    }
}