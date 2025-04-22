#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
fn collatz(mut n: u32) -> u32 {
    let mut len = 1;
    while n > 1 {
        match n%2==0 {
            true => n = n/2 ,
            false => n = 3*n +1,
        };
        len+=1;
    }
    len
}

fn transpose(matrix: [[i32;3]; 3]) -> [[i32;3];3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }
    result
}

fn magnitude(v: &Vec<f32>) -> f32 {
    let mut res:f32 = 0.0;
    for coord in v {
        res += coord*coord;
    }
    res.sqrt()
}

fn normalize_inplace1(v: &mut Vec<f32>) {
    let mag = magnitude(v);
    for coord in v.iter_mut() {
        *coord /= mag;
    }
}

fn normalize_inplace2(v: &mut Vec<f32>) {
    // change in-place - saves memory.
    let mag = magnitude(v);
    *v = (*v).iter().map(|x| x/mag).collect();
}

/*
fn normalize_inplace(v: &Vec<f32>) -> Vec<f32> {
    // change in-place - saves memory.
    let mag = magnitude(v);
    let new = v.iter().map(|x| x/mag).collect();
    new
}
*/



fn main() {
    /*
    let a = 'A';
    let b = 'B';
    let mut r = &a; // this is called a mutable reference
    println!("{}",*r);
    r = &b;
    println!("{}", r); // Rust derefences automagically

    let arr = [[1,2,3], [4,5,6], [7,8,9]];
    let sleis = &arr[1..=2]; // automatic borrow not recommended here
    let tup: (i32, bool) = (7, true);
    println!("2nd element of {arr:?} is {:?}", &arr[1]);
    println!("Transposed: {:?}", transpose(arr));
    println!("Collatz of 20 is: {}", {collatz(20)});

    let string1: &str = "world";
    let mut string2: String = String::from("Hello "); // .to_string() also works;
    string2.push_str(string1); // Rust adds &
    println!("{}", &string2[2..=6]); // & compulsory here for some reason
    */
    let mut mahvec = vec![1.0, 2.0, 3.0, 4.0];
    println!("{:?}", normalize_inplace1(&mut mahvec));
    normalize_inplace1(&mut mahvec); 
    println!("{:?}", mahvec);
    normalize_inplace2(&mut mahvec); 
    println!("{:?}", mahvec);
    // modify the normalize fn to take a pointer/ref to a vector (rather than a vector)
    

}