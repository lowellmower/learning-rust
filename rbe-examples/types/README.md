### Types

### Concepts
- Casting
- Literals
- Inferences
- Aliasing

### Notes
There is no implicit type conversion (coercion) between the primitives, however
one can explicitly convert a type (cast) using the reserved word `as`. Casting
is very similar to C convention

Literals can be suffixed or unsuffixed and will be initialized accordingly. May
be worth noting that if a number is initialized unsuffixed, it will be an `i32`
and floating point numbers captured as `f64` as default values.

The compiler can infer types pretty well, as an example, one could initialize a
new `Vec` but leave it empty until a few lines later where a value is then put
in, the types would look like:
```
// On init - (`Vec<_>`)
let mut vec = Vec::new();

// Later on - (`Vec<i32>`)
vec.push(75);
```

Aliases are used largely to reduce boilerplate; for example the `IoResult<T>`
type is an alias for the `Result<T, IoError>` type.
