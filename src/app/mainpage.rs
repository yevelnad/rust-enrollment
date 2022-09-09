use std::io;

use super::adminpage::UserInput;

pub struct MainPage{}

impl MainPage{
    pub fn mainpage()->i32{
        println!("Please choose an option:");
        println!("1. Login as an admin to add subjects.");
        println!("2. Login as a staff to enroll students");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let i32_input: i32 = input.trim().parse().unwrap_or_default();
        i32_input
    }
    pub fn login()->UserInput<String>{
        let mut user_input = Vec::new();
        println!("++++++++++++++++++++++++++++++++");
        println!("+  Please enter your username: +");
        println!("++++++++++++++++++++++++++++++++");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        user_input.push(input);
        println!("++++++++++++++++++++++++++++++++");
        println!("+ Please enter your password:  +");
        println!("++++++++++++++++++++++++++++++++");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        user_input.push(input);
        let inputs = UserInput::new(user_input);
        inputs
        
    }
}