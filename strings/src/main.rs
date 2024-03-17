use std::fmt::Debug;
use std::any::type_name_of_val;

fn main() {
    strings();
}


fn  strings () {
    // let mut s = String::new();
    let a = "hello world";
    let s = a.to_string();
    let ss = a.find('l');

    assert_eq!(s.find('l'), Some(2));

    match ss {
        Some(pos)=>println!("The position of 'l' is: {}", pos),
        None => println!("'l' not found"),
    };

    println!("{}", s);


    let str="good".to_string();

    println!("result is {}", str);
}
