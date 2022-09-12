
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