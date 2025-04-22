#![allow(unused)]
#![allow(non_snake_case)]

mod code; // import the other files in cwd
use crate::code::*;
use crate::stack::Stack;
#[derive(Debug)]
enum Direction{Left,Right}

// create an alias dir for Direction
type Dir = Direction;

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    /*
    let dir = Dir::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: \{player_move:?\}");
    let daeg = Dog(String::from("Cheems"));
    println!("{:#?}", daeg);
    */

    let mut mystk: Stack<i32> = Stack::new();
    println!("{}", crate::longestvalidParantheses::longest_valid("(())"));
    println!("{}", crate::bracketChecker::check_parens("[}"));
}