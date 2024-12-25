use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.insert(0, '/');
    if *s.last().unwrap() == '_' {
        s.push('/');
    }

    let mut ans = vec![];

    for i in 0..s.len() {
        if s[i] == '/' {
        } else if s[i] != '_' {
            ans.push(s[i]);
        } else {
            let c = match s[i-1] {
                '0' => {
                    match s[i+1] {
                        '0' | '(' => '+',
                        '/' | ')' | '+' => ')',
                        _ => { unreachable!() }
                    }
                },
                '+' => {
                    match s[i+1] {
                        '+' | ')' | '/' => '0',
                        '(' | '0' => '(',
                        _ => { unreachable!() }
                    }
                },
                '(' => {
                    match s[i+1] {
                        '0' | '(' => '(',
                        '+' | ')' => '0',
                        _ => unreachable!()
                    }
                },
                ')' => {
                    match s[i+1] {
                        '0' | '(' => '+',
                        ')' | '+' | '/' => ')',
                        _ => unreachable!()
                    }
                },
                '/' => {
                    match s[i+1] {
                        '0' | '(' => '(',
                        '+' | '/' => '0',
                        _ => unreachable!()
                    }
                }
                _ => { unreachable!(); }
            };
            ans.push(c);
        }
    }

    println!("{}", ans.iter().collect::<String>());
} 