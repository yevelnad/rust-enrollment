
pub struct UserInput<T>{
    inputs: Vec<T>,
}
impl <T> UserInput<T> {
    pub fn new(inputs: Vec<T>)->UserInput<T>{
        UserInput{inputs}
    }
    pub fn inputs(self)->Vec<T>{
      self.inputs
    }
    pub fn is_empty(&self)->bool{
      if self.inputs.is_empty(){
        true
      }
      else{
        false
      }
    }
    
}