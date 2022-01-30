// for increment/changing val must be declared with mut
fn increment(mut val: u32, incrment_val: u32) {
    val += incrment_val;
    println!("{}", val);
}

fn main() {
    let a = 100;
    increment(a, 30);
}
