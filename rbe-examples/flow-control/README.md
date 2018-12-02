### Flow Control

### Concepts
- General logic and data flow

### Notes
Worth remembering that `if/else` is an expression (as indicated with `;`) and
is thus suppressed from any assignment (obviously). If logic is used to pass
assignment to something, both/all branches must return the same type, e.g.
```rust
let x = 
    if <condition> {
        i32
    } else {
        // must also return i32
        i32
    };
```

Explicit reserved word for an infinite loop, e.g. `loop`, which has the same
characteristics you'd expect with `break` and `continue`. Cool that you can
easily influence the flow of an outer loop from a nested loop but requires the
application of a `'label` like:
```rust
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            // some logic which may lead to the desire to break
            // or continue the outer loop
            break 'outer;
        }
        // because this code is never reached a global attribute
        // should/could be applied to silence compiler warnings
        // e.g. #![allow(unreachable_code)]
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");    

```

For loops can be inclusive by using an assignment operator like `n in 1..=100`
or for traditional behavior like `n in 1..101` where in both cases `n` will be
of values `1, 2, ...100`.

Very important to note the three types of iterators which are default to the
standard lib, `iter`, `into_iter`, `iter_mut`. If not explicit selection is made
the default is `into_iter` which can make for some newbie headaches with respect
to ownership/lifetime.
```
// iter - borrows each elem of the collection through each iteration
// and thus does not manipulate the collection at all

// into_iter - consumes the collection so that on each iteration the
// exact data is provided. Once the collection is consumed, it is no
// longer able to be reused as it has been 'moved' within the loop.

// iter_mut - allows you to borrow each element and modify in place
// reassign names as mutable
```

It feels as if `match`, which is similar to C like `switch` has been given a
prime place in the language. It can do a number of convenient operations which
allow the comparison statements to be compact and concise.


