trait MyTrait {
    fn f(&self) -> Self;
}


impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function(x: impl MyTrait)  -> impl MyTrait {
    x.f()
}