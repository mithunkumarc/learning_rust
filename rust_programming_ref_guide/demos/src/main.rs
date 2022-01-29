mod sysargs;
use std::env;

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => sysargs::print_message(n),
        None => panic!("Didn't receive any name ?")
    }
}
