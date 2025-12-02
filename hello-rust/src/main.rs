fn main() {
    println!("Hello, world!");

// Variables
    let x = 5;
    println!("x is {x}");
    let mut y = 6;
    println!("Current value of Y is {y}");
    y = 9;
    println!("New value of Y is {y}");

    let z: i32 = 10;
    println!("Z is a integer and its value is {z}");

// Floats

    let f = 3.14;
    println!("fl is a float and its value is {f}");
// Slices
    let s = String::from("Hello world");
    let hello = &s[0..5];
    println!("hello is a slice of s and its value is {hello}");

    let world = &s[6..11];
    println!("world is a slice of s and its value is {world}");

}
