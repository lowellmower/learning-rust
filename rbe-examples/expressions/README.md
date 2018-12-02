### Expressions

### Concepts
- Block and assignments as expressions and statements

### Notes
Though there is not a lot to cover in this section it was worth making a note
for it as the suppression of expressions using a `;` can be tricky to read or
cause for some future headache. Example can be seen in `./expressions.rs` but
for ease, also below:
```rust
let x = {
    2 + 2;
};
// x is an empty expression () as it was suppressed by the semicolon

let x = {
    2 + 2
};
// x is 4i32 as the expression was evaluated and 'returned' or assigned
```
