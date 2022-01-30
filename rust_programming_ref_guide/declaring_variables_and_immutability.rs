fn main() {
    let target = "world"; // by default immutable
    let mut greeting = "Hello"; // mut : indicates mutable
    //target = "Rust!"; // compile error
    println!("{}",target);
    println!("{}", greeting);
    greeting = "Howdy";
    println!("{}", greeting);
}
