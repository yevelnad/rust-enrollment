use std::io;

use crate::app::userinput::UserInput;





pub struct MainPage{}

impl MainPage{
    pub fn mainpage()->UserInput<i32>{
        println!("Please choose an option:");
        println!("1. Login as an admin to add subjects.");
        println!("2. Login as a staff to enroll students");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let i32_input: i32 = input.trim().parse().unwrap_or(0);
        let user_input = UserInput::new(vec![i32_input]);
        user_input
    }
    pub fn login()->UserInput<String>{
        let mut username = String::new();
        println!("++++++++++++++++++++++++++++++++");
        println!("+  Please enter your username: +");
        println!("++++++++++++++++++++++++++++++++");
        io::stdin().read_line(&mut username).expect("Failed to read line");
        let mut password = String::new();
        println!("++++++++++++++++++++++++++++++++");
        println!("+ Please enter your password:  +");
        println!("++++++++++++++++++++++++++++++++");
        io::stdin().read_line(&mut password).expect("Failed to read line");
        let user_input = UserInput::new(vec![username,password]);
        user_input
    }
}