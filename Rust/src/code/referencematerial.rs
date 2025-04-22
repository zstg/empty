#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let s1: String = String::from("Hello!");
    let s2  = &s1;
    let s3 = &s2;
    println!("{} {} {}", s1, s2, s3);
}