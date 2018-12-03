### Modules

### Concepts
- Code and project organization
- Public and private code

### Notes
By default, the items in a module have private visibility, but this can be
overridden with the `pub` modifier.

The `use` declaration can be leveraged to access and import specific parts of
a crate or a deeply nested function and can alias the names with `as` e.g.
```rust
use deeply::nested::function as other_function;

fn main() {
    other_function();
}
```

The file hierarchy is as such:
```
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```
split.rs
```rust
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
```
my/mod.rs
```rust
// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");
    private_function();
}
```
my/nested.rs
```rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```
my/inaccessible.rs
```rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```