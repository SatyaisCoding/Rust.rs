

fn main() {
    println!("Hello, world!");

    let x = 20;
    let y = 1000;
    let z = 100.1;

    println!("{}",x+y);

    let mut num = 124;
    for i in 0..100{
        num+=127;
    }
    print!("Number: {}", num);



    let greeting = "hello world";  // --> this is one way of writing skills 
     print!("{}", greeting);


    let greeting = String::from("Hello World");

    println!("{}", greeting);

    let char1 = greeting.chars().nth(1);   // if we will have to find the 1st char of "Hello World", then throught .unwrap(), we can easily do this 

    print!("char1 : {}", char1.unwrap());

//     // unwrap() --> this means that you will ok with the run time



//     // But what hap[pen if we have to find out the 100 char from greetings , it won't exist ..right ? , but if we try to find from java script , then js simply return the undefined 
//     // which is not good , this means that , we are fetching the value which simply does not exist.
//     // this is the main drawback of Java Script 
//     // But in the rust you cannot fetch the value that does'nt exist ... it will show the run time error






// }


// // ********** variable **********//
// //  exp : let x : i8 = 5;
// // breakdown --> 8(number) represent number of bits
// // i --> represent .... that the given number is signed number (-ve / +ve)
// // u ---> represent unsigned ...which cannot be -ve 
// // 


// ********** Conditional Statement **********//

pub fn main(){
    let x = 99;
    let is_even = is_even(x);

    if is_even {
        print!("{} is Even", x);
    }else{
        print!("{} id Odd", x);
    }
}

pub fn is_even(x:i32)-> bool{
    return x%2 ==0;
}




// ********** Loops ********** //

use std::fs::remove_dir;


pub fn main() {
    let str = String::from("Satya Prakash");
    println!("First name {}", get_first_name(str))
    
}

pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;


    println!("");





   


}


// ********** Functions ********** //


fn exampFunction (){
    let a  = 10 ;
     let b =  20 ;
      let sum = do_sum(a,b);
      print!("Sum of {} and {} is {}",a, b, sum);
}

fn do_sum(a:i32,b:i32)->i32{
        return a + b;
}




// ********** Mutability ********** //


        let x: i32 = 1;
        x = 2; // Error because x is immutable
        println!("{}", x);


        // if x is initialized once , then it cannot be reinitialized , this is because , in rust the vairables is immutable...

    //     By default, all variables in Rust are immutable because
    //     1 : Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
    //    2 : Knowing that certain data will not change allows the compiler to optimize code better. 


    fn mutability() {
        let mut x: i32 = 1;
        x = 2; // No error
        println!("{}", x);
    }

    // This code will show no error , because , if we want to make the string mutable , we can use the keyword "mut " and make the variables mutable mannually



// ********** Stack vs Heap **********//

// Rust has clear rules about stack and heap data management:
// 1 :Stack: Fast allocation and deallocation. Rust uses the stack for most primitive data types and for data where the size is known at compile time (eg: numbers).
// 2 : Heap: Used for data that can grow at runtime, such as vectors or strings.


fn stackVsHeap() {
    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
}





  // ********** OwnerShip ********** //

  let s1 = String::from("Hello");


  print!("s1:{}",s1);   //Print --> "Hello"


  let s2 = s1;

  print!("s1:{}",s1);     // Output --> Showing error that the following result given below  : -->


//     let s1  = String::from("Hello");
//     |        -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
//   ...
//   6 |    let s2  = s1;
//     |              -- value moved here
//   7 |    
//   8 |     print!("{}", s1);
//     |                  ^^ value borrowed here after move



 print!("s2:{}",s2);    // Print --> "Hello"




 fn main() {
    let my_string = String::from("hello");
    takes_ownership(my_string);
    println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.


// At any time, each value can have a single owner. This is to avoid memory issues like
//1 :  Double free error.
//2 :  Dangling pointers.


//     Fix?
// Clone the string

    fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // Compiles now
}

    // But what if you want to pass the same string over to the function? 
    // You don’t want to clone it, 
    // and you want to return back ownership to the original function?

    fn main() {
    let s1 = String::from("hello");
    let s2 = takes_ownership(s1);
    println!("{}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); 
    return some_string; // return the string ownership back to the original main fn
}


    // Is there a better way to pass strings (or generally heap related data structures)
    // to a function without passing over the ownership?
    // Yes - References
}

    // ********** Borrowing and references ********** //

// References : -->
// References mean giving the address of a string rather than the ownership of the string over to a function.

    fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
}


// Borrowing : --> 
// You can transferring ownership of variables to fns.
// By passing a reference to the string to the function take_ownership,
// the ownership of the string remains with the original variable, in the mainfunction. 
// This allows you to use my_stringagain after the function call.
    
    fn main() {
    let my_string = String::from("Hello, Rust!");
    takes_ownership(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}

// Mutable references : -->
// What if you want a function to update the value of a variable?

    fn main() {
    let mut s1 = String::from("Hello");
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

    // Try having more than one mutable reference at the same time -


    fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    update_word(&mut s1);
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

//     Rules of borrowing : -->
// 1: There can me many immutable references at the same time

    fn main() {
    let  s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;
    
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}
// No errors


 // 2 : There can be only one mutable reference  at a time ..

    fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = update_word(&mut s1);
    
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
// Error


    // 3 : If there is a mutable reference , you can’t have another immutable reference either.

    fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = &s1;
    
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

// This to avoid any data races/inconsistent behaviour
// * If someone makes an immutable reference , they don’t expect the value to change suddenly .
// * If more than one mutable references happen, there is a possibility of a data race and synchronization issues .


    // ********** Struct ********** //
    //    --> Struct in rust , let you structure data together. Similar to objects in javascript

    struct User {
        active : bool,
        username: String,
        email: String,
        sign_in_control: u64,
    }

    fn main(){
        let user1 = User{
            active : true,
            username: String::from("satya.sk.prakash"),
            email:String::from("satya.sk.prakash@gmail.com"),
            sign_in_control:1,
        };
        print!("User 1 username: {:?}, user1.username");
    }


    // ********** Struct Implementation ********** //

    use std::fmt::Debug;
    struct Rect {
   width: u32,
   height: u32,
}

impl Debug for  Rect {
    fn area(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         write!(f, " the rectangular print like this {}", self.width * self.height)
    }
}

fn main() {
    let rect:React = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {:?}", rect);
}


    // ********** Enums ********** //

    // Enums in rust are similar to enums in Typescript.
    // They allow you to define a type by enumerating its possible variants.

            enum Direction {
            North,
            East,
            South,
            West,
        }
        
        fn main() {
            let my_direction = Direction::North;
            let new_direction = my_direction; // No error, because Direction is Copy
            move_around(new_direction);
        }
        
        fn move_around(direction: Direction) {
            // implements logic to move a character around
        }

        // Why not simply do the following - >

                    fn main() {
                move_around("north".to_string());
            }
            
            fn move_around(direction: String) {
                if direction == "north" {
                    println!("Moving North");
                }
            }

    
    // Because we don’t enforce the 4 variants of directions. 
    // So this is much looser than strictly allowing only 4 variants for direction.
    

            // Define an enum called Shape
        enum Shape {
            Circle(f64),  // Variant with associated data (radius)
            Square(f64),  // Variant with associated data (side length)
            Rectangle(f64, f64),  // Variant with associated data (width, height)
        }
        
        // Function to calculate area based on the shape
        fn calculate_area(shape: Shape) -> f64 {
            // calculates the area of the shape 
            return 0
        }

        
        fn main() {
            // Create instances of different shapes
            let circle = Shape::Circle(5.0);
            let square = Shape::Square(4.0);
            let rectangle = Shape::Rectangle(3.0, 6.0);
            
        }

        // We will be implementing the calcuate_area function in the pattern matching section.







    // ********** Pattern Matching *********** //

    // Let you pattern match across various variants of an enum and run some logic.

    // Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,   // if u need singl;e stmt 
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) =>{                ----->    // if u need multiple stmt
             width * height
        },
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}

    
    
}
