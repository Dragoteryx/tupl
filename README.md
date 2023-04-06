# Tupl

A small library for handling Rust tuples using traits.\
It provides a `Tuple` trait implemented for tuples of arity (length) 0 to 50.

```rust
use tupl::Tuple;

let mut tuple = (2, 3);

// access the first & last elements of the tuple
assert_eq!(&2, tuple.head());
assert_eq!(&mut 2, tuple.head_mut());
assert_eq!(&3, tuple.tail());
assert_eq!(&mut 3, tuple.tail_mut());

// append a new element to a tuple
let tuple = tuple.append(4);
assert_eq!(tuple, (2, 3, 4));

// prepend a new element to a tuple
let tuple = tuple.prepend(1);
assert_eq!(tuple, (1, 2, 3, 4));

// truncate the first element of a tuple
let (head, tuple) = tuple.truncate_head();
assert_eq!((head, tuple), (1, (2, 3, 4)));

// truncate the last element of a tuple
let (tuple, tail) = tuple.truncate_tail();
assert_eq!((tuple, tail), ((2, 3), 4));
```