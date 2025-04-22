/*
From https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-01-box.html
Box<T> Points to Data on the Heap and Has a Known Size
The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. Refer to Chapter 4 to review the difference between the stack and the heap.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that needs to know an exact size
When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
When you want to own a value and only care that it’s a type that implements a particular trait rather than knowing the concrete type
*/
#[derive(Debug)]
enum List {
    // Cons(i32, List), // we can't use a List here since Rust will not be able to determine the recursion depth
    // Surround it with a Box to fix this.
    Cons(i32, Box<List>),
    Nil
}
use List::{Cons, Nil};
fn main() {
    // Store and access a value from the heap.
    let b = Box::new(5);
    println!("{}",b);

    // Boxes also enable recursive types. Boxes have a fixed size, so they're useful in cases such as recursion.
    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. 
    // cons list
    let cons_list = Cons(1,Box::new(Cons(2,Box::new(Nil))) );
    println!("{:?}", cons_list);


    let x = 5;
    let y = Box::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}