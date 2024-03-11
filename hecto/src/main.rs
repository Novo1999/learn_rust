use std::io::{self, Read}; // imports using use keyword, importing io here because we need to do something with input output, bringing Read into the code lets us use bytes()
fn main() {
    for b in io::stdin().bytes() { // stdin -> standard input stream, bytes() let us iterate over it
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' { // characters require single quote like 'q'
            break;
        }
    }
}