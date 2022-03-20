##### Primitive types

Rust has the following built-in primitive types:
  
bool : These are the usual booleans and can be either true or false .
  
char : Characters, such as e .
  
Integer types: These are characterized by the bit width. Rust supports integers
that are up to 128 bits wide:

        signed
        i8
        i16
        i32
        i64
        i128

        unsigned
        u8
        u16
        u32
        u64
        u128

isize : The pointer-sized signed integer type. Equivalent to i32 on 32-bit CPU
and i64 on 64-bit CPU.
  
usize : The pointer-sized unsigned integer type. Equivalent to i32 on 32-bit CPU
and i64 on 64-bit CPU.
  
f32 : The 32-bit floating point type. Implements the IEEE 754 standard for
floating point representation.
  
f64 : The 64-bit floating point type.
  
[T; N] : A fixed-size array, for the element type, T , and the non-negative
compile-time constant size N.
  

[T] : A dynamically-sized view into a contiguous sequence, for any type T .
str : String slices, mainly used as a reference, that is, &str .
  
(T, U, ..) : A finite sequence, (T, U, ..) where T and U can be different types.
  
fn(i32) -> i32 : A function that takes an i32 and returns an i32 . Functions
also have a type.
