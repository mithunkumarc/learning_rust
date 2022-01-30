fn main() {
    hello();  //simple function without params
    let a: u64 = 100;
    let b: u64 = 200;
    let result = add(a, b);
    println!("{}",result) // 300
}

fn hello() {
    println!("hello");
}

fn add(a: u64, b: u64) -> u64{
    a + b // no return statement required
}
