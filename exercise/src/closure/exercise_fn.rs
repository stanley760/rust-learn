#[allow(unused)]
fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}
#[allow(unused)]
fn call_mut(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}
#[allow(unused)]
fn call_once(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_exercise() {
        let v = vec![0u8; 1024];
        let v1 = vec![0u8; 1023];

        // Fn，不移动所有权
        let mut c = |x: u64| v.len() as u64 * x;
        // Fn，移动所有权
        let mut c1 = move |x: u64| v1.len() as u64 * x;

        println!("direct call: {}", c(2));
        println!("direct call: {}", c1(2));

        println!("call: {}", call(3, &c));
        println!("call: {}", call(3, &c1));

        println!("call_mut: {}", call_mut(4, &mut c));
        println!("call_mut: {}", call_mut(4, &mut c1));

        println!("call_once: {}", call_once(5, c));
        println!("call_once: {}", call_once(5, c1));
    }

    #[test]
    pub fn test_exercise2() {
        let name = String::from("Tyr");
        let vec = ["Rust", "Elixir", "Javascript"];
        let v = &vec[..];
        let data = (1, 2, 3, 4);
        let c = move || {
            println!("data: {:?}", data);
            println!("v: {:?}, name: {:?}", v, name.clone());
        };
        c();
        //println!("name:{}", name);
    }
}
