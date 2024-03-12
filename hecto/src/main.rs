// // use std::io::{self, Read}; // imports using use keyword, importing io here because we need to do something with input output, bringing Read into the code lets us use bytes()

// use std::io::{self, stdout, Read};


// fn main() {
//     for b in io::stdin().bytes() { // stdin -> standard input stream, bytes() let us iterate over it
//         let c = b.unwrap() as char;
//         println!("{}", c);
//         if c == 'q' { // characters require single quote like 'q'
//             break;
//         }
//     }
// }


fn main() {

    let _f3: f32 = 1.2534;
    let f1: f32 = 1.234;
    let f2: f32 = 1.456;

    println!("first float - {}, second float - {} ", f1, f2);

    // by default immutable
    let mut _name: &str = "Novodip"; // mutable with mut keyword

    _name  = "Sam";

    // re-declarable in current scope
    let _x: &str = "test";
    let x: &str = "test2";

    println!("{}", x);
    
    let is_coder: bool = true;
    let is_singer: bool = false;

    let novo = is_singer;

    if novo == is_coder {
        println!("{} is a coder", _name);
    } else {
        println!("{} is a singer", _name);
    }

    let g = 147;
    let h = g as i32;
    println!("g is {}, h is {}", g, h);

    let v: Vec<i32> = vec![10, 20, 30]; // every value has a single owner

    let v: &Vec<i32> = &v;
    // print_vector(&v); // ownership of the vector passed

    println!("{}", v[0]);

    let num = String::from("12345");
    println!("{}" , num);
    
    // flow control

    let age = 34;

    let message = if age > 33 {"age is more than 33"} else {"age is less than 33"};

    println!("{}", message);

    // match
    let num = 100;

    // match num {
    //     100 => println!("hundred"),
    //     200 => println!("Two hundred"),
    //     _ => println!("Who cares") // default case
    // }

    match num {
        25..=50 => println!("25 to 50"),
        51..=100 => println!("50 to 100"),
        _ => println!("Who cares")
    }

}

// fn print_vector(x: Vec<i32>) { // vector passed as parameter here
//     println!("inside function {:?}", x)
// }