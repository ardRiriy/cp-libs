use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: i64,
        a: [i64; n],
    }
    let mut bit = FenwickTree::new(n, 0);
    let mut ai_list = vec![vec![]; m as usize];
    for i in 0..n {
        bit.add(i, 1);
        ai_list[a[i] as usize].push(i);
    }
    let mut cur = 0i64;
    for (i, _) in a.iter().enumerate().sorted_by_key(|(_, &ai)| ai) {
        bit.add(i, -1);
        cur += bit.sum(0..i);
    }

    println!("{}", cur);

    for i in (1..m as usize).rev() {
        for &pi in ai_list[i].iter() {
            cur += pi as i64;
            cur -= n as i64 - pi as i64 - 1;
        }
        println!("{}", cur);
    }
}

