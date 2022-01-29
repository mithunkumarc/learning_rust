//https://learning-rust.github.io/docs/d3.modules.html

pub fn print_message(name: String) {
    println!("hello : {}",name);
}

/*
some vs none
https://doc.rust-lang.org/std/option/

---main.rs----

mod sysargs;
use std::env;

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => sysargs::print_message(n),
        None => panic!("Didn't receive any name ?")
    }
}

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
https://github.com/GitCredentialManager/git-credential-manager

*/