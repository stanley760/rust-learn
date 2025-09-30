pub struct Solution;

impl Solution {
    pub fn valid_ip_address(address: String) -> String {
        // address.replace(".", "[.]")
        address.as_bytes().into_iter().map(|c| match c {
            b'.' => "[.]".to_string(),
            _ => (*c as char).to_string(),
        }).collect::<String>()
    }
}

#[test]
fn test_valid_ip_address() {
    assert_eq!(Solution::valid_ip_address("1.1.1.1".to_string()), "1[.]1[.]1[.]1".to_string());
    assert_eq!(Solution::valid_ip_address("255.100.50.0".to_string()), "255[.]100[.]50[.]0".to_string());
}