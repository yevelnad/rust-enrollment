pub struct Student{
    name: String,
    age: u32,
}
impl Student{
    pub fn new(name: String, age: u32)->Student{
        Student { name: name, age: age }
    }
    pub fn name(&self)->String{
        let name = self.name.trim();
        name.to_string()
    }
    pub fn age(&self)->u32{
        self.age
    }
   
}

pub struct StudentList{
   pub students: Vec<Student>
}
impl StudentList {
    pub fn new()->StudentList{
        StudentList{students: Vec::new()}
    }
    pub fn add_student(&mut self, student: Student)->&Self{
        self.students.push(student);
        self
    }
}