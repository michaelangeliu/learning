
fn main() {
    // Initial program
    // `area` should calculate the area of a retangel, but the function takes in two parameters and it's not clear the parameters are related
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_width_x_height(width1, height1)
    );

    // With a tuple, we can use a single argument, but there's no name to the width/height
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_dimensions(rect1)
    );

    // with a struct, we can use a single argument and the properties are clearly named
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect2)
    );

    // println!("{}", rect2); // println! macro cannot default "Display" print structs because there can be ambiguity in the formatting
    println!("{:?}", rect2); // :? uses the "Debug" print to allow devs to see its value when debugging code. May need to add the `#[derive(Debug)]` attribute to the struct definition for this to work
    println!("{:#?}", rect2); // :#? uses the "Debug" print with formatting

    // dbg! macro returns ownership of the value
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area()
    )
}

fn area_width_x_height(width:u32, height: u32) -> u32 {
    width * height
}

fn area_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}