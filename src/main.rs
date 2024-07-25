

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
}
