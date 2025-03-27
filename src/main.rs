// fn main() {
//     println!("Hello, world!");
// }

// primatuve data types 
// int float boolean char

// integers are of two type unsigned which contains only positives values and signed which can hold positives values
// u8, u16, u32, u64, u128
// i8, i16, i32, i64, i128

//fn main() {

    // // int types
    // let x: i32 = -139;
    // let y: u32 = 100;

    // println!("the signed interger value is {}", x);
    // println!("the signed unsigned interger value is {}", y);

    // float contain floating point value
    // f32 f64
    
    // let f: f32 = 3.99;
    // println!("this is a float value {}", f);

    //

//}

// compund data type 
// array tuples strings and slices 

//arrays



// fn main() {
//     let array: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("my array is {:?}", array);
//     let fruit: [&str; 5] = ["apple", "bannana", "orange", "pear", "apple"];
//     println!("my fruit arrays are {:?}", fruit);
//     println!("printing fruit index 2 {}", fruit[2]);

//     // tuple
//     let human: (String, i32, bool) = ("alice in wonderland".to_string(), 30, true);
//     println!("print tuple {:?}", human);

//     //slice 
//     let animal_slice: &[&str] = &["dog", "cow", "cat"];
//     println!("animal slice {:?}", animal_slice);
//     let mut string: String = String:: from("hello");
//     string.push_str("yoo");

//     println!("it print this: {}", string);

//     let slice: &str = &string[0..5];
//     println!("it going to print this slice: {}", slice );

//     add(4, 4);

//    let weight = 32.5;
//    let height = 65.5;
//   let bmi = bmi(weight, height);
//   println!("the bmi is {:.2}", bmi)

// }

// fn add(a: i32, b: i32)  {
//     let c = a + b;
//     println!("sum of both numbers is: {}", c)
// }



// fn bmi(weight: f64, height: f64) -> f64 {
//     let h2 =height * height;
//     let answer = weight / h2;
//     return  answer
// }


// ownership 
//rules in owneship 
// 1. each value in rust as a value that as a owner
//2. they can only be one owner at a time 
// 3. when the owner goes out of scopes the value will be dropped 
// fn main() {
//     // s1 is the owner of ther value
//     let s1 = String::from("RUSTY");
//     // let transer ownership to s2
//     let _s2 = s1;
//     // i cannot longer use s1 cuz the value as been transferred to s2. so _s2 is the new owner
//    // println!("{}", s1);
//     println!("{}", _s2);
//     // refrencing s1 to len 
//     // let lenght = callen(&s1);
//     let lenght = callen(&_s2);
//     println!("the lenght of {} is {}", &_s2, lenght);


// }

// fn callen(len: &String) -> usize {
//     return len.len()
// };

//borrowing and reference
// refrence enable you to borrow values without taking ownership of the value
// we can create a reference by adding & to the variable
// refrence can be mutable and immutable 
// to make a refrence mutable we add  &mut to the variable we are referencing to and also add mut keyword to the orignal variable 
// fn main() {
//     let _x = 5;
//     let _r = &_x;
//     println!("_x: {}, _r {}", _x, _r)
// }


// mutable refrence
// fn main() {
//     let mut _x = 5;
//     let _r = &mut _x;

//     *_r += 1;
//     println!(" _r {}", _r)
// }

// struct
// 

// shawdowing 
// fn main() {
//     let x = 1;

//     println!("this is the value of x: {}", x);

//     let x = x + 1;

//     println!("this is the value of x shawdowed by it self: {}", x);

// }

//loop
// fn main() {
//   let mut counter = 0;

//   let result = loop {
//      counter+= 1;
//      println!("counter: {counter}");

//      if counter == 10 {
//         break
//         counter * 2;
//      }
//   };

//   println!("result: {result}")




// }

// while 
// fn main() {
//     let mut counter = 5;
//     while counter != 0 {
//          println!("{counter}");
//          counter -= 1;
      
//     }
// }

// for loop
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for numbers in a  {
        println!("{numbers}");
    }
}