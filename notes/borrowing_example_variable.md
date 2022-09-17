

        fn main() {
         let mut x = 100;
         let y = &mut x;
         //println!("{}", x); // error: y borrowed x's reference
         *y = 200; // use dereferencing to update value
         println!("{}", y);
        }
