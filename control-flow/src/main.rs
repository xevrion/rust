// if expressions

// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// Multiple conditions with else if
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Using if in a let statement
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// if else types are not compatible (will throw an error)
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// repeating code with `loop`
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// returning values from loops
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Disambiguating with Loop Labels
// // // //
// If you have loops within loops, break and continue apply to the innermost loop at that point.
// but we can also use loop labels to break or continue other loop and not directly impact inner most loop.. lemme show an example

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up; // break the outermost (mentioned loop)
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// now,
// Streamlining Conditional loops with while
// so, while the condition is true, the loop runs. when its false, the loop breaks~
// example:

// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }
//     println!("LIFEOFF!!!");
// }

// Looping Through a Collection with `for`
// eg. that we can also do this with while loop, but we have for loop too which will be shown in the next example.
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// now the same with `for` loop.
// but yeah here we made the code more safer, because in the while loop there might be chances of bugs that we would go beyond the scope of the array but that wont happen here, as we're letting rust do that :) and even the machine code generated from `for` loops is more efficient because index doesnt need to be compared to the length of the array at every iteration :)
// e.g.
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// Introducing Range
// countdown but with a range , and also rev();
// fn main() {
//     for number in (1..4).rev() {
//         // 4 isnt inclusive
//         println!("{number}!");
//     }
//     println!("LIFTOFF!");
// }

// Some practice questions :3

// 1) Convert temperatures between Fahrenheit and Celsius.

// Let's just do *F to *C;

// use std::io;

// fn main() {
//     println!("Input temprature in Fahrenheit:");
//     let mut temp_f = String::new();

//     io::stdin().read_line(&mut temp_f).expect("Failed");

//     let temp: f32 = match temp_f.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("That was not a valid integer!");
//             return;
//         }
//     };

//     let temp_c: f32 = (temp - 32.0) * 5.0 / 9.0;

//     println!("Temprature in Celcius: {temp_c}");
// }

// 2) Generate the nth Fibonacci number.

// use std::io;
// fn main() {
//     println!("Input a number N:");
//     let mut number = String::new();

//     io::stdin().read_line(&mut number).expect("Failed");

//     let mut number: i32 = match number.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("That was not a valid integer!");
//             return;
//         }
//     };

//     let mut first = 1;
//     let mut second = 1;
//     let mut third = 1;
//     while number - 2 > 0 {
//         third = first + second;
//         first = second;
//         second = third;
//         number -= 1;
//     }
//     println!("nth fibonacci number is: {third}");
// }

// 3) Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// fn main() {
//     let days = [
//         "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
//         "tenth", "eleventh", "twelfth",
//     ];
//     let gifts = [
//         "and a partridge in a pear tree.",
//         "two turtle doves,",
//         "three French hens,",
//         "four calling birds,",
//         "five gold rings,",
//         "six geese a-laying,",
//         "seven swans a-swimming,",
//         "eight maids a-milking,",
//         "nine ladies dancing,",
//         "ten lords a-leaping,",
//         "eleven pipers piping,",
//         "twelve drummers drumming,",
//     ];

//     for i in 0..12 {
//         println!(
//             "On the {} day of Christmas,\nmy true love sent to me:",
//             days[i]
//         );
//         if i == 0 {
//             println!("a partridge in a pear tree.");
//             println!();
//             continue;
//         }
//         for j in (0..=i).rev() {
//             println!("{}", gifts[j]);
//         }
//         println!();
//     }
// }
//
//
