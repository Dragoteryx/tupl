# Tupl

A small library for handling Rust tuples using traits.\
It provides a `Tuple` trait implemented for tuples of arity (length) 0 to 50.

```rs
use tupl::Tuple;

let tuple = (2, 3);

let tuple = tuple.append(4); // append a new element to a tuple
assert_eq!(tuple, (2, 3, 4));

let tuple = tuple.prepend(1); // prepend a new element to a tuple
assert_eq!(tuple, (1, 2, 3, 4));

let (head, tuple) = tuple.truncate_head(); // truncate the first element from a tuple
assert_eq!((head, tuple), (1, (2, 3, 4)));

let (tuple, tail) = tuple.truncate_tail(); // truncate the last element from a tuple
assert_eq!((tuple, tail), ((2, 3), 4));
```