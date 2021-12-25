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
