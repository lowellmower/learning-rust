use std::fmt;

// a struct to impl the display func
#[derive(Debug)] // can have debug and custom printer
struct SomeStruct(i32);

// implement your own printer for a struct
impl fmt::Display for SomeStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed.
        
        // write!(f, "val is {0}, would be default {1}", self.0, 99);
        // the above could error and should be written using a ?
        // so as to return an error if so
        // this could also be written with try! which is deprecated
        // try!(write!(f, "val is {0}, would be default {1}", self.0, 99));
        write!(f, "val is {0}, would be default {1}", self.0, 99)
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let ss = SomeStruct(32);
    println!("Display: {}", ss);
    println!("Debug: {:?}", ss);

    let name = "Peter";
    let age = 37;
    let peter = Person { name, age};

    println!("{:#?}", peter);
    /*
    {:#?} will print like:
    hello-world :> ./pprint 
    Person {
        name: "Peter",
        age: 37
    }
    */
}