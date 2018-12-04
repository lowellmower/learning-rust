### Generics

### Concepts
- Generic parameters
- Generic traits
- Generic functions
- Bounds and limiting generics

### Notes
Generally, a generic type in Rust is represented as `<T>` but can be identified
as such with angle brackets and camel casing, `<GenericType>` is an example of
another valid generic. An example of a generic function taking a generic parameter
```rust
// reads as, a generic function foo takes an argument of any type T
fn foo<T>(arg: T) { ... }
```

Implementations can also handle generics but require some syntactic hurdles to
execute, they don't read terribly well but one can see a world where they would
be extraordinarily useful
```rust
// impl of GenVal for a generic type `T`
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}
```

Bounds on generics force the implementation of some specific trait, thus placing
boundaries on what can and cannot respond, making things slightly less 'generic'
```rust
// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any function which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }
```

Multiple bounds can be placed on a function or trait with the `+` operator like
```rust
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

// etc ...
```
