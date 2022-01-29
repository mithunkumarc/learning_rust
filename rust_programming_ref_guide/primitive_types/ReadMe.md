1. bool
2. char
3. Integer type

      signed i8 i16 i32 i64 i128
      unsigned u8 u16 u32 u64 u128

4. isize : pointer sized integer type. i32 on 32 bit os, i64 on 64 bit os.
5. usize : same as isize but unsigned
6. f32 : 
7. f64
8. [T; N]: fixed array, for element type T, non negative compile time constant size N.
9. [T]: dynamically sized view into a contiguous sequence, for any type T.
10. str: String slices, mainly used as reference, that is &str.
11. (T, U ..): finite sequece(type?) T and U are different types
12. fn(i32) -> 32: A functiontakes i32 and return i32. Functions also have type.
