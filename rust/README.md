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

### Functions

- snake_case is the conventional style for function and variable names
- `fn` is the function definition keyword

#### Parameters

- special variables that are part of a function's signature
- `arguments` are the concrete values that are provided to a function
- function signatures require parameter type annotations to allow it to give a more helpful error message

#### Statements and Expressions

- function bodies are made up of a series of statements, optionally ending in an expression
    - __Statements__ are instructions that perform somea ction and do not return a value
    - __Expressions__ evaluate to a resultant value
        ```rust
        fn main() {
            let y = {
                let x = 3;
                x + 1
            };

            println!("The value of y is: {y}");
        }
        ```
        - Note the lack of a semicolon. Adding a semicolon to the end of an expression turns it into a statement and it will not return a value.

#### Functions with Return Values

- declare function return value type after an arrow (`->`).
    ```rust
    fn main() {
        let x = plus_one(5);

        println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    ```
    vs.
    ```rust
    fn main() {
        let x = plus_one(5);

        println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1; // This semi-colon means its a statement and returns the unit type (), which contradicts the return value annotation
    }
    ```

### Comments

- `//` is the standard single line comment.
- Sometimes at the end of lines, but more typically used as annotation on a separate line above the code.

### Control Flow

#### `if` expressions

- start with `if` keyword, foollowed by an expression.
- blocks of code are placed in curly braces
    - sometimes called _arms_, like the `match`
- `else` is optional
- will not automatically try to convert non-Boolea types to a Boolean.
- multiple conditions with `else if`.
- `if` is an expression, so it can be used to assign variables (i.e. ternary assignment)
    ```rust
    fn main() {
        let condition = true;
        let number i= if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }
    ```
    - Since they are expressions, the return types cannot be mismatched, or else ther eis an error.

#### Repetition with Loops

- `loop`
    - repeats forever until quit
        - `ctrl-c` can quit
        - `break` will exit the loop
            - Including the value after the `break` will return that value from the loop
        - `continue` skips the remaining code to go to the next iteration
    - `break`/`continue`
        - applies to the inner most loop, if there are multiple
    - loops can be labeled which can be used with `break`/`continue`
        ```rust
        fn main() {
            let mut count = 0;
            'counting_up: loop {
                println!("count = {count}");
                let mut remaining = 10;

                loop {
                    println!("remaining = {remaining}");
                    if remaining == 9 {
                        break;
                    }
                    if count == 2 {
                        break 'counting_up;
                    }
                    remaining -= 1;
                }

                count += 1;
            }
            println!("End count = {count}");
        }
        ```
- `while`
    - Handles some of the nesting that would be necessary if you used `loop`, `if`, `else`, `break`.
- `for`
    - most commonly used for increase safety/conciseness.
    - can avoid an out of index exception when looping through arrays or for a certain number of times using `Range`
    ```rust
    fn main() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 { { // changing this to 6 would cause the code to panic when accessing a[6].
            println!("the value is {}", a[index]);

            index += 1;
        }}

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    ```