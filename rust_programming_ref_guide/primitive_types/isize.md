Use usize and isize when it’s related to memory size – the size of an object, or indexing a vector, for instance. 
It will be a 32-bit number on 32-bit platforms, as that’s the limit of memory they can address, and likewise for 64-bit.


[discussion](https://users.rust-lang.org/t/i32-vs-isize-u32-vs-usize/22657)
