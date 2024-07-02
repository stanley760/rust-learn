use crate::traitobj::full_limited_grammer::Pilot;
use crate::traitobj::full_limited_grammer::Wizard;

struct Human;
impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn invoke() {
    let person = Human;
    println!("Pilot: {}", Pilot::fly(&person));
    println!("Wizard:{}", Wizard::fly(&person));
    person.fly();
}
