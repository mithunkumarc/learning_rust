fn main() {
    let mut x = 100;
    loop {
        if x <= 0 {
            break;
        } else if x % 2 == 0 {
            x -= 1; // if you miss here, it will goto infinite loop
            continue;
        } else {
            println!("x : {}",x); // only odd numbers
        }
        x -= 1;
    }
}
