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


