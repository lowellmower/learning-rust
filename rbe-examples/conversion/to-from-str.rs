use std::string::ToString;

struct MyType {
    stringy: i32,
}

struct Circle {
    radius: i32
}

impl ToString for MyType {
    fn to_string(&self) -> String {
        format!("fucking strings {:?}", self.stringy)
    }
}

impl ToString for Circle {
    // interesting to note when referencing self in a trait impl
    // one does not need argument assignment like fn foo(s: &self){}
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let tt = MyType{ stringy: 99 };
    println!("Stringy foo() -> {}", tt.to_string());

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // common parsing "turbofish" syntax
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};    
}
