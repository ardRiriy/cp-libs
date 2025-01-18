use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars
    }
    let cs = x
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as i32)
        .rev()
        .collect_vec();
    let mut imos = vec![0; cs.len()+1];
    for (i, xi) in cs.iter().enumerate() {
        imos[0]+=xi;
        imos[i+1]-=xi;
    }
    {
        // flush
        let mut cur = 0;
        for i in 0..imos.len() {
            cur+=imos[i];
            imos[i]=cur;
        }
    }
    let mut ans = vec![];
    let mut kuriagari = 0;
    for xi in imos {
        kuriagari += xi;
        ans.push(kuriagari%10);
        kuriagari/=10;
    }
    while kuriagari!=0 {
        ans.push(kuriagari%10);
        kuriagari/=10;
    }
    while ans.len()>0&&*ans.last().unwrap()==0{
        ans.pop();
    }
    println!("{}", ans.iter().rev().join(""));
}
