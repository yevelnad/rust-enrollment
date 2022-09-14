pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
 use enrollment::app::user::{UserList, User};
    #[test]
    fn user_test(){
        let user = User::new_admin(
            "username".to_string(),
            "password".to_string()
        );
        let user2 = User::new_admin(
            "username".to_string(),
            "password".to_string()
        );
        assert_eq!("username", user.name());
        assert_eq!("password", user.password());
        let mut user_list = UserList::new();
        user_list.add_user(user);
        assert_eq!(true, user_list.login(user2));

        
    }
}
