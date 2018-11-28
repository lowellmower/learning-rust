macro_rules! p_pi {
    // prints the value of pi
    () => (
        println!("{}", 3.14159);
    )
}

fn main() {
    // print and append a new line
    println!("Hello, World!");

    // format a string and interpolate it with an int
    // which will be stingified
    println!("{} days has September; April, June, and November", 30);

    // multiple interpolations in and out of order
    println!("Some months have {0} days, others have {1}. {1} > {0}", 30, 31);

    // named arguments work as well
    println!("{subject} {verb} {object}",
             object="the lazy dog.",
             subject="The quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // print our predefined pi macro
    p_pi!();

    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3)); // uncomment to get error
    /*
    Trying to run rustc with the above will result in a compiler error
    error[E0277]: `main::Structure` doesn't implement `std::fmt::Display`
      --> hello.rs:27:49
       |
    27 |     println!("This struct `{}` won't print...", Structure(3));    
       |                                                 ^^^^^^^^^^^^ `main::Structure` cannot be formatted with the default formatter
       |
       = help: the trait `std::fmt::Display` is not implemented for `main::Structure`
       = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
       = note: required by `std::fmt::Display::fmt`    
    */
}