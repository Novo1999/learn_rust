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
    let mut _name = "Novodip"; // mutable with mut keyword

    _name  = "Sam";

    // re-declarable in current scope
    let _x = "test";
    let x = "test2";

    println!("{}", x);
    
    let is_coder: bool = true;
    let is_singer: bool = false;

    let novo = is_singer;

    if novo == is_coder {
        println!("{} is a coder", _name);
    } else {
        println!("{} is a singer", _name);
    }

    let g = 3.99;
    let h = g as i32;
    println!("g is {}, h is {}", g, h);
}