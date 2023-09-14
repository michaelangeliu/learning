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

## Understanding Ownership

### Stack & Heap memory

- Stack
    - LIFO = Last In, First Out
        - push & pop onto the stack
    - must have a known szie
- Heap
    - returns a pointer to the address of the allocated space
    - allocate/deallocate memory
- pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data.
- allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
- accessing data in the heap is slower than on the stack because you must follow a pointer

### Ownership Rules

- each value in Rust has an _owner_.
- there can only be one owner at a time.
when the owner goes out of scope, the value will be dropped.

### Variable Scope

#### The String Type

- allocated onto the heap
- can be created using a string literally using the `from` function
    ```rust
    let s = String::from("hello");
    ```
- When a variable goes out of scope, it will call a special function called `drop` to return the memory
    - In C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_.
- Simple vs. complex values
    ```rust
    let x = 5; // stores 5 into memory
    let y = x; // stores 5 into memory

    // vs.

    let s1 = String::from("hello"); // adds the meta-data/pointer and also stores the string contents
    let s2 = s1; // essentially copies the meta data, but does not copy the heap that the pointer refers to. Copying the heap memory could make it very expensive
    ```
    - a String is made of three parts
        - `ptr` = points at the memory that holds the string contents
        - `len` = length of the string
        - `capacity` = total space allocated
    - when `s2` and `s1` both go out of scope, both will try to free the same heap memory (_double free_ error).
    ```rust
        let s1 = String::from("hello");
        let s2 = s1;

        println!("{}, world!", s1); // compile error because Rust considers s1 no longer valid after the reassignment
    ```
- Rust will never automatically create "deep" copies of data, so _automatic_ copying is assumed to inexpensive
- `clone` will allow a "deep" copy
    ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    ```
- Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack. If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    - If the type or any of its parts has implemented the `Drop` trait, it cannot be implemented with `Copy`
    - e.g.
        - integers
        - Booleans
        - floating points
        - chars
        - tuples if they only contain types that also implement `Copy`

#### Ownership and Functions

- Passing aa value to a function is similar to assigning a value to a variable
- will move/copy, just like an assignment
    - e.g.
        - passing a String will transfer ownership
        - passing an integer will make a copy
- returning the value will transfer ownership back if the value is assigned
- Could potentially be tedious by using tuples to return multiple values:
    - e.g. this scenario where we want to use both the string and the length
        ```rust
        fn main() {
            let s1 = String::from("hello");

            let (s2, len) = calculate_length(s1);

            println!("The length of '{}' is {}.", s2, len);
        }

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String

            (s, length)
        }
        ```
- _references_ allow using a value without transfering ownership

### References and Borrowing

- a reference is like a pointer that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference
    ```rust
    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    ```
    - no tuple code and they use `&` in both the argument and the function definition
        - `&` represents _references_, which allows values to be referenced without taking ownership of it.
- _borrowing_ = the act of creating a reference
- also immutable by default but we can create mutable references
    ```rust
    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    ```
    - cannot create multiple mutable __simultaneous__ references to a value
        - allows for mutation, but carefully
        - can prvent data races
            - e.g.
                - two or mre pointers access the same data at the same time
                - at least one of thpointers is being used towrite to the data
                - there's no mechanism being used to synchronize access to the data
        - using a curly brace can create separate scopes to allow for multiple mutable references, just not _simultaneous_ ones
    - similar rule for combining mutable and immutable references

#### Dangling References

- _dangling pointer_ = pointer that references a location in memory that may have been given to someone else
```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
- Solution:
    ```rust
    fn no_dangle() -> String {
        let s = String::from("hello");

        s // return the string
    }
    ```

- The Rules of References
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.

### Slice Type

- A _string slice_ is a reference to part of a `String`
    ```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    ```
    - Just references a portion of a string using a starting and ending index
    - Ommitting the index means starting at either the beginning or end of the string
    - Avoids the value being freed when there are useless references to the size later.
- Swapping `&String` with `&str` increases the flexibility through _deref coercions_
    ```rust
    fn first_word(s: &String) -> &str { ... }
    // vs
    fn first_word(s: &str) -> &str { ... }
    ```
    - this is because `&str` is a slice pointing to a specific point of the binary data
- Arrays can also be sliced
    ```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // type of &[i32]

    assert_eq!(slice, &[2, 3]);
    ```

## Structs

### Defining and Instantiating Structs

- Essentially JS objects
    ```rust
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // builds and returns a User struct with these properties. Reduces duplication of property names by using the same name as a shorthand
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    ```
    - structs can be mutable, but the whole thing must be mutable to reassign any individual properties using dot notation.
- Struct update syntax allows the reuse of previous values using the `..` syntax
    ```rust
    fn main() {
        // --snip--

        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
        // vs.
        let user3 = User {
            email: String::from("another@example.com"),
            ..user1 // note, this won't compile because `user1` is already moved above.
        };
    }
    ```
- Tuple Structs
    Tuple strctus are useful when the whole tuple needs a name and it should be different than other tuple types.
    ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    ```
- Unit-like Structs
    - Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

### Debugging Structs

- `#[derive(Debug)]` - prepended to a struct allows the values to be debug printed with the `println!` macro
    - debug printing uses `{:?}` or `{:#?}`, which changes the default formatting.
- `dbg!` macro
    - an alternative uses the `dbg!` mactro, which will take ownership of the expression
    - `println!` just takes a reference
    - This will also print the file, the line number, the resultant value of the expression, and returns ownership of the value
    - prints to `stderr`, but `println!` prints to `stdout`

### Method Syntax

- similar to functions
    - declared with the `fn` keyword and a name
    - can have parameters and a return value
    - contain code that is run
- defined in the context of a struct (or an enum or trait object)
    - first parameter is always `self`, which represents the kinstance of the struct the method is being called on
- `impl` (implementation) block allows defining the context where the method is defined
    - `&self` is shorthand for `self: &Self`, which in the `impl` block, `Self` is an alias for the type that the `impl` block is for.
    - may be abbreviated this with only the name `self` in the first parameter spot
    - Must still use `&` to indicate that this method borrows the `Self` instance and is immutable
- a _getter_ could be defined with a method to make the field private and the method public, to enable read-only access to that field

#### Associated Functions

- are not methods because they don't have `self` as the first parameter
    - e.g. `String::from`
    ```rust
    impl Rectangle {
        // creates a square without needing to define both the width and height
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    ```
    - `Self` keyword in the return type are aliases for the type of the `impl` keyword
- `impl` blocks can be separated which could be useful for generic types and traits

## Enums and Pattern Matching

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
- Any IP address can either be v4 or v6, but not both at the same time
- Allows functions to be defined to take either variant
    ```rust
    fn route(ip_kind: IpAddrKind) {}
    ```
- rather than putting an enum into a struct, we can put data directly into each enum variant
    - the name of the enum variant also becomes a function that constructs an instance of the enum
    - variants can have different types and amounts of associated data
        e.g. IPv4 have four numeric components between 0 and 255, but we may still want IPv6 to be aa single string. 
- There is a [standard library for ip address encodings](https://doc.rust-lang.org/std/net/enum.IpAddr.html)
- We could use structs to individually define variants, but it would be more difficult to define a function to take any of the types

### `Option` enum

```rust
enum Option<T> {
    None,
    Some(T),
}
```
- allows rust to avoid having a `null` value.
    - `None` is still better because using the `Option` type allows the compiler to check if the appropriate actions are implemented, which avoides the problem of assuming that something isn't null when it actually is.
- part of the standard library prelude
- Will need to get the value out of the `Option<T>` to use it
    - https://doc.rust-lang.org/std/option/enum.Option.html

#### `match` control flow

- similar to a `switch/case` or a series of `if` conditions, but doesn't need to evaluate to a Boolean value
- e.g. coin sorter
- matches are exhaustive, so all "arms" must be covered
    - helps prevent forgetting to explicitly handle the `None` case for `Option<T>`.
- `match` has two catch-all patterns
    - `other` can cover all the remaining cases, when we want to use the value in the predicate arm
    - `_` can cover all the remaining cases, when we do NOT want to use the value in the predicate arm
        - prevents a wanring about an unused variable

### if let

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// vs

let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```
- The `_ => ()` is just boilerplate if we're not doing anything with it, but switching to `if let` loses the exhaustive pattern matching
- can inclue an `else` statement to handle the cases for the catch-all too.

## Packages, Crates, and Modules

- __Packages__ = A Cargo feature that lets you build, test, and share crates
- __Crates__ = A tree of modules that produces a library or executable
- __Modules__ and __use__ = Let you contro the organization, scope, and privacy of paths
- __Paths__ = A way of naming an item, such as a struct, function, or module

### Packages and Crates

- _crate_ = smallest amount of code that the Rust compiler considers at a time
- `rustc` considers the single file that is passed as a crate
- 2 forms
    - binary crate = programs compiled to an executable that can be run (CLI program or server)
        - must have a function called `main`
    - library crate = define functionality intended to be shared with multiple projects
        - references to "crate" usually mean a "library" crate
- _package_ = bundle orf one or more crates that provides a set of functionality
    - contains a _Cargo.toml_ file that describes how to build those crates
    - Cargo is actually a package
    - must contain at least one crate
        - many binary crates
        - only one library crate
    - __BY CONVENTION__ - _src/main.rs_ is the crate root of a binary crate
    - __BY CONVENTION__ - _src/lib.rs_ is the crate root for a library crate
    - may have multiple binary crates by placing files in the _src/bin_ directory

### Defining Modules to Control Scope and Privacy

- [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)
    1. Start from the crate root (_src/lib.rs_ or _src/main.rs_)
    2. Declare modules - compiler looks for the modules code
        - e.g. a "garden" module could be declared with `mod garden;`. The compiler looks for the module's code
            - inline (by replacing the semicolon with curly brackets)
            - in the file _src/garden.rs_
            - in the file _src/garden/mod.rs_
                - This is an older style, but still supported (can be confusing to have many files named _mod.rs_)
    3. Declare sub modules - Therse are declared in the other files
        - e.g. a "vegetable" submodule could be declared with `mod vegetables;` in _src/garden.rs_. The compiler looks for the submodule's code
            - inline (by replacing the semicolon with curly brackets)
            - in the file _src/garden/vegetables.rs_
            - in the file _src/garden/vegetables/mod.rs_
                - This is an older style, but still supported (can be confusing to have many files named _mod.rs_)
    4. Paths to code in modules - once a module is part of your crate, you can refer to code in that module from anywhere else in the same crate as long as privacy rules allow
        - NOT the same as `include` in other languages because only needed once for the whole project. All references will refer to the loaded file's code
    5. Private vs public - code is private by default, but can be made public by using `pub mod`
    6. `use` keyword can create shortcuts to reduce repetition of long paths

### Paths for referring to an item in the Module Tree

- show where to find an iterm in a module tree
    - _absolute path_ = full path starting from a crate root
        - external crates start with the crate name
        - current crate starts with the literal `crate`
            - like using `/` to start from the filesystem root in your shell
    - _relative path_ starts from the current module and uses `self`, `super`, or an identifier in the current modules
- separate identifies with double colons (`::`)
- depends on whether you're more likely to move item definition code separately from or together with the code that uses the itme.
    - generally absolute path is preferred because it's more likely we'll want to move code definitions and item calls independently of each other.
- parents can't use private items in child modules, but child modules can use the items int heir ancestor modules
    - child modules wrap and hide their implementation details
    - defaulting to private and hiding inner implementaiton details lets you know which parts of the inner code you can change without breaking outer code.
- __BEST PRACTICE__ - packages can contain both _src/main.rs_ and _src/lib.rs_ with the package name by default. Packages containing both types of crates will typically have just enough code in the binary crate to start an executable that calls code with the lbirary crate. This allows other projects to benefit rom the most functionality that the package provides because the library crate's code cna be shared.
    - Module tree should be defined in the _src/lib.rs_, only public items can be used by the binary crate. So the binary crate is just another user of the library crate, just like a completely external crate would use the library crate.
- `super` allows construction of relative paths that begin in the parent module rather than the current module or crate root. (i.e. filesystem path with the `..` syntax)

#### Making Structs and Enums Public

- Structs - `pub` will only make the struct public, but the fields are still private.
- Enums - `pub` will make all of its variants public

### Bringing Paths into Scope with the `use` keyword

- `use` creates a shortucut to a path allowing the shorter name everywhere else in scope.
- similar to a symbolic link in the filesystem
- only a shortcut for the particular scope in which the `use` occurs
    - can't use the shortcut in child modules, etc.
- Creating Idiomatic `use` Paths
    - Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
    - For structs, enums, and other items, it's idiomatic to specify the full path.
- Creating new names with the `as` keyword
    - allows for changing the name when there are two types with the same name.
- Rexporting Names with `pub use`
    - brings item into scope, but also makes the item available for other sto bring into their scopee
    - useful when the internal structure of your code is different from how programmers calling your code would think about the domain.
        - we can write the code with one structure but expoes a different structure
        - makes our library organized for programmers working on the library and programmers calling the library
- External Packages
    - can be used by adding them to _Cargo.toml_
- Nested Paths
    - Can change
        ```rust
        // --snip--
        use std::cmp::Ordering;
        use std::io;
        // --snip--
        ```
        to
        ```rust
        // --snip--
        use std::{cmp::Ordering, io};
        // --snip--
        ```
    - or
        ```rust
        // --snip--
        use std::io;
        use std::io::Write;
        // --snip--
        ```
        to
        ```rust
        // --snip--
        use std::{self, Write};
        // --snip--
        ```
- Glob Operator
    - `*` brings all public items in a defined path into scope
    - be careful since it can make it harder to tell what names are in scope and where a name used in your program was defined
    - often used when testing or as part of the prelude pattern

## Common Collections

- most other data types represent one specific value, but collections can contain multiple values.
- unlike built-in array an dtuple types, the data these collections point to is stored on the heap,
    - means the amount of data does not need to be known at complie time and can grow or shrink as the program runs
- 3 often used collections
    - _vector_ = store a variable number of values next to each other.
    - _string_ = collection of characters
    - _hash map_ = assoiate a value with a particular key
        - specific implementation of a more general data structure called a _map_
    - [Other Collections](https://doc.rust-lang.org/std/collections/index.html)

### Storing Lists of Values with Vectors

- Create a new empty vector with `Vec::new` or `vec!`
    - `push` will add elements to the vector if it is mutable
- Read elements of vectors via indexing or the `get` method
    - Using indexing will cause the program to panic because it references a nonexistant element `let does_not_exist = &v[100];`
    - Using `get` returns an `Option`, so it can rreturn `None`
- Iterate through a vector using a `for-in` loop.
- Enum to store multiple types
    - Vectors can only store values that are the same type, so using an enum allows different types to be held
    - The compiler needs to know how much memory on the heap will be needed to store each element
    - Enum plus a `match` expression will ensure at compile time, every case is handled
        - traits can be used if the exhhaustive set of types is unknown
- Drop element 

### Storing UTF-8 ESncoded Text with Strings

- `str` is the only String type in the core language
    - usually seen in its borrowed form `&str`
- `String` is provided by Rust's standard library
    - gorwable, mutable, owned, UTF8-encoded
- `String` is a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities
    - `to_string` method is available to any type that implements the `Display` trait
        - creates a `String` from a string literal
    - `from` creates a String from string literal
- Updating Strings
    - `push_str` to append a string slice
    - `push` to append a single character
    - `+` uses the `add` method
        ```rust
        fn add(self, s: &str) -> String {}
        ```
        - copiler coerces the `&String argument into a `&str`
    - `format!` macro for Multiple concatenations
- Indexing into Strings
    - `String` cannot be indexed by an integer because unicode scalar values may take more than one byte of storage
        - Strings can be viewed as bytes, scalar values, and grapheme clusters to allow for handling many different human languages
        - Not an O(1) operation because Rust would have to walk through the contents from the beginning to the index to determine valid characters
- Slicing Strings
    - Since it's not clear what the return type of the string indexing operation should be, Rust uses a a range to create a string slice containing particular bytes
    - if the slice only contains part of a characater's bytes, it will panic at runtime
- Methods for iterating over Strings
    - `chars` will return the separate characters
    - `bytes` will return each raw byte
        - Some valid Unicode scalar values may be made up of more than 1 byte
    - grapheme clusters are complex, so it's not included in the standard library
- other methods are built into the standard library
    - `contains` for searcdhing in a string
    - `replace` for substituting parts of a string

### Hash Maps

- `HashMap<K, V>` to store data with keys of tkype `K` and values of type `V`
    - not commonly used, so it is not included with the prelude
- Create with `new`
- Add elements with `insert`
- Access with `get`
    - can be iterated through with a `for` loop in an arbitrary order
- types that implement the `Copy` trait will be copied into the hasmap
- owned values will be moved and the hash map will be the owner of those values
    - inserting references to values into the hash map won't move the values, but the values must be valid for at least as long as the hash map is valid
- Updating has options, depending on the desired outcome
    - Overwrite the value by calling `insert` twice. Will only keep the latest insert.
    - Adding a key and value only if a key isn't present
        - `entry` is a special API which returns an enum called `Entry` that represents a value that might or might not exist.
        - `Entry::or_insert` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists
    - Updating a value based on the old value
        - `Entry` returns a mutable reference, which can be dereferenced and modified
- Default hashing function is _SipHash_ for resistance to DoS attacks involving hash tables
    - more secure, but slightly slower, can be changed by using a different hashing function which implements the `BuildHasher` trait

## Error Handling

- Two major categories: _recoverable_ and _unrecoverable_ errors
    - Recoverable errors can just be reported to the user and the operation retried
        - `Result<T, E>` is used
    - Unrecoverable errors are the symptoms of bugs
        - `panic!` macro stops execution

### Unrecoverable Errors with `panic!`

- By default `panic!` will
    1. print a failure message
    2. unwind
        - walks bback up the stack and cleans up the data from each function it encounters
        - _aborting_ is an alternative, which ends the program without cleaning up
            ```toml
            [profile.release]
            panic = 'abort'
            ```
    3. clean up the stack
    4. quit
- via an environment variable, we can have Rust display the call stack to make it easier to track down the source of the panic
    ```shell
    RUST_BACKTRACE=1 cargo run
    ```
    - debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag

### Recoverable Errors with `Result`

- `Result`` is an enum
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```
- `Result`, like `Option`, is brought into scope by the `std` prelude.
- Using the `match` expression is one way to handle the `Result`.
    ```rust
    use std::fs::File;

    fn main() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
    ```
- We can nest `match` to handle different errors
    ```rust
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }
    ```
    - `file::open` returns an `Err` variant of `io::Error`, which has a `kind` metthod to get an `io::ErrorKind` value (e.g. `ErrorKind::NotFound`)
- Using a closure can be more concise than using `match`.
    - can use the `unwrap_or_else` method to pull the type out of the `Result` enum.

#### Shortcuts for Panic on Error: `unwrap` and `expect`

- `unwrap` will return the value inside `Ok` or call `panic!`
- `expect` lets us choose the `panic!` message, to make tracking down the source of panic easier
    ```rust
    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }
    ```
    - generally use `expect` over `unwrap` in production quality code to provide more context about why the operation is expected to always succeed. If assumptions are wrong, there's more information to use in debugging.

#### Propagating Errors

- Returning the error to the calling code can give more control to the calling code where there might be more information or logic that dictates how the error should be handled
- `?` operator is a shortcut for propagating errors
    - defined to work the same as the `match` ... `return Err`
    - error values that have the `?` operator called on them go through the `from` function, which is used to convert values from one type into another.
        - this returns one error type ot represent all the ways a function might fail, even if parts might fail for many different reasons
    - chainable
        ```rust
        use std::fs::File;
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username = String::new();

            File::open("hello.txt")?.read_to_string(&mut username)?;

            Ok(username)
        }
        ```
        - instead of creating a variable `username_file`, the `read_to_string` is chained directly to the result of `File::open("hello.txt")?`
    - reading a file to a string is fairly common, so the `fs` library provides `fs::read_to_striGng`, which opens a file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it. Handling all the errors.
    - can only be used when the return type of the calling function is a `Result` or `Option` or a type that implements `FromResidual`, since it's defined to use the `match` and those specific early return arms.
        - if it's an `Option`, it will either return `None` or the type inside `Option<T>`
        - will not convert between `Result` and `Option` or vice versa.
            - We can use `Result::ok` or `Option::ok_or` to explicitly convert
- `main` is special because it's the entry and exit point of executable programs
    - can return aa `Result<(), E>`
    - if `main` returns `Ok(())`, it will exit with a value of `0` and will exit with a nonzero value if `main` returns an `Err`
        - This is compatible with the C convention.

### To `panic!` or Not to `panic!`

- Using `Result` gives the calling code options to recover when defining a function that might fail
- In prototype code/tests, it might be more useful to call `panic!`
    - useful before deciding how to handle errors
    - `unwrap` and `expect` leave markers in the code for when we're ready to make the program more robust
    - We can use `expect` to temporarily mark logically impossible failures
        - e.g. if we hardcode something that should never fail, if we end up not hard coding something in the future, the error will need to be handled.
        ```rust
        use std::net::IpAddr;

        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("Hardcoded IP address should be valid");
        ```
        - parse returns a `Result` and the compiler will want us to handle the `Err` variant as a possibility
            - the compiler isn't smart enough to know that the hard coded IP address is always valid, but if the IP did come from a user, then it does have a possibility of failure so we know to come back and make the `expect` more robust
- Panicking is advised if the code could end up in a _bad state_
    - e.g.
        - some assumption, guarantee, contract, or invariant has been broken (invalid, contradictory, missing values)
        - AND one or more of the following:
            - something that is unexpected, as opposed to something that will likely happen occasionally (e.g. a user enters data in the wrong format)
            - code after this point needs to rely on not being in this bad state, rather than checking for a problem at every step
            - not a good way to encode this information in the types we use
    - if continuing could be insecure or harmful, calling `panic!` encourages the person using the library to fix the bug during development
- if failure is an expected possibility, then returning `Result` helps force the calling code to decioide how to handle it.
- using the type system takes advantage of using the compiler to help ensure that only appropriate code is compiled
    - e.g. if we don't use `Option`, then there must be a value since `None` cannot be returned

#### Creating Custom Types for Validation

- We can create a new type that encapsulates validations and forces the calling function to use the type
    ```rust
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
    ```
    - `value` is a private field in `Guess` and can only be accessed through the `value` getter function.
        - keeping the field private ensures that the field cannot be set outside of using `new`, which forces the value through the validation.

## Generic Types, Traits, and Lifetimes

- Tools for effectively hadnling the duplication of concepts
- _generics_ are abstract stand-ins for concrete types or other properties
- _traits_ define behavior in a generic way and can be combined with generic types to constrain them to only accept types that have a particular behavior
- _lifetimes_ are a variety of generics that give the compiler information about how references relate to each other
    - can ensure references will be valid in more situations than it coudl without our help
- We can use _functions_ to abstract duplicated functionality without dealing with generics
    1. Identify duplicate code
    2. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
    3. Update the two instances of duplicated code to call the function instead

### Generic Data Types

- Any identifier can be used as a type parameter name. `T` is used because, by convention, type parameters names are short and Rust's type-naming ocnvention is PascalCase. `T` is short for "type".
    - The generic type parameter name must be included in the function signature. e.g. `fn largest<T>(list: &[T]) -> &T { ... }` or `impl<T> Point<T> { ... }`
- Generic types can be restricted to those that implement certain traits
- Can use multiple generic type parameters by including more type parameter names.
    - Too many can make the code hard to read
- `Option<T>` and `Result<T, E>` are both enums thhat can hold generic data types in their variants.
- We can implement methods on structs that are only defined for specific types, constraining them to specific instances that apply to that specific type.
    - Other instances of the struct that are not of that type won't have that method defined.f

#### Performance

- generic types won't make the program run any slower than it would with concrete types.
    - _monomorphization_ of the code turns generic code into specific code by filling in the concrete types that are used when compiled.
    - makes generics efficient at runtime

### Traits: Defining Shared Behavior

- _traits_ define functionality a particular type has and can share with other types
    - similar to a feature often called _interfaces_ in other languages
- _trait bounds_ specify that a generic type can be any type that has certain behavior
- way to group method signatures together to definde a set of behaviors necessary to accomplish some purpose
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    ```
    - Instead of an implementation, we end it with a semicolon, allowing each type implementing the trait to provide its own custom behavior for the body of the method.
    - multiple methods can be included per a trait, one per line, each ending in a semicolon
- implementing a trait on a type uses `impl <TRAIT_NAME> for <TYPE>`
    ```rust
    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    ```
    - at least one of the trait or the type must be local to our crate. Both cannot be external
        - _coherence - orphan rule_ restriction ensures that other people's code can't break your code. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.
    - default behavoir can be implemented using `{ ... }` instead of `;`
        - when implementing, we can use an empty block (e.g. `impl Summary for NewsArticle {}`)
- It's possible to use overriding implementations within a default implementation
    - e.g.
    ```rust
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String { // Only needs to define summarize_author
            format!("@{}", self.username)
        }
    }

    // Calling .summarize from a Tweet type will return the "Read more..." along with the summarize_author infomration
    ```
- traits can be used on parameters to bind parameters to specific traits
    ```rust
    pub fn notify(item: &impl Summary) { // Must pass a type that implements Summary
        // ...
    }

    // The above is syntactic sugar for the Trait Bound Syntax
    pub fn notify<T: Summary>(item: &T) {
        // ...
    }
    ```
    - The `impl Trait` syntax can allow the same method to have parameters that use different types. The Trait Bound synatx must be used if we want them to have the same type
        ```rust
        pub fn notify(item1: &impl Summary, item2: &impl Summary) { }
        // vs.
        pub fn notify<T: Summary>(item1: &T, item2: &T) { }
        ```
    - `+` syntax can be used to bind multiple traits
        ```rust
        pub fn notify(item: &(impl Summary + Display)) { }
        // vs.
        pub fn notify<T: Summary + Display>(item: &T) { }
        ```
    - `where` is an alternate syntax for multiple trait bounds to increase clarity
        ```rust
        fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }
        // vs.
        fn some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        { }
        ```
- return types can implement traits
    ```rust
    fn returns_summarizable(switch: bool) -> impl Summary { }
    ```
    - can only use `impl Trait` if we're returning a single Type
        - restriction around how the `impl Trait` syntax is implemented in the compiler
        - There is a way to work around this discussed later
- We can conditionally implement traits based on restricdtions
    - i.e. we can implement certain traits for every generic type or add trait bounds to restrict the need to implement a trait to only types that meet the prerequisites
- _blanket implementations_ can be used to implement a trait for any type that implements another trait
    - e.g. the standard library implements the `ToString` trait on any type that implements the `Display` trait, allowing us to call `.to_string` on type that implements `Display`.
        ```rust
        impl<T: Display> ToString for T { }
        ```

### Validating References with Lifetimes

- generic that ensures that references are valid as long as we need them to be.
- required by Rust to ensure the actual references used at runtime will definitely be valid
- _dangling reference_ cause a program to reference data other than the data it's intended to reference
    - e.g. declaring a variable in the outer scope without initializing it, setting it in an inner scope, then referencing it back in the outer scope
        - the variable has gone out of scope before we try to use it. 
- The _borrow checker_ compares scopes to determine whether all borrows are valid

#### Generic lifetimes in Functions

- When the return value has an ambiguous lifetime, it needs a lifetime annotation
    ```rust
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
    - we don't know whether the reference to `x` or to `y` will be returned and the compiler doesn't know either, so adding generic lifetime parameters help define the relationship between the references so the borrow checker can perform its analysis
- Lifetime parameter names must start with an apostrophe (`'`) and are usually all lowercase and very short
    - place the lifetime parameter after the `&` of a reference, using a space to separate the annotation from the reference's type.
    - `'a` is often used for the first lifetime annotation
    - Lifetime annotations in function signatures are declared inside angle brackets
        ```rust
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        ```
        - for some lifetime `'a`, the function takes two parameters, both are string slices that live at least as long as lifetime `'a`
    - doesn't change the lifetime, but adding a constraint that rejects values that don't adhere to these contraints
        - lifetime annotations become part of the contract of the function, much like the types in the signature
- Lifetime Annotations in Struct Definitions
    - if the struct holds references, then references in the struct's definition need lifetime annotations
        ```rust
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        ```
- Lifetime Elision
    - set of particular cases where lifetime annotations are automatically handled by the compiler because they follow a deterministic pattern
    - if there's ever ambiguity, then the compiler will default to erroring, which can be resolved by adding explicit annotations
    - _input lifetimes_ = lifetimes on function or method parameters
    - _output lifetimes_ = lifetilmes on return values
    - 3 rules (1 for input, 2 for output) to figure out the lifetimes of the referencew when there aren't explicit annotations
        1. Each parameter that is a reference gets its own lifetime parameter
        2. if there is only 1 input lifetime parameter, it is assigned to all output lifetime parameters
        3. if tthere are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, then the lifetime of `self` is asssigned to all output lifetime parameters

- Lifetime Annotations in Method Definitions
    - Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
    - method signatures may be dependent or independent of the lifetimes of the struct's fields

- Static Lifetime
    - `'static` denotes that the affected reference _can_ live for the entire duration of the program
    - Some error messages may recommend using `'static`, but think about if the reference actually needs to live the entire lifetime of the program or not.
        - usually because of dangling references or a mismatch of available lifetimes

## Writing Automated Tests

- In his 1972 essay "The Humble Programmer," Edsger W. Dijkstra said that "Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence." That doesn’t mean we shouldn’t try to test as much as we can!
- types helps handle correctness, but automated tests can help ensure the functionality works as intended

### How to Write Tests

- Typically perform three actions
    1. Set up any needed data or state
    2. Run the code you want to test.
    3. Assert the results are what you expect
- Tests are annotated with the `test` attribute
    - Add `#[test]` on the line before the `fn`
    - Can be run with `cargo test`
    - library projects will automatically generate a test module

### Checking Results with the `assert!` Macro

- `assert!` requires an argument that evaluates to a Boolean
    - `true` -> nothing happens, test passes
    - `false` -> calls panic! and test fails
- Testing equality
    - `assert_eq!` = test for equality
        - uses `left` and `right`, instead of `expected` and `actual`
        - uses `==`
    - `assert_ne!` = test for inequality
        - useful for when we know what the value shouldn't be
        - uses `!=`
    - Must implement `PartialEq`, and `Debug` traits
        - usually as easy as adding `#[derive(PartialEq, Debug)]` to the struct or enum definition
- Adding Custom Failure Messages
    - any argumets specified after the required arguments are passed along to the `format!` macro
- `#[should_panic]` = tests that the code should panic, fails if the code doesn't panic
    - placed after the `#[test]` attribute
    - will panic, even for reasons not than the one we're expecting
    - optional `expected` parameter
- Using `Result<T, E>` in Tests
    - enables you to use the question mark operator, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant
        - won't panic, so we can't use `#[should_panic]`
            - instead use `assert!(value.is_err())` without the `?` operator on the `Result<T, E>` value.

### Controlling How Tests Are Run

- `cargo test` compiles code in test mode and runs the resulting test binary
    - default runs tests in parallel and caputres output generated during test runs
    - only shows test result output
        - i.e. `println!` won't be displayed
    - use `--` to separate arguments
        - `cargo test --help` = displays options for `cargo test`
        - `cargo test -- --help` = displays options for after the `--` separator
- `--test-threads=1` = restricts tests to running consecutively
- `--show-output` = will allow prints to be displayed
- `cargo test <NAME>` = will only run the tests that match the `<NAME>`
- `#[ignore]` = excludes tests if added underneat the `#[test]` attribute
    - `--ignored` = only runs ignored tests
    - `--include-ignored` = runs all tests, including ignored

### Test Organization

- _unit tests_ = small, more focused, testing individual modules in isolation. Can test private interfaaces
    - Put in the _src_ directory, in each file with the code that they're testing
    - Convention is to create a module named `tests` in each file to contain test function and to annotate the module with `cfg(test)`
        - `#[cfg(test)]` = compile and run the test code only when you run `cargo test`, not when you run `cargo build`
            - `cfg` = _configuration_
                - tells Ruest that the following item should only bie included given a certain configuration option.
    - private functions are testable within the file since `use super::*` brings the file's scope in to allow it to be tested
- _integration tests_ are entirely external to yoru library and use your code in the same way any other external code would. Only uses public interfaces
    - put in the _tests_ directory, next to _src_
    - each file is compiled as an individual crate, so include the library in scope with `use`
        - this allows each file to create it's own scope similar to how users might use the crate
    - no `#[cfg(test)]` annotation needed because the _tests_ directory is special
    - to share code within the _tests_ directory, use the older `mod.rs` syntax within a folder
        - This tells Rust not to treat the module as an integration test file
        - Files in subdirectories of the _tests_ directory don’t get compiled as separate crates or have sections in the test output.
- Three sections of tests unit -> integration -> doc
    - if a preceding section fails, the following sections will not be run.
- binary crates with only a _src/main.rs_ file do not expose functions that other crates can use
    - best practice to keep the _src/main.rs_ simple and import the _src/lib.rs_ modules as needed.
        - the _src/lib.rs_ modules can be integration tested

## I/O Project: Building a Command Line Program

- `grep` = Global search a Regular Expression and Print
    - searches a specified file for a specified string
    - arguments
        - file path
        - search string
    - reads the file, finds lines in that file that contain the stirng argument and prints those lines
- `stderr` - prints to the standard error console stream, instead of the standard output `stdout`

### Accepting Command Line Arguments

- `--` double hyphens indicate the following arguments are for our program rather than for `cargo` when we run `cargo run`.
- Can bring in external modules at any level, but `use std::env` so that we can use other functions and it's less ambiguous when calling `env::args`
    - `std::env::args` Only takes valid unicode
        - can use `args_os`, which produces `OsString`, which differs depending on the operating system
    - `collect` = turns the iterator into a vector containing all the values produced by the iterator.
        - needs to be annotated because Rust isn't able to infer the kind of collection
    - first avalue of `args` is the binary (like in C)
- `fs::read_to_string` takes a file_path and returns a `std::io::Result<String>`

### Refactoring to Improve Modularity and Error Handling

- Separate functionality so each function is responsbile for a single task. Keeps the `main` function from being cluttered
    - currently parses arguments and reads the file
- Group configuration variables into one structure to make their purpose clear
    - `query` and `file_path` are for configuraiton, but `contents` is business logic
- `expect` will print an error when reading the file fails, but too generic
    - could be missing file, could incorrect permissions, etc.
- `expect` being used in multiple places can trigger an `index out of bounds` error if there are not enough arguments
    - consolidating the error-handling code in one place helps future maintainers and ensures we're printing meaningful messages

#### Separation of Concerns for Binary Projects

- Split into _main.rs_ and _lib.rs_ and move the logic into _lib.rs_
- if the command line parsing logic is small, it can remain in _main.rs_
    - if it gets complicated, move it to _lib.rs_
- Limit `main` function to
    - calling the command line parsing logic with the argument values
    - setting up any other configuration
    - calling a `run` function in _lib.rs_
    - handling the error if `run` returns an error
- _main.rs_ handles running the program
    - since `main` cannot be tested directly, should be small enough to verify correctness by reading it
- _lib.rs_ handles the business logic

#### Grouping Configuration Values

- Using a struct can give a properties a meaningful name instead of a generic tuple that is immediately broken apart again
- `Config` owns the `String` values
    - using `clone` makes a full copy of the data for the `Config` instance to own.
        - more time and memory than storing a reference
        - more staightforward because we don't have to manage the lifetimes of the references

#### Creating a COnstructor for Config

- `parse_config` creates a new `Config`, so it makes sense to make it a `new` function instead
    - more idiomatic grouping the function with  the struct itself

#### Fixing the Error Handling

- Using `panic!` is helpful, but more appropriate for a programming problem than a usage problem
- Returning `Result` may be more appropriate
    - return `Err` when there's an issue
        - `'static` lifetime is ok for the error values since it will exist for the lifetime of the program
    - return `Ok` when successful
- `unwrap_or_else` will return the inner `Ok` value, but if there is an `Err`, it will pass the error message into the anonymous function