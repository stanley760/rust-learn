trait Person {
    fn name(&self) -> String;
}

// Person 是 Student 的 supertrait .
// 实现 Student 需要同时实现 Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) 是 Programmer
// 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "/*********************************\n\
        My name is {} and I attend {}. \n\
        My favorite language is {}. \n\
        My Git username is {}\n\
        /*********************************",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

// 为 CSStudent 实现所需的特征
impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}


impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.to_string()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.to_string()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.to_string()
    }
}

pub fn invoke() {
    let student = CSStudent {
        name: "LeeHua".to_string(),
        university: "UST".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    // 填空
    println!("{}", comp_sci_student_greeting(&student));
}