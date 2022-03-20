/*
Closures are like functions but have more information of
the environment or scope in which they are declared.
helpful link : https://www.howtosolutions.net/2020/12/rust-closure-compiler-error-unit-type-not-implementing-std-fmt-display-trait/
*/
fn main() {
    let doubler = |x| x * 2; // lambda/anonymous function
    let value = 5;
    let twice = doubler(value);
    println!("{} doubled is {}", value, twice);

    //closure
    let big_closure = |b, c| {
        let z = b + c;
        // have access to twice outside current scope 
        z * twice   // implicit return, don't end with semicolon
    }; // expression, ends with semicolon
    
    let some_number = big_closure(1,2);
    println!("{}", some_number);
    
}
