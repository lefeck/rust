use std::any::type_name_of_val;
use std::io;
use rand::Rng;

fn main() {

    guess_game();

    println!("whether a is equal b: {}", assert_fn());

    let sum = add(8,9);
    println!("sum of two number : {}", sum);

    let result  = plus_or_sub(10, 20);
    println!("result is : {}",result);

    printer_number(3, 10);

    numeric_type();

    float_type();

    // float_type2()

}


// printer two number
fn printer_number (x: i32, y: i32) {
    println!("print value of x is {}, value of y is {}",x,y);
}

// guess number
fn guess_game() {
    // println!("input your number");

    //preload dependic to Cargo.toml  rand = "0.8.5"
    let secret_number =  rand::thread_rng().gen_range(0..=100);

    println!("secret number: {}", secret_number);

    let  mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("not read line data");

    println!("your number is : {}", guess );

}



fn assert_fn () -> bool{
    let a = 3;
    let b = 1 + 4;
    if a == b {
        return  true;
    }
    return  false
}

// add plus cacultion
fn add (x: i8, y: i8) -> i8 {
    x + y
}

fn  plus_or_sub (x:i32,y:i32) -> i32 {
    if x > y {
        return  x - y
    }
    return  x + y
}


fn numeric_type () {
    let a : u8 = 255;
    assert_eq!(a.checked_add(20), None);

    let b = a.wrapping_add(20);

    println!("the b value is {}",b);
}

fn float_type () {
    let a:f64 = 3.3;
    let b:f32 = 3.0;

    // 打印 a 和 b 的类型
    println!("The type of a is: {}", type_name_of_val(&a));
    println!("The type of b is: {}", type_name_of_val(&b));


    // assert!(0.1 + 0.2 == 0.3);
}

// fn float_type2 () {
//     let abc:(f32,f32,f32)= (0.1,0.2,0.3);
//     let xyz:(f64,f64,f64)= (0.1,0.2,0.3);
//     println!("abc (f32)");
//     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("         0.3: {:x}", (abc.2).to_bits());
//     println!();
//
//     println!("xyz (f64)");
//     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!("         0.3: {:x}", (xyz.2).to_bits());
//     println!();
//
//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

