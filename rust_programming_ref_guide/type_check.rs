fn type_of<T>(_: &T) { // confusing, in c we use *T, but rust uses &
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let s = "hello";
    type_of(&s); // output : &str , pass by reference
}


/* pass by value works too: but if the object is big(many states) copying takes memory
fn type_of<T>(_: T) { // confusing, in c we use *T, but rust uses &
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let s = "hello";
    type_of(s); // &str , pass by reference
}

*/
