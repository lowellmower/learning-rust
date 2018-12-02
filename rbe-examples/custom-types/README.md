### Custom Types

### Concepts
- Defining and characterizing a structure (`struct`)
- Defining an enumeration (`enum`)
- Constants / statics (`const`, `static`)

### Notes
Three notable types of structs, named tuples (meh), unit structs which are
useful for generics, and the classic C style struct.

A useful macro was introduced to keep warnings quite from the compiler with
respect to unused code `#![allow(dead_code)]` or apply it as the debug macro
is on a single object.

Enums are useful C-like structs which are flexible for implementing data
which may have default values which incrementally increase. They are flexible
enough to be given methods which they can respond to without implementing an
entirely new object struct. The `use` and `match` pattern (see linked list)
is a powerful characteristic of the `enum`.
