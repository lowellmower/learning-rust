use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    // constructor
    fn new() -> List {
        Nil
    }

    fn prepend(self, data: u32) -> List {
        Cons(data, Box::new(self))
    }

    // `self` has to be matched, because the behavior of this method
    // depends on the variant of `self`
    // `self` has type `&List`, and `*self` has type `List`, matching on a
    // concrete type `T` is preferred over a match on a reference `&T`
    fn len(&self) -> u32 {
        // Can't take ownership of the tail, because `self` is borrowed;
        // instead take a reference to the tail
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(4);
    list = list.prepend(2);

    println!("Linked list has length {}", list.len());
}