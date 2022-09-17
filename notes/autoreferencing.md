#### Auto dereferencing works for struct type, for primitive type use * for deferencing

      struct City {
          name: String
      }

      fn change_struct(c :&mut City) {
          c.name = String::from("durga"); // auto dereferencing for struct references
      }
      
      fn change_value(x :&mut i32) {
          *x += 2;                    // explicit dereferencing for primitive types
          println!("{}", x)
      }

        fn main() {
            let mut z = 1;
            change_value(&mut z);
            println!("{}", z);
            let mut u = City {
                name: String::from("chinmuladri")};
            change_struct(&mut u);
            println!("{}", u.name);
        }
