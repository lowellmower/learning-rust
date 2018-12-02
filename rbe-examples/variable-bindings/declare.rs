// This declare first pattern is largely not used as it can result in variables
// which go uninitialized. Though at first glance the pattern seems useful, in
// that we can imagine a world where we're not sure of a type or piece of data
// this is something which Go handles well by using interfaces and not having
// the support of generics, that is a strong pattern.

fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
