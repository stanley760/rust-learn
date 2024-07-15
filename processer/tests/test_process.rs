#[cfg(test)]
mod tests {
    use processer::action::process::Process;
    
    #[test]
    fn it_works() {
        let result = Process::run();
        assert_eq!(result.len(), 0);
    }
}