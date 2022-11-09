#![allow(unused)]

use core::num;
use std::io;
use std::str::SplitWhitespace;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main(){
    // Unsigned integer : u8, u16, u32, u64, u128, usize
    // Signed integer : i8, i16, i32, i64, i128, isize
    
    // u32 unsign 32 bits integer 
    const ONE_MIL: u32 = 1_000_000;
    // float
    const PI: f32  = 3.141592;
    let age: &str = "47";
    // .trim get rids of white space
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
    numbers_out_put();
    // greeting_main();
    numbers();
    random_nums();
    birthday();
    vote();
    match_nums();
    match_age();
    arrays(); 
    while_loops();
    for_loops();
    tuple(); 
    mut_string();
    strings();
    casting();
    enums();
    vectors();
} // end of main

fn greeting_main(){
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    // to receive input from the user you're going to need to use readline
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
} // end of greeting_main

fn numbers_out_put(){
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
} // end of numbers_out_put

fn booleans(){
    let is_true: bool = true;
    let my_grade = 'A';
} // end of booleans

fn numbers(){
    let num_1: f32 = 1.111111111111111;
    print!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    print!("f64 : {}\n", num_2 + 0.111111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    print!("5 + 4 = {}\n", num_3 + num_4);
    print!("5 - 4 = {}\n", num_3 - num_4);
    print!("5 * 4 = {}\n", num_3 * num_4);
    print!("5 / 4 = {}\n", num_3 / num_4);
    print!("5 % 4 = {}\n", num_3 % num_4);
}// end of numbers

fn random_nums(){
    let random_num = rand::thread_rng().gen_range(1..101);
    print!("Random number is : {}\n", random_num);
}// end of random_nums

fn birthday(){
    let age: i32 = 8;
    if (age >= 1) && (age <= 18){
        print!("Important Birthday\n")
    } else if (age == 21) || (age == 50){
        print!("Important Birthday\n");
    } else if age >= 65 {
        print!("Important Birthday\n");
    } else {
        print!("Not an Important Birthday\n");
    }
}// end of birthday

fn vote() {
    let mut my_age = 47;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    print!("Can Vote : {}\n", can_vote);
}// end of vote

fn match_nums(){
    let age2: i32 = 8;
    match age2 {
        1..=18 => print!("Your age is {} Important Birthday! \n", age2),
        21 | 50 => print!("21 - 50 Important Birthday"),
        65..=i32::MAX => print!("Important Birthday"),
        _ => print!("Not an Important Birthday"),
    };
}// end of match_nums

fn match_age(){
    let my_age: i32 =18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => print!("Can't vote!"),
        Ordering::Greater => print!("Can vote!"),
        Ordering::Equal => print!("Your age is {}, you gained access to vote!\n", my_age),
    };
}// end of match_age

fn arrays(){
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
    // Loop
    let mut loop_idx =0;
    loop {
        if arr_1[loop_idx] % 2 == 0{
            loop_idx +=1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("Val : {} ", arr_1[loop_idx]);
        loop_idx +=1;
    }
}// end of arrays

fn while_loops(){
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx =0;
    while loop_idx < arr_1.len(){
        println!("Arr : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }
}// end of while_loops

fn for_loops(){
    let arr_1: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx =0;
    // you have to type 
    // for val in arr_1.iter()
    // it will format in this below 
    for val in arr_1.iter(){
        println!("Val : {}", val);
    }
}// end for_loops

fn tuple() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1); 
}// end of tuple 

fn mut_string(){
    // you have to type
    // let mut st1 = String::new();
    // it will format in this below
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("word\n");
    // for word in st1.split_whitespace()
    for word in st1.split_whitespace(){
        println!("{}\n", word);
    }
    let st2 = st1.replace("A", "Another ");
    println!("{}\n", st2);
} // end of mut_string
 
fn strings(){
    let st3: String = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    // dedup removes duplets
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from(" words ");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("st6 + st7 {}", char);
    }
}// end of strings 

fn casting(){
    let int_u8: u8 =5;
    let int2_u8: u8 =4;
    let int3_u32: u32 =(int_u8 as u32) + (int2_u8 as u32);
    println!("int3_u32 = {}", int3_u32);
}

fn enums(){
    enum Days{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }
    let today:Days = Days::Monday;
    match today {
        Days::Monday => println!("Everyone hates Monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost Weekend"),
        Days::Saturday => println!("Weekend"),
        Days::Sunday => println!("Weekend"),
    }
    println!("Is today the weekend? {}", today.is_weekend());
}// end of enums

fn vectors(){
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("2nd : {}", second), 
        None => println!("No 2nd value"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}
