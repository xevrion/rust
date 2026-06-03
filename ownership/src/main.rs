// A VERY IMPORTANT THING IS STARTING HERE MUWHEHEHEEHEH
// borrowing, slices, and how Rust lays data out in memory.

// so there are 3 types of languages,
// 1st where the language it self does the garbage collection
// 2nd is where the programmer must explicitly allocate and free the memory.
// 3rd is rust, its memory is managed through a system of ownership with a set of rules that the compiler checks.
//
// ❤️❤️❤️❤️❤️❤️❤️❤️ LOVELY EXPLANATION OF "The Stack and the Heap" ❤️❤️❤️❤️❤️❤️❤️❤️
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap

// Some rules:
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// Variable Scope: (A scope is the range within a program for which an item is valid.)

// {                      // s is not valid here, since it's not yet declared
//     let s = "hello";   // s is valid from this point forward

//     // do stuff with s
// }                      // this scope is now over, and s is no longer valid

// Now lets talk about String Type
// as its complex enough to properly understand ownership and the same things which we will learn for 'String' will apply for other complex data types as well.
