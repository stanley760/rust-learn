#[cfg(test)]
mod tests {
    use process_killer::action::process::Process;

    #[test]
    fn it_works() {
        let result = Process::run();
        println!("{:#?}", result);
    }
}
