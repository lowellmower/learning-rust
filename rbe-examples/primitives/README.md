### Primitives

### Concepts:
- Scalar types (uint, sint, f32, f64, char, bool, empty tuples -> () )
- Compund types (arrays -> [1,2,3], tuples -> (1, true))

### Notes:
Not much in the way of foreign concepts in this chapter. One thing worth
noting I suppose would be the `mem` package as part of the standard lib.
Pretty cool that you can analyze the size of the stack allocated array
with `mem::size_of_val(&array)`

Accessing and allocating seems to exercise common patterns like:
```
// allocate an array of 5 unsigned 32 bit integers
let arr: [i32; 5] = [1,2,3,4,5];

// assign a common value
let homo_arr: [i8; 3] = [0; 3];

// len is called upon versus being passed to
arr.len() -> 5
homo_arr.len() -> 3
```
