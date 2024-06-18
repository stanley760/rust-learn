struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}