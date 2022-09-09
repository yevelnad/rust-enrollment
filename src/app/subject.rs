pub struct Subject{
    name: String,
    teacher: String,
}
impl Subject{
    pub fn new(name: String, teacher: String)->Subject{
        Subject { name: name, teacher: teacher }
    }
    pub fn name(&self)->String{
        let name = self.teacher.trim();
        name.to_string()
    }
    pub fn teacher(&self)->String{
        let name = self.teacher.trim();
        name.to_string()
    }
   
}

pub struct SubjectList{
    subjects: Vec<Subject>
}
impl SubjectList {
    pub fn new()->SubjectList{
        SubjectList{subjects: Vec::new()}
    }
    pub fn subjects(&self)->Vec<Subject>{
        self.subjects
    }
}