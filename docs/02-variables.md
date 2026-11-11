# 02 Variables

Variables, mutability, shadowing, types and constants; code in [practice/variables](../practice/variables).

## Setup

- once, from the repo root: create the project
    ```bash
    cargo new practice/variables --vcs none
    ```
- one program per exercise: `src/bin/<name>.rs`
    - each file is its own binary in the same package; nothing gets overwritten
    - run one, from the repo root ([workspace](01-install.md#workspace))
        ```bash
        cargo run -p variables --bin <name>
        ```
    - `-p`: exact package `name` from `Cargo.toml`; `--bin`: file name without `.rs`
    - started as `src/main.rs`, moved later
        ```powershell
        New-Item -ItemType Directory practice/variables/src/bin; Move-Item practice/variables/src/main.rs practice/variables/src/bin/mutability.rs
        ```
- `println!("x is {x}");`: prints a line; `!` marks a macro, `{x}` inserts the value

## Immutable by default

- `let x = 5;` creates a variable that can't change
    - why: a value that can't change can't be changed by mistake elsewhere
    - **binding**: Rust's word for the name ↔ value link
- exercise: reassign it, `src/bin/mutability.rs`
    ```rust
    fn main() {
        let x = 5;
        println!("x is {x}");
        x = 6;
        println!("x is {x}");
    }
    ```
    ```bash
    cargo run -p variables --bin mutability
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
- reading an error
    - `E0384`: error code; `rustc --explain E0384` explains it with examples
    - `src\main.rs:4:5`: file, line 4, column 5
    - `-`: first assignment; `^^^^^`: the forbidden one
    - `help:` + `+++`: suggested fix, text to add

## Mutability

- `let mut x = 5;` creates a variable that can change
    - why: changing is opt-in, so every change is visible in the code
- exercise: fix E0384 in `mutability.rs`, line 2
    ```rust
    let mut x = 5;
    ```
- expect
    ```
    x is 5
    x is 6
    ```

## Shadowing

- `let x = x + 1;` creates a new variable with the same name; it hides the old one
    - why: reuse a name for a transformed value, stay immutable, even change the type
- how it works
    - the right side runs first, reading the old `x`
    - from that line on the name means the new `x`, until the end of its scope
    - the old one is untouched; after a `{ }` block ends it is visible again
    - **scope**: the `{ }` block a variable lives in
- shadowing vs `mut`
    - `let x = x + 1;`: new variable, old value kept, type may change, immutable again
    - `x = x + 1;`: same variable overwritten, needs `mut`, type must stay
- when to use it
    - transform but keep the name: `let input = input.trim();`
    - change the type: `let spaces = spaces.len();` instead of `spaces_str` / `spaces_num`
- exercise: `src/bin/shadowing.rs`
    ```rust
    fn main() {
        let x = 5;           // x #1 = 5
        let x = x + 1;       // x #2 = x #1 + 1 → 6, hides #1
        {
            let x = x * 2;   // x #3 = x #2 * 2 → 12, hides #2 inside { }
            println!("inner x is {x}");
        }                    // #3 is gone, x means #2 again → 6
        println!("outer x is {x}");

        let spaces = "   ";
        let spaces = spaces.len();   // text → number: shadowing may change the type
        println!("spaces is {spaces}");
    }
    ```
    ```bash
    cargo run -p variables --bin shadowing
    ```
- expect
    ```
    inner x is 12
    outer x is 6
    spaces is 3
    ```
    - `.len()`: length of a text, here 3 spaces

## Types don't change

- every variable gets one type when created; `mut` changes the value, never the type
    - why: the compiler knows what every value is, so mix-ups are caught before running
- **type inference**: Rust works out the type from the value, no need to write it
    - `"   "` → `&str`: a piece of text (string slice)
    - `.len()` → `usize`: whole number ≥ 0, for sizes and lengths
- exercise: `src/bin/types.rs`, change the type with `mut`
    ```rust
    const MAX_POINTS: u32 = 100_000;

    fn main() {
        let mut spaces = "   ";
        spaces = spaces.len();
        println!("spaces is {spaces}, max is {MAX_POINTS}");
    }
    ```
    ```bash
    cargo run -p variables --bin types
    ```
- expect: compile error
    ```
    error[E0308]: mismatched types
     --> practice\variables\src\bin\types.rs:5:14
    4 |     let mut spaces = "   ";
      |                      ----- expected due to this value
    5 |     spaces = spaces.len();
      |              ^^^^^^^^^^^^ expected `&str`, found `usize`
    ```
- fix: shadow instead, and drop the now unneeded `mut`
    ```rust
    let spaces = "   ";
    let spaces = spaces.len();
    ```
    - or a new name: `let len = spaces.len();`
- expect: `spaces is 3, max is 100000`

## Constants

- `const MAX_POINTS: u32 = 100_000;` a value fixed at compile time
    - why: named values used across the program, e.g. limits
- rules
    - type always written out; `u32` = whole number ≥ 0, 32 bits
    - names in `UPPER_SNAKE_CASE`
    - never `mut`; can live outside `fn main()`
    - `100_000`: `_` only for readability, same as `100000`
- exercise: in `types.rs` above

## Errors and warnings

- **error**: the program is wrong; the build stops, nothing runs (e.g. E0384, E0308)
- **warning**: the program is valid but suspicious; it builds and runs
- both show file:line:column, `^` under the spot, and often a `help:` fix
- why care about warnings: they often point at a forgotten line
- met so far
    - `unused variable: x`: a value made and never read
    - `variable does not need to be mutable`: `mut` on a variable never changed
- `_x`: a leading underscore marks a variable unused on purpose and silences the warning

## Lessons
- `let` is immutable; changing a value needs `mut`
- the compiler checks the whole program before it runs: errors come before any output
- shadowing makes a new variable; `mut` changes the existing one
- `mut` changes the value, never the type
- a warning still runs; read it anyway, it often marks a forgotten line
- `-p` takes the exact package name from `Cargo.toml`

## Problems
- `error[E0384]: cannot assign twice to immutable variable`
    - cause: assigning a new value to a `let` variable without `mut`
    - fix: `let mut x = 5;`
- `error[E0308]: mismatched types` (expected `&str`, found `usize`)
    - cause: storing a number in a `mut` variable that holds text
    - fix: shadow with `let`, or use a new name
- `error: package(s) 'variable' not found in workspace`
    - cause: typo in `-p`; the package is `variables`
    - fix: use the `name` from the package's `Cargo.toml`
- `warning: unused variable: 'x'` (shadowing.rs:5)
    - cause: inner `x` created but never read; its `println!` was missing
    - fix: add `println!("inner x is {x}");` inside the block, or name it `_x` if unused on purpose
- `warning: variable does not need to be mutable` (types.rs:4)
    - cause: `mut` left on a variable that is shadowed, never changed
    - fix: remove `mut`

## Changes
- 2026-11-11 — restructured: one section per concept instead of "What & why" + "Steps"; types (E0308, inference, `&str`, `usize`) and constants added
- 2026-11-11 — shadowing (how, vs `mut`, when), error vs warning, `src/bin/` added; steps 3-4; lessons and problems for `-p` typo and unused variable
- 2026-11-04 — step 2: fix with `mut`, output `x is 5` / `x is 6`
- 2026-11-04 — step 1 runs from the repo root via the workspace: `cargo run -p variables`
- 2026-11-04 — step 1 result: E0384 error and how to read it; lessons, problems added
- 2026-11-04 — page created: what & why, step 1
