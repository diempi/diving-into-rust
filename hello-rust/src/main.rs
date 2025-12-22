fn froggy(s: String){
    println!("{s} {s} {s} {s} {s} {s}");      
}

fn main() {
    println!("Hello, world!");
    println!("Hello, world! again");

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

    // Vectors

    let mut v = vec![1, 2, 3];
    println!("Vector v is a vector and its actual value is {:?}", v);

    // push to the vector
    v.push(4);
    println!("Vector v is a vector and its new value is {:?}", v);

    v.push(5);

    println!(
        "adding to the Vector v is a vector and its new value is {:?}",
        v
    );

    // Remove from vector

    v.pop();

    println!(
        "removing from the Vector v is a vector and its new value is {:?}",
        v
    );

    // Iterators

    let vec = vec![1, 2, 3, 4, 5];
    let mut iter = vec.iter();

    println!(
        "the first value of the iterator is {}",
        iter.next().unwrap()
    );
    println!(
        "the first value of the iterator is {}",
        iter.next().unwrap()
    );
    println!(
        "the first value of the iterator is {}",
        iter.next().unwrap()
    );
    println!(
        "the first value of the iterator is {}",
        iter.next().unwrap()
    );
    println!(
        "the first value of the iterator is {}",
        iter.next().unwrap()
    );
    //println!("the first value of the iterator is {}",iter.next().unwrap());

    // Functions

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let result = add(3, 5);
    println!("the result of the addition is {result}");

    // Conditional Flow

    let number = 7;
    if number < 5 {
        println!("number is less than 5");
    } else if number > 5 {
        println!("number is greater than 5");
    } else {
        println!("number is equal to 5");
    }

    froggy("🐸".to_string());

}
