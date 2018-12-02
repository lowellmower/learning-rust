### Conversion

### Concepts
- Convert between types using traits
- Generic conversion usinf `From` and `Into`

### Notes
Conversions are merely just trait implementations. The only bit which seems
worth noting here is that when implementing a trait for a Type, one does not
need to assign `&self` to an argument variable/identifier, e.g.
```
struct MyType {
    field i32,
}

impl Foo for MyType {
    fn foo(&self) -> ReturnType {
        // can ref self.field
    }
}
```
You can use `impl` to modify the traits to your liking but common conversion
follows turbofish syntax like below
```
fn main() {
    // common parsing "turbofish" syntax
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
```