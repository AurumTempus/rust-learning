# Weather Journal

A small Rust project that simulates a simple temperature log using tuples and arrays.

## What it does

- Stores fixed-length daily temperature data as an array of tuples (`(&str, f32)`)
- Prints each entry in a readable format
- Calculates the average temperature
- Extracts all dates into a vector
- Finds the day with the highest temperature

## What I learned

While building this project, I practiced:

- Using arrays and tuples in Rust
- Working with iterators (`iter()`)
- Transforming data using `map()` and collecting into `Vec`
- Finding values using `max_by()` with custom comparators
- Handling `Option` and using `unwrap()`
- Understanding lifetimes (`'a`) when returning references
- Using `*` to dereference a reference — e.g., `*max_entry` turns a `&(&str, f32)` into `(&str, f32)`


