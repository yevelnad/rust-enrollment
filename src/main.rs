use enrollment::app::{subject::SubjectList, page::Pages};



extern crate enrollment;

fn main() {
    let mut subject_list = SubjectList::new();
    let mut session = "login";
    loop{
        match session{
            "login" => {

                let user_input = Pages::login().inputs();
                if login(user_input){
                    session = "show subjects";
                }
                else{
                    println!("Wrong username or password");
                }

            },
            "show subjects" => {
                let user_input = Pages::show_subjects(&subject_list.subjects);
                
                match user_input {
                    1 => {
                        println!("PLEASE ADD ONE SUBJECTS");
                        session = "add subject";
                    },
                    2 => {
                        println!("YOU HAVE LOGOUT!!!");
                        println!("BACK TO LOGIN PAGE");
                        session = "login";
                    },
                    _ =>{
                        println!("wrong choice me amigos!");
                    }
                }
            },
            "add subject" => {
                    let subject = Pages::add_subjects();
                    subject_list.add_subject(subject);
                    session = "show subjects";
            },
            _ => {
                println!("Page not found");
                break;
            }
        }

    }
}

fn login(inputs: Vec<String>)->bool{
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