/*
if condition can be used as both branching statment and as expression
1. when using if condition as branching statment, 
use semicolon at the end of block inside if and else.
when using if condition as branching statment,
if else (expression) return empty () paranthesis.
To accomodate paranthesis use {:?}

2. when using if condition as expression, 
don't use semicolon at the end of block inside if and else.

in both two ways mentioned above if else always ends with semicolon
*/
fn main() {
    /*using if else as branching statment */
    let result = if 1 == 2 {        
        // do something(function call)
        println!("Nothing makes sense"); // semicolon important
    } else {
        // do something else(function call)
        println!("Sanity reigns"); // semicolon important.
    };
    println!("Result of computation: {:?}", result);// ()

    /*using if else as expression*/
    let expression_result = if 1 == 2 {
        do_today() //no semicolon, and returns some value
    } else {
        do_tomorrow() // no semicolon, and return some value
    };
    println!("expression result : {}", expression_result); // do it tommorow
}

fn do_today() -> String {
    let x = String::from("do it today");
    x
}

fn do_tomorrow() -> String {
    let x = String::from("do it tomorrow");
    x
}
