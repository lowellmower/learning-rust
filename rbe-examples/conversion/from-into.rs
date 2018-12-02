use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    
    let num1 = Number::from(30);
    println!("My number is {:?}", num1);
    // must declare type and cannot infer with assignment like
    // let num = int.into();
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
