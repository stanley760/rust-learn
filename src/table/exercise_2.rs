use std::collections::HashMap;

pub fn invoke() {
    let teams = [
        ("qing", 12000),
        ("anglo", 11000),
        ("derogatory", 1000),
    ];
    let mut map = HashMap::new();
    for x in &teams {
        map.insert(x.0, x.1);
    }

    let map1 = HashMap::from(teams);
    
    assert_eq!(map, map1);
    
    println!("HashMap exercise_2 is success!");
}