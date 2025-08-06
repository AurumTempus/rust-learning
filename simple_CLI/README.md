# Command-Line Interpreter (Rust Mini CLI)

A simple command-line interpreter written in Rust to practice working with enums, pattern matching, user input, and basic CLI behavior.

## Features

- Supports the following commands:
  - `add x y` — adds two integers
  - `subtract x y` — subtracts the second integer from the first
  - `echo text` — repeats the given text
  - `invalid` or any unknown command — returns an error message
- Accepts commands via hardcoded test data and interactive user input
- Processes input using enums and match expressions

## What I practiced

- `enum` for representing different command types
- `match` for clean and safe branching
- `Vec`, `String`, `&str` and slicing
- Parsing user input from `stdin` using `BufRead`
- Basic CLI logic with input loop and graceful exit
- `unwrap_or` for simple error handling

##  Example usage

```text
> add 5 10
Sum: 15

> subtract 10 4
Difference: 6

> echo Hello world!
Echo: Hello world!

> something
Invalid command

