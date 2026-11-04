# 02 Variables

Variables, mutability, constants and shadowing; code in [practice/variables](../practice/variables).

## What & why

- **`let x = 5;`**: a variable, immutable by default
    - why: a value that can't change can't be changed by mistake elsewhere
- **`let mut x = 5;`**: a mutable variable
    - why: changing a value is opt-in, so every change is visible in the code
- **`const MAX: u32 = 100;`**: a constant, type required, fixed at compile time
    - why: named values used across the program, e.g. limits
- **shadowing `let x = x + 1;`**: a new `x` that hides the old one, may change type
    - why: reuse a name for a transformed value without making it mutable
- **`println!("x is {x}");`**: prints a line; `!` marks a macro, `{x}` inserts the value

## Steps

1. **Break the rule: reassign an immutable variable.**
    - once, from the repo root: create the project
        ```bash
        cargo new practice/variables --vcs none
        ```
    - `src/main.rs`
        ```rust
        fn main() {
            let x = 5;
            println!("x is {x}");
            x = 6;
            println!("x is {x}");
        }
        ```
    - run, from the repo root ([workspace](01-install.md#steps), step 7)
        ```bash
        cargo run -p variables
        ```
    - expect: compile error, nothing printed; the program never runs
        ```
        error[E0384]: cannot assign twice to immutable variable `x`
         --> src\main.rs:4:5
        2 |     let x = 5;
          |         - first assignment to `x`
        4 |     x = 6;
          |     ^^^^^ cannot assign twice to immutable variable
        help: consider making this binding mutable
        2 |     let mut x = 5;
          |         +++
        ```
    - reading it
        - `E0384`: error code; `rustc --explain E0384` explains it with examples
        - `src\main.rs:4:5`: file, line 4, column 5
        - `-`: first assignment; `^^^^^`: the forbidden one
        - `help:` + `+++`: suggested fix, text to add
        - **binding**: Rust's word for the name ↔ value link
2. **Fix it with `mut`.**
    - `src/main.rs`, line 2
        ```rust
        let mut x = 5;
        ```
    - run, from the repo root
        ```bash
        cargo run -p variables
        ```
    - expect
        ```
        x is 5
        x is 6
        ```

## Lessons
- `let` is immutable; changing a value needs `mut`
- the compiler checks the whole program before it runs: errors come before any output

## Problems
- `error[E0384]: cannot assign twice to immutable variable`
    - cause: assigning a new value to a `let` variable without `mut`
    - fix: `let mut x = 5;`

## Changes
- 2026-11-04 — step 2: fix with `mut`, output `x is 5` / `x is 6`
- 2026-11-04 — step 1 runs from the repo root via the workspace: `cargo run -p variables`
- 2026-11-04 — step 1 result: E0384 error and how to read it; lessons, problems added
- 2026-11-04 — page created: what & why, step 1
