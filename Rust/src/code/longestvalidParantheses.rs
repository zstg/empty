use crate::stack;
/*
    "(()" -> 2
    ")()())" -> 4
*/

pub fn longest_valid(s: &str) -> i8 {
    let mut stack = Vec::new();
    let mut len=0;
    for ch in s.chars() {
        match ch {
            '(' => stack.push('('),
            ')' => {
                if stack.pop().is_none() {
                    return 0;
                }
                len+=1;
            },
            _ => {}
        }
    }
    len
}