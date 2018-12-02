fn main() {
    // for n in 1..101 {
    // for n in 1..=100 {    <- can also be written as inclusive
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 5 == 0 {
    //         println!("fizz");
    //     } else if n % 3 == 0 {
    //         println!("buzzzz");
    //     } else {
    //         println!("{:?}", n);
    //     }
    // }

    // iter - borrows each elem of the collection through each iteration
    // and thus does not manipulate the collection at all
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter - consumes the collection so that on each iteration the
    // exact data is provided. Once the collection is consumed, it is no
    // longer able to be reused as it has been 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("Cannot access: {:?}", names);
    // ...
    // 34 |     println!("Cannot access: {:?}", names[0]);
    //    |                                     ^^^^^ value used here after move
    //    |
    //    = note: move occurs because `names` has type `std::vec::Vec<&str>`,
    //            which does not implement the `Copy` trait

    // iter_mut - allows you to borrow each element and modify in place
    // reassign names as mutable
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}