use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        mut a: [u64;n]
    }

    let v = a.iter()
        .sorted()
        .dedup()
        .tuple_windows()
        .map(|(ai,aj)| {
            aj-ai
        })
        .sorted()
        .collect_vec();
    if v.len() < m { 
        println!("0");
    } else {
        println!("{}", v[..=v.len()-m].iter().sum::<u64>());    
    };
}

