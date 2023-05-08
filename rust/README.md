# Learning Rust

## Getting Started

### Installation

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Installed the default installation, which includes `cargo`, `clippy`, `rust-std`, `rustc`, `rustfm`

- Metadata and toolchains installed into the Rustup home directory `/Users/michaelliu/.rustup` and can be modified with the `RUSTUP_HOME` environment variable
- commands added to Cargo's bin directory `/Users/michaelliu/.cargo/bin`
    - commands added to the PATH environment variable by modifying
        - `/Users/michaelliu/.profile`
        - `/Users/michaelliu/.zshenv`
- To configure your current shell, run: `source "$HOME/.cargo/env"`

### Cargo

- Initialize new projects
    ```shell
    cargo new <PROJECT_NAME>
    ```
- Build projects
    ```shell
    cargo run
    ```
- Run projects
    ```shell
    cargo run
    ```
- Install libraries by adding them to the Cargo.toml file under dependencies. e.g.
    ```toml
    [dependencies]
    rand = "0.8.5"
    ```
- Cargo.lock ensures reproducible builds 
- Build documentation for all dependencies locally
    ```shell
    cargo doc --open
    ```

## Common Programming Concepts

### Variables and Mutability

- `let` allows the declaration of a immutable variable
- `let mut` allows the declaration of a mutable variable 
- `const` allows the declaration of a value that is allways immutable
    - the type _must_ be annotated
    - may only be set to a constant expression
        - cannot be set to a value that is computed at runtime
    - __BY CONVENTION__ - all uppercase with underscores between words
- shadowing variables
    - first variable is shadowed by the second and the second
    - repeated use of `let` and the same variable name
    - `let` is important because the variable is still immutable
    - shadowing allows the type of the variable can change
        ```rust
        // valid
        let spaces = "   "; // string
        let spaces = spaces.len(); // number

        // compile time error
        let mut spaces = "   "; // string
        spaces = spaces.len(); // number
        ```
        - avoids different names like `spaces_str` and `spaces_num`

### Data Types

#### Scalar

- Integer
    - signed/unsigned numbers in [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
    - e.g. `i32`, `u32`
    - `isize` and `usize` depends on the archtecture the computer is running on
        - e.g. 64 bits if on a 64-bit archtecture
    - `i32` is a good default integer size generally
    - Can use `_` as a visual separator
        e.g. `1_000` is equivalent to `1000`
    - integer overflow can cause the program to panic
        - when debugging, the compiler will check
        - the `--release` flag will skip overflow checks and use _two's complement wrapping_
            - relying on wrapping is considered an error
        - Explicit handling by:
            - Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
            - Return the None value if there is overflow with the `checked_*` methods.
            - Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
            - Saturate at the value’s minimum or maximum values with the `saturating_*` methods.
- Floating-point
    - `f32` and `f64`
        `f32` is single-precision flooat
        `f64` is double-precision flooat
    - `f64` is default due to increased precision and the assumption that it's running on modern CPUs
    - IEEE-754 standard
- Numeric Operations
    - `+`, `-`, `*`, `/`, `%`,
    - integer division truncates toward zero to the nearest integer
        - e.g. (`-5/3 === -1` and `5/3 === 1`)
- Booleans
    - `bool` keyword
    - 1 byte in size
        i.e. `00000000` or `11111111`
- Characters
    - `char` keyword
    - 4 bytes
        - uses Unicode Scalar Values from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive
    - specified with single quotes, as opposed to double quotes, which use double quotes.

#### Compound Types

- Tuple
    - Group a number of values with a variety of types together
    - fixed length, cannot grow or shrink
    - comma-separated list of values inside parentheses
    - each position in the tuple has a type, but can be different
    - Can use pattern matching to destructure a tuple value
        ```rust
        fn main() {
            let tup = (500, 6.4, 1);

            let (x, y, z) = tup;

            println!("The value of y is: {y}");
        }
        ```
    - or with indexes
        ```rust
        fn main() {
            let x: (i32, f64, u8) = (500, 6.4, 1);

            let five_hundred = x.0;

            let six_point_four = x.1;

            let one = x.2;
        }
        ```
    - _unit_ is a special name ofr a tuple without any values and represents an empty value or an empty return type
        - expresssions implicitly return the _unit_ vavlue if they dont' return any other value
- Array
    - collection of values with the same type
    - fixed length, cannot grow or shrink
    - comma separated list of values inside square brackets
    - stored in a stack, rather than a heap, in memory
    - examples
        ```rust
        let a: [i32; 5] = [1, 2, 3, 4, 5]; // type can be specified like so:

        let b: [3; 5]; // equivalent to [3, 3, 3, 3, 3]
        ```
    - access with squqare brackets and indexes
        - invalid index access will cause the app to panic

