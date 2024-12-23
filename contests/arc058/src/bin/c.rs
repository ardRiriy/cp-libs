use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        d: [char; k],
    }
    
    let check = |x: usize| -> bool {
        let s = x.to_string();
        d.iter().all(|&c| !s.contains(c))
    };
    
    for i in n.. {
        if check(i) {
            println!("{}", i);
            return;
        }
    }
}

