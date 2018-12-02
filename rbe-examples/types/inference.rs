fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    // Should L11 not exist, you would receive a compiler error like
    // 6 |     let mut vec = Vec::new();
    //   |         -------   ^^^^^^^^ cannot infer type for `T`
    //   |         |
    //   |         consider giving `vec` a type

    println!("{:?}", vec);
}
