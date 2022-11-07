#![allow(unused)]


use core::num;
use std::io;
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
    // numbers_out_put();
    // greeting_main();
    numbers();

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
    print!("f64 : {}", num_2 + 0.111111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    print!("5 + 4 = {}\n", num_3 + num_4);
    print!("5 - 4 = {}\n", num_3 - num_4);
    print!("5 * 4 = {}\n", num_3 * num_4);
    print!("5 / 4 = {}\n", num_3 / num_4);
    print!("5 % 4 = {}\n", num_3 % num_4);
} // end of numbers

 