/*
for , can be used like iterating array/map
    syntax : 
    for index in iterator {

    }
*/

fn main() {
    // excludes last number
    for i in 1..10{
        println!("{}",i); // prints from 1 to 9
    }
    println!("***********");
    // inclusive range, includes last number/element
    for i in 1..=10{
        println!("{}",i); //prints 1 to 10
    }
}
