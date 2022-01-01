# Rust Notes

## Warnings

- Be aware on iterating / looping on empty slices or collections. The iteration will not happen, but subsequent code is still executed. On day03, function most_common_byte_at(), the input slice came empty, the for loop did not execute and in consequency the return was Some(0).

## Generic Iterator with &str

Puzzle Reference is ADV21, Day 03, Part 02. There was not used a fully generic call.

<https://stackoverflow.com/questions/51758485/why-cant-i-use-iteratoritem-string-as-an-iterator/51758645>

```rust
fn max_width<T: AsRef<str>>(strings: impl IntoIterator<Item = T>) -> usize {
    let mut max_width = 0;
    for string in strings {
        let string = string.as_ref();
        if string.len() > max_width {
            max_width = string.len();
        }
    }
    max_width
}
```

```rust
fn max_width<T: AsRef<str>>(strings: impl IntoIterator<Item = T>) -> usize {
    strings
        .into_iter()
        .map(|s| s.as_ref().len())
        .max()
        .unwrap_or(0)
}
```

## String Processing

Nice link: <https://lise-henry.github.io/articles/optimising_strings.html>

## Error Handling and the ? Operator

Not able to mix the ? operator with Options and Results: <https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no>

Parameter type not living long enough: <https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no>

## Iterators

<https://doc.rust-lang.org/std/iter/index.html>
<https://doc.rust-lang.org/std/iter/trait.Iterator.html>
<https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html>

### Iterators of pairs

Slices already have the methods `chuncks()` and `chuncks_exact()` to create iterator yielding multiples elements by iteration.

## General Tips

Always prefer clarity while coding. Coding needs to be maintained and be easy to understand after the initial design. For example, consider the snippets bellow:

```rust
let y0 = min(p0.y, p1.y);
let y1 = max(p0.y, p1.y);
```

```rust
let (y0, y1) = (min(p0.y, p1.y), max(p0.y, p1.y));
```

Both snippets compare two values and saves the minimum and maximum values into variables. The second one may be more concise, but the first convey much better the intent.
