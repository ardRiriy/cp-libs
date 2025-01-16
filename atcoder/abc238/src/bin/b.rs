use itertools::Itertools;
use proconio::input;

fn main(){
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut cur=0;
    let mut v = vec![cur];
    for ai in a{
        cur+=ai;
        v.push(cur%360);
    }
    v.sort();
    v.push(360);
    println!("{}",v.iter().tuple_windows().map(|(&a, &b)|(a-b).abs()).max().unwrap());

}