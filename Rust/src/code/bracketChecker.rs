/*
Checks if brackets, parantheses etc are matched.
Uses a stack
*/
use crate::code::stack;
pub fn check_parens(s: &str) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch), // Push opening brackets onto the stack
            ')' => {
                if stack.pop() != Some('(') {
                    return false; // Mismatch or stack underflow
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false; // Mismatch or stack underflow
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false; // Mismatch or stack underflow
                }
            }
            _ => {} // Ignore non-bracket characters
        }
    }

    stack.is_empty() // If stack is empty, all brackets matched
}

