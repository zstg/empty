#![allow(unused)]
fn magnitude(v: &Vec<f32>) -> f32 {
    let mut res:f32 = 0.0;
    for coord in v {
        res += coord*coord;
    }
    res.sqrt()
}

fn normalize_inplace_ptr(v: &mut Vec<f32>) {
    let mag = magnitude(v);
    for coord in v.iter_mut() {
        *coord /= mag;
    }
}

fn normalize_inplace_map(v: &mut Vec<f32>) {
    // change in-place - saves memory.
    let mag = magnitude(v);
    *v = (*v).iter().map(|x| x/mag).collect();
}

fn normalize_notinplace(v: &Vec<f32>) -> Vec<f32> {
    // change in-place - saves memory. 
    // Obviously the reference to v doesn't have to be mutable since v isn't getting modified; the results are stored in a new vector.
    let mag = magnitude(v);
    let new = v.iter().map(|x| x/mag).collect();
    new
}


fn main() {
    let mut mahvec = vec![1.0, 2.0, 3.0, 4.0];
    normalize_inplace_ptr(&mut mahvec); 
    println!("Ptr: {:?}", mahvec);
    normalize_inplace_map(&mut mahvec); 
    println!("Map: {:?}", mahvec);  
    println!("Not in-place: {:?}", normalize_notinplace(&mahvec));
}