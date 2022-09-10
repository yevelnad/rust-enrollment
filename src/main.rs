
use enrollment::app::pages::staffpage::StaffPages;
use enrollment::app::student::{StudentList};
use enrollment::app::subject::SubjectList;
use enrollment::app::pages::{adminpage::AdminPages, mainpage::MainPage};





extern crate enrollment;

fn main() {
    let mut subject_list = SubjectList::new();
    let mut student_list = StudentList::new();
    let mut session = "main";
    loop{
        match session{

            "admin login" => {

                let user_input = MainPage::login().inputs();
                if login_admin(user_input){
                    session = "show subjects";
                }
                else{
                    println!("Wrong username or password");
                }

            },

            "show subjects" => {
                let user_input = AdminPages::show_subjects(&subject_list.subjects);
                
                match user_input {
                    1 => {
                        println!("PLEASE ADD ONE SUBJECTS");
                        session = "add subject";
                    },
                    2 => {
                        println!("YOU HAVE LOGOUT!!!");
                        println!("BACK TO LOGIN PAGE");
                        session = "main";
                    },
                    _ =>{
                        println!("wrong choice me amigos!");
                    }
                }
            },

            "add subject" => {
                    let subject = AdminPages::add_subjects();
                    subject_list.add_subject(subject);
                    session = "show subjects";
            },
            "staff login" => {
                let user_input = MainPage::login().inputs();
                if login_staff(user_input){
                    session = "show students";
                }
                else{
                    println!("Wrong username or password");
                }
            },
            "show students" => {
                let user_input = StaffPages::show_students(&student_list.students);
                match user_input {
                    1 =>{
                        println!("You have selected to add students");
                        session = "add students";
                    }
                    2 => {
                        println!("You have logout!!!!");
                        session = "main";
                    }
                    _ => {
                        println!("Wrong choice buddy!!");
                    }
                }
            },
            "add students" => {
                let student = StaffPages::add_student();
                let age = student.age();
                match age {
                    1..=13 => {
                        println!("This student seems very young. you can only enroll students with age 14-30.")
                    },
                    14..=30 =>{
                        println!("This student is accepted ");
                        student_list.add_student(student);
                        session = "show students";
                    },
                    31..=99 => {
                        println!("We don't accept student with age above 30");
                    },
                    200..=9999 =>{
                        println!("You seems very old to be a human.");
                        println!("We only accepts human");
                        println!("But please teach me how to become an immortal");
                    }
                    _ => {
                        println!("Something is not right. Only students with age 14 - 30 are accepted");
                    }

                }
            },
            "main" => {
                let user_input = MainPage::mainpage();
                match user_input{
                    1 => {
                        session = "admin login";
                        println!("You have selected to login as admin.");

                    },
                    2 => {
                        session = "staff login";
                        println!("You have selected to login as staff.")
                    },
                    _ => {
                        println!("Wrong choice me amigos");
                    }
                }
            }
            _ => {
                println!("Page not found");
                break;
            }
        }

    }
}

fn login_admin(inputs: Vec<String>)->bool{
    let username = inputs[0].trim();
    let password = inputs[1].trim();
    if username == "admin" && password == "admin"{
        println!("success");
        true
    }
    else{
        println!("user: {} and pass: {} doesn't match.",username, password);
        println!("failed");
        false
    }

}

fn login_staff(inputs: Vec<String>)->bool{
    let username = inputs[0].trim();
    let password = inputs[1].trim();
    if username == "staff" && password == "staff"{
        println!("success");
        true
    }
    else{
        println!("user: {} and pass: {} doesn't match.",username, password);
        println!("failed");
        false
    }

}