#### Stack and Heap

* Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.   
* The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. 
* Adding data is called pushing onto the stack, and removing data is called popping off the stack. 
* All data stored on the stack must have a known, fixed size. 
* Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
* The heap is less organized: when you put data on the heap, you request a certain amount of space. 
* Finding memory for data on heap, this process is called allocating on the heap and is sometimes abbreviated as just allocating 
* Pushing values onto the stack is not considered allocating. 
* Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. 
* Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. 
* Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
* Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 
* When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
 
#### Ownership Rules 

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.


#### Variable Scope

A scope is the range within a program for which an item is valid. Take the following variable:

            {                      // s is not valid here, it’s not yet declared
                let s = "hello";   // s is valid from this point forward
                                   // do stuff with s
            }                      // this scope is now over, and s is no longer valid



In other words, there are two important points in time here:

1. When s comes into scope, it is valid.
2. It remains valid until it goes out of scope.

At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we’ll build on top of this understanding by introducing the String type.

---

#### String type


* To illustrate the rules of ownership, we need a data type that is more complex “Data Types” like string/object(struct). 
* The types covered previously are of a known size(primitive types), can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope. 
* But we want to look at data that is stored on the heap(object) and explore how Rust knows when to clean up that data, and the String type is a great example.
* We’ll concentrate on the parts of String that relate to ownership. These aspects also apply to other complex data types, whether they are provided by the standard library or created by you. 
* We’ve already seen string literals, where a string value is hardcoded into our program. 

       let s = "hello"; // also called as &str, you will see later
       
* String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. One reason is that they’re immutable. Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, String. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:

       let s = String::from("hello");
       


* The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from. refer :  namespacing with modules in “Paths for Referring to an Item in the Module Tree”. This kind of string can be mutated:

      let mut s = String::from("hello");
      s.push_str(", world!"); // push_str() appends a literal to a String
      println!("{}", s); // This will print `hello, world!`

* So, what’s the difference here? Why can String be mutated but literals cannot? The difference is in how these two types deal with memory.


---

#### Memory and Allocation

* In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

* With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
  * The memory must be requested from the memory allocator at runtime.
  *  We need a way of returning this memory to the allocator when we’re done with our String

* That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

* The second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

* Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example  using a String instead of a string literal:

       {
            let s = String::from("hello"); // s is valid from this point forward 
           // do stuff with s
       }                                  // this scope is now over, and s is no longer valid

* There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called **drop**, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

* This pattern may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we’ve allocated on the heap. Let’s explore some of those situations now.



