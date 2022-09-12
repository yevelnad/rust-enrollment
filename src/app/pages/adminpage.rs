use std::io;

use crate::app::{subject::Subject, userinput::UserInput};




pub struct AdminPages{}

impl AdminPages{
    pub fn show_subjects(subject_list: &Vec<Subject>)->UserInput<i32>{
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
        let i32_input:i32 = input.trim().parse().unwrap_or(99);
        let user_input = UserInput::new(vec![i32_input]);
        user_input
    }

    pub fn add_subjects()->UserInput<String>{
        println!("name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        println!("teacher:");
        let mut teacher = String::new();
        io::stdin().read_line(&mut teacher).expect("Failed to read line");
        let userinput = UserInput::new(vec![name,teacher]);
        userinput
    }
}