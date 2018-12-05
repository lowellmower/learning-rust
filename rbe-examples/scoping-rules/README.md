### Scoping Rules

### Concepts
- GC and moves
- Ownership
- Borrowing
- Lifetimes

### Notes

#### Scope:
Variables in Rust do more than just hold data in the stack: they also own
resources, e.g. `Box<T>` owns memory in the heap. Rust enforces RAII which
is an acronym for Resource Acquisition Is Initialization. What this does is
help the language determine how to clean up, without using a GC. This is done
by enforcing a scope upon allocation, where when that object goes out of said
scope the destructor is called, freeing the resources it once held.

The destructor which is executed on an allocation when it goes out of scope is
implemented through the `Drop` trait. The trait can be implemented with logic
just as any other traits are, allowing for destructor logic to be customizable.
```rust
struct ToDrop;

// custom destructor logic when Drop is executed on resource
// going out of scope.
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
```

By nature of this design, where resources are responsible for cleaning up after
themselves, a resource can thus have only one owner. Changing ownership, or 
shifting the scope of an allocation is known in Rust as a 'move'. Upon being
moved, a resources previous owner can no longer access that resource's place in
memory. This prevents dangling pointers.

It is worth noting that when a resource is moved, some important characteristics
can be changed, such as mutability (`mut`).

It is fair to assume, however, that we'd like to pass around data without each
function taking ownership and destroying the previous reference. This is where
borrowing is introduced. Instead of passing the object itself, they are passed
by reference e.g. instead of passing `T`, reference `&T` is passed.

Mutable data can be mutably borrowed by passing a mutable reference `&mut T`
thus giving read/write access to the borrower. In contrast, an immutable
reference which is borrowed is "frozen". Frozen data can't be modified via the
original object until all references to it go out of scope.

Data can be immutably borrow any number of times, however, while that data is
borrowed the original data cannot be mutably borrowed. In the same light, data
cannot be mutably borrowed and simultaneously immutably borrowed and a mutable
reference cannot be passed in parallel either. Think locking behavior based upon
scope.

Quick note on syntax, RE: references. Both of the below patterns are valid
although I personally prefer the later
```rust
    let ref ref_c1 = c;
    let ref_c2 = &c;
```

#### Lifetimes
A lifetime is a construct of the compiler, specifically the compiler's borrow
checker, which ensures all borrows are valid. A lifetime of a variable, or any
allocation of memory for that matter, begins at instantiation and ends at the
point of its destruction (drop). Lifetimes are _not_ the same as scopes, yet it
is common they are referred to similarly or interchangeably. To understand this
subtle distinction, think in terms of borrowing a reference. The lifetime of
the reference is determined by where that variable is declared, and as a result
only remains valid as long as the lender is not destroyed. Yet the scope of the
borrow is determined by where the reference is used.
```rust
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

Using explicit syntax, `'`, one can set the lifetime of something like
```rust
foo<'a, 'b>
``` 
where `foo` cannot exist past the lifetime of `a` or `b` (delineated by the
single apostrophe). This magic dissipates when you look at functions as mere
closures or scopes on borrows.

A longer lifetime can coerced into a shorter one so that it is 'alive' within a
scope it normally would not work in. See the example in `./lifetime-coercion.rs`
There also exists a notion of static lifetimes which exist for the life of the
running application, thus making them the longest possible lifetimes. Coercion
works both ways though, and a static lifetime may be coerced into a shorter one.
For an example of this see `./lifetime-static.rs`



