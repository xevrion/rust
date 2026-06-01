// snake case for function naming and variable naming
// e.g. hi_this_is_good

// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// rust doesnt care where you define the function, before calling it, after calling it.
// but, it should be in the same scope

// parameters
//
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// in function signatures, you must declare the type of each parameter.
//
//
// when defining multiple parameters, use comma as a separator

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Statements and Expressions
//
// Statement - they do not return values.
// fn main() {
//     let y = 6; // this is a statement
// }

// now lets prove that statements dont return value, run the following function
// fn main(){
//     let x=  (let y = 6);
// }

// this shows that you cant write x = y = 6, like you do in C or Ruby.
//

// Expression - they return a value.
// fn main() {
//     let y = {
//         let x = 3; // this is a statement, it does not return a value, it just assigns 3 to x
//         x + 1 // this is an expression, it evaluates to 4, and then that value is assigned to y
//     };
//     println!("The value of y is: {y}");
// }

// functions with return values
// fn five() -> i32 {
//     // the -> i32 syntax indicates that this function will return a value of type i32
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// another example
// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
