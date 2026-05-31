// fn main() {
//     let _guess: u32 = "42".parse().expect("Not a number!");
// }
// if you dont add the : u32, compiler will throw an error because it needs more info

// u32 is 32 bits unsigned integer

// isize and usize depend on the computer architecture, if its a 32 bit computer, then it uses 32 bits and similarly for 64 bits.

// 1_000 = 1000 (just for better readability)

// https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
// goated integer overflow handling in rust, most interesting is at `--release`
// but relying on the integer wrapping overflow is considered an error.

// floating point
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// numeric operations
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
// }

// the boolean type
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// the char type
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// compound types
// group multiple values into one type. tuples and arrays.
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// destructuring tuples
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

// if we want to directly access a tuple element, just use this
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }
//

// array type
// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// array element access
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }
//
//

// invalid array element access
// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }
