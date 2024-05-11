A [TermIter] trait abstraction for generating/consuming sequences of values with an explicit termination value

The fundamental method is [TermIter::into_next] which consumes the [TermIter] to either produce an [Update::Next] with both an item and a `Self` state for continuing, or else an [Update::Terminate] with the [TermIter::Terminal] value. Because [TermIter::into_next] consumes the state, if it terminates, there is no further state available to the caller (and any such state is necessarily dropped).

# Example

```
use termiter::TermIter;
use std::io::{BufRead, BufReader, Read};

fn count_lines_and_chars<R>(r: R) -> std::io::Result<(usize, usize)>
where
    R: Read,
{
    let mut lines = 0;
    let mut chars = 0;
    let seq = BufReader::new(r).lines().terminate_on_err();
    seq.for_each(|line| {
        lines += 1;
        chars += line.chars().count();
    })?; // Notice the '?' propagation.

    Ok((lines, chars))
}
```

# [Iterator] name collisions

Because [TermIter] provides a lot of similar functionality to [Iterator], the same method names are used where it makes sense. Meanwhile, there is a blanket impl for [TermIter] for every [Iterator], which is convenient for enabling any [TermIter] consuming API to be passed an [Iterator] directly (or likelywise for [IntoTermIter]).

This means in some cases there is method name ambiguity:

## Example: Method Name Ambiguity
```rust,compile_fail
use termiter::TermIter;

let it = 0..5;
let mut acc = 0;

// Do we mean `Iterator::for_each` or `TermIter::for_each`?
it.for_each(it, |inc| acc += inc);

assert_eq!(acc, 10);
```

The most direct solution is to use [Fully Qualified Syntax for Disambiguation](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name):

```rust
use termiter::TermIter;

let it = 0..5;
let mut acc = 0;
TermIter::for_each(it, |inc| acc += inc);
assert_eq!(acc, 10);
```

However, this can be avoided wherever variable bounds can disambiguate:

```rust
use termiter::TermIter;

fn sum_elements<S>(seq: S) -> u64
where
    S: TermIter<Item = u64>
{
    let mut acc = 0;
    seq.for_each(|inc| acc += inc);
    acc
}

let sum = sum_elements(0..5);
assert_eq!(sum, 10);
```

