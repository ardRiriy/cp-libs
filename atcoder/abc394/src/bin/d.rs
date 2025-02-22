#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars,
    }
    let mut stk = vec![];
    for &c in s.iter() {
        if c == '(' || c == '[' || c == '<' {
            stk.push(c);
        } else {
            if let Some(pair) = stk.pop() {
                match c {
                    ']' => {
                        if pair != '[' {
                            println!("No");
                            return;
                        }
                    }
                    ')' => {
                        if pair != '(' {
                            println!("No");
                            return;
                        }
                    }
                    '>' => {
                        if pair != '<' {
                            println!("No");
                            return;
                        }
                    }
                    _ => unreachable!()
                }
            } else {
                println!("No");
                return;
            } 
        }
    }
    println!("{}", if stk.is_empty() { "Yes" } else { "No" });
}

