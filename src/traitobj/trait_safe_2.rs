trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42)}
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}


pub fn invoke() {
    let x = 42u32;
    let y = "hello".to_string();
    let _z = my_function(Box::new(x));
    let _w = my_function(Box::new(y));
    
}