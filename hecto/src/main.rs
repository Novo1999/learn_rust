// use std::io::{self, Read}; // imports using use keyword, importing io here because we need to do something with input output, bringing Read into the code lets us use bytes()

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

    _name = "Sam";

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
    println!("{}", num);

    // flow control

    let age = 34;

    let message = if age > 33 {
        "age is more than 33"
    } else {
        "age is less than 33"
    };

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
        _ => println!("Who cares"),
    }

    // let mut i = 0;
    // while i < 10 {
    //     println!("{}", i);
    //     i += 1;
    // }

    let arr = [12, 34, 55, 66, 77];
    for elem in arr {
        if elem == 66 {
            println!("At 66");
            break;
        }
        if elem < 50 {
            println!("less than 50 {}", elem)
        }
        println!("{}", elem)
    }

    // enums
    let c: TrafficLight = TrafficLight::Yellow;

    // match c {
    //     TrafficLight::Red => println!("RED"),
    //     TrafficLight::Green => println!("GREEN"),
    //     TrafficLight::Yellow => println!("YELLOW"),
    // }

    let favnum: Option<i32> = Option::Some(105);

    match favnum {
        Some(n) => println!("My favorite number is {}, good choice", n),
        None => println!("Error"),
    }

    let name = "12";

    let numbah = name.parse::<i32>();

    println!("A number: {:?}", numbah);

    let _good_str = String::from("1964");

    let mut v: Vec<i32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

    let mut i: usize = v.len() - 1;
    while i > 0 {
        println!("{}", v[i]);
        i -= 1;
    }

    // tuple
    let tuple1 = (456, String::from("Bangladesh"), 3.14);

    println!("{:?}", tuple1.1);

    // vector

    let mut vec1: Vec<i32> = Vec::<i32>::new();

    vec1.push(100);

    let mut vec2 = vec!["Dhaka"];

    vec2.insert(1, "element");

    println!("{:?}", vec2);

    println!("{:?}", vec1);

    let string1 = String::from("hello");
    let string2 = &string1;

    println!("string1 {}", string1);
    println!("string2 {}", string2);

    // let r;

    // {
    //     let x = 5;
    // dangling reference
    //     r = &x;
    // }

    // println!("r: {}", r);
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// fn print_vector(x: Vec<i32>) { // vector passed as parameter here
//     println!("inside function {:?}", x)
// }
