fn main() {
    println!("Hello, world!");
    println!("{} ", add(10, 20));
    let s:&str = "Hello, I'm LiaoZhengqiang";
    println!("{}", s);
    let mut s1 = String::from("Hello ");
    s1.push_str("World");
    println!("{}", s1);
    println!("{}",sub(15, 20));
    // let mut arr: = [0, 2, 4, 6]; // Error , Please use std::fmt::Display
    // println!("{}", arr);
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn fib(n: i32) -> i32 {
    if n <= 0 { 0 } else { fib(n-1) + fib(n-2) }
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}