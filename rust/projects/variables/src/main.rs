fn main() {
    // Mutability
    println!("\nMutability");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Consts
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    println!("\nShadowing");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Integer overflow
    println!("\nInteger Overflow");
    const VALUE: u8 = 254;
    const ADDEND: u8 = 2;

    let mut x: u8 = VALUE;
    x = x + ADDEND;  // panics when debugging, overflows to 0 if using the `--release` flag
    println!("The result of {VALUE} + {ADDEND} is: {x}");
    
    let x: u8 = VALUE;
    let x = u8::wrapping_add(x, ADDEND); // explicitly wraps
    println!("The u8 wrapped result of {VALUE} + {ADDEND} is: {x}");

    let x: u8 = VALUE;
    match u8::checked_add(x, ADDEND) { // returns None if overflows
        None => println!("The u8 checked result of {VALUE} + {ADDEND} overflowed"),
        Some(x) => println!("The u8 checked result of {VALUE} + {ADDEND} is: {x}"),
    };

    let x: u8 = VALUE;
    let x = u8::overflowing_add(x, ADDEND); // returns boolean
    println!("The u8 overflowing result of {VALUE} + {ADDEND} is: {:?}", x);

    let x: u8 = VALUE;
    let x = u8::saturating_add(x, ADDEND);
    println!("The u8 saturating result of {VALUE} + {ADDEND} is: {x}");

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of b is: {b}");
}
