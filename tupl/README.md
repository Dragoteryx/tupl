# Tupl

A `#![no_std]` crate for handling Rust tuples using traits.\
It also provides traits to manipulate functions.

```rust
use tupl::{GrowableTuple, NonEmptyTuple};

let mut tuple = (1, 2);

// access the first & last elements of a tuple
*tuple.head_mut() = *tuple.head() + 1;
*tuple.tail_mut() = *tuple.tail() + 1;
assert_eq!(tuple, (2, 3));

// append an element to a tuple
let tuple = tuple.append(4);
assert_eq!(tuple, (2, 3, 4));

// prepend an element to a tuple
let tuple = tuple.prepend(1);
assert_eq!(tuple, (1, 2, 3, 4));

// truncate the first element of a tuple
let (head, tuple) = tuple.truncate_head();
assert_eq!((head, tuple), (1, (2, 3, 4)));

// truncate the last element of a tuple
let (tuple, tail) = tuple.truncate_tail();
assert_eq!((tuple, tail), ((2, 3), 4));
```

