use num::Complex;

fn type_num() {
    println!("===================complex==================>");
    numb_complex();
    println!("=====================range==================>");
    fun_range();
    println!("=================bit_operation==============>");
    operation_bit();
    println!("=================numb_diff=================>");
    defined_numb_diff();
    println!("==================numb_nan=================>");
    numb_nan();
    println!("==================float_diff===============>");
    float_diff_eq();
    println!("================var_shadowing==============>");
    _var_shadowing();
    println!("========================================>");
    _define_x();
    _mut_var();
    _struct_dec();
    let x = "hello";
    println!("{}, world", x);
}

fn numb_complex() {
    let b = Complex {re: 1.0, im: -2.0};
    let a = Complex::new(11.1, 22.2);
    let res = a + b;
    println!("({}) + ({}) = ({})", a, b, res);
}

fn fun_range() {
    for i in 'a'..='z' {
        print!("{} ",i);
    }
    println!();
}

fn operation_bit() {
    let a = 2; // 010
    let b = 3; // 011

    println!("a & b : {}", a & b);
    println!("a | b : {}", a | b);
    println!("a ^ b : {}", a ^ b); // 001
    println!("!a : {}", !a); // 101
    println!("!b : {}", !b); // 100
    println!("a << b : {}", a << b);
    println!("a >> b : {}", a >> b);
    let mut a = a; 
    a <<= b;
    println!("a <<= b : {}", a);
}

fn defined_numb_diff() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;

    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:.2}", forty_twos[0]);
}

fn numb_nan() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

fn float_diff_eq() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    //assert!(xyz.0 + xyz.1 == xyz.2); err
}

fn _var_shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the current x:{}", x);
    }
    println!("the new current x:{}", x)
}

fn _struct_dec() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
}

fn _define_x() {
    let _x = "hello";
}

fn _mut_var() {
    let mut x = 5;
    println!("the x value:{}", x);
    x = 6;
    println!("the current x:{}", x);
}
