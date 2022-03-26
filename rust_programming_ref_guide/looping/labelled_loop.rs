/*
If you have nested loops you may get confused when you want use break, since you cannot refer loop by name.
so loop can be labelled. use label to break loop. 

syntax : 'label: loop {}

*/

fn main() {
    let mut x = 100;
    'outer: loop {
        'inner: loop {
            x = x - 1;
            if x == 50 {
                break 'outer; // breaking outer loop when x is 50
            } else {
                println!("value of x is {}",x);
            }
        }
    }
}
