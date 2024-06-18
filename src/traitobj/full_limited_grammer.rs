trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

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
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

pub fn invoke() {
    let person = Human;

    assert_eq!(Pilot::fly(&person), String::from("This is your captain speaking."));
    assert_eq!(Wizard::fly(&person), String::from("Up!"));
    assert_eq!(person.fly(), String::from("*waving arms furiously*"));

    println!("Success!")
}