// const: An unchangeable value (the common case).
// static: A possibly mutable variable with 'static lifetime. 
//         The static lifetime is inferred and does not have 
//         to be specified. Accessing or modifying a mutable
//         static variable is unsafe.

static LANG: &str = "Rust";
const TOP: i32 = 10;

fn over_top(n: i32) -> bool {
    // Access constant from global scope
    n > TOP
}

fn main() {
    let n = 13;
    println!("{} is {}", n, if over_top(n) { "over the top." } else { "not over top." });
}