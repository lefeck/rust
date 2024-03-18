

fn main() {
    for_fn();
    for_item_fn();

    while_fn ();

    break_fn();
    continue_fn();
    loop_fn();
}

fn  for_fn () {
    for i in 1..=5 {
        println!("value is {}", i);
    }

    let n = 5;
    for i in 0..n {
        println!("the current value is {}",i) //0~n-1
    }

    for i in 0..=n {
        println!("the current value is {}",i) //0~n
    }

    let a: Vec<usize> = vec![0,1,2,3,4];
    for i in a.iter() {
        println!("the current value is {}",i) //0~n-1
    }

    for i in a[0..n].iter() {
        println!("the current value is {}",i) //0~n-1
    }


    for i in a[0..=n-1].iter() {
        println!("the current value is {}",i) //0~n-1
    }

    for i in 1..n {
        println!("the current value is {}",a[i]) //0~n-1
    }

}


fn for_item_fn () {
    let a = ["jack","luce", "trump", "john"];
    for (i,v) in a.iter().enumerate(){
        println!("第{}个元素是 {}", i + 1, v);
    }



    println!("total length is  {}", a.len());

    // for i in 0..a.len() {
    //     let item = a[i];
    //
    // }

    for value in a.iter() {
        println!("the value is {}", value)
    }
}

//用 while 来实现 for 的功能
fn while_fn () {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5  {
        println!("the value is: {}", a[index]);
        index = index+1;
    }
}


fn break_fn() {
    for i in 1..4 {
        if i == 2 {
            break
        }
        println!("the current value is {}",i);
    }
}

fn continue_fn() {
    for i in 1..4 {
        if i == 2 {
            continue
        }
        println!("the current value is {}",i);
    }
}

fn loop_fn () {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" the alphabet 'O' index is : {}", location)
}
