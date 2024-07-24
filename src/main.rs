

fn main() {
    // println!("Hello, world!");

    // let x = 20;
    // let y = 1000;
    // let z = 100.1;

    // println!("{}",x+y);

    // let mut num = 124;
    // for i in 0..100{
    //     num+=127;
    // }
    // print!("Number: {}", num);



    // let greeting = "hello world";  // --> this is one way of writing skills 
    //  print!("{}", greeting);


    let greeting = String::from("Hello World");

    println!("{}", greeting);

    let char1 = greeting.chars().nth(1);   // if we will have to find the 1st char of "Hello World", then throught .unwrap(), we can easily do this 

    print!("char1 : {}", char1.unwrap());



    /// But what hap[pen if we have to find out the 100 char from greetings , it won't exist ..right ? , but if we try to find from java script , then js simply return the undefined 
    /// which is not good , this means that , we are fetching the value which simply does not exist.
    /// this is the main drawback of Java Script 
    /// But in the rust you cannot fetch the value that does'nt exist ... it will show the run time error 
    /// 






}


// ********** variable **********//
//  exp : let x : i8 = 5;
// breakdown --> 8(number) represent number of bits
// i --> represent .... that the given number is signed number (-ve / +ve)
// u ---> represent unsigned ...which cannot be -ve 
// 
