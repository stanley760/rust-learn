#[cfg(test)]
mod tests {
    use processer::action::process::Process;
    
    #[test]
    fn it_works() {
        let result = Process::run();
        println!("{:#?}",result);
    }
}