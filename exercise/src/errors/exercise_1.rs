fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        panic!("Ahhh, this is yucky.");
    }

    println!("Exercise Failed if printing out this line!");
}

pub fn invoke() {
    drink("lemonad");

    println!("Exercise Failed if printing out this line!");
}
