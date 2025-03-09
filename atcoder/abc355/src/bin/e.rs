use std::io::{stdin, BufReader, StdinLock};

use proconio::{input, source::line::LineSource};

fn solve(n: usize, offset: usize, l: usize, r: usize, mut source: &mut LineSource<BufReader<StdinLock<'static>>>) -> u64 {
    if n == 0 {
        return 0;
    }
    let big_l = n * offset;
    let big_r = n * (offset+1);

    if big_l == l && big_r == r {
        // 質問クエリ
        let mut k = r - l;
        let mut cnt = 0;
        while k != 1 {
            cnt += 1;
            k /= 2;
        }
        eprintln!("=========Q=========");
        dbg!(l, r);
        eprintln!("===================");
        println!("? {cnt} {offset}"); 
        input! {
            from &mut source,
            res: u64,
        }
        return res;
    }


    let cnt1 = cnt(n, offset, l, r);
    let cnt2 = cnt(n, offset, n*offset, n * (offset+1)) 
        + if l == n*offset { 0 } else { cnt(n, offset, n*offset, l) }
        + if r == n*(offset+1) { 0 } else { cnt(n, offset, r, n*(offset+1)) };
    let mut res = 0;
    if cnt1 <= cnt2 {
        let m = big_l + n / 2;
        if l < m {
            res += solve(n/2, offset*2, l, m.min(r), source);
        }
        if m < r {
            res += solve(n/2, offset*2+1, m.max(l), r, source);
        }
    } else {
        dbg!(n, l, r);
        dbg!(offset);
        res += solve(n, offset, n*offset, n * (offset+1), source);
        res += 200 - (
            if l == n*offset { 0 } else { solve(n, offset, n*offset, l, source) } +
            if r == n*(offset+1) { 0 } else { solve(n, offset, r, n*(offset+1), source) }
        );
    }
    res % 100
}

fn cnt(n: usize, offset: usize, l: usize, r: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    let big_l = n * offset;
    let big_r = n * (offset+1);

    if big_l == l && big_r == r {
        // 質問クエリ
        let mut k = r - l;
        let mut cnt = 0;
        while k != 1 {
            cnt += 1;
            k /= 2;
        }
        return 1;
    }

    let mut res = 0;
    let m = big_l + n / 2;
    if l < m {
        res += cnt(n/2, offset*2, l, m.min(r));
    }
    if m < r {
        res += cnt(n/2, offset*2+1, m.max(l), r);
    }
    res
}
fn main() { 
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: u32,
        l: usize,
        r: usize,
    }
    

    let ans = solve(2usize.pow(n), 0, l, r+1, &mut source);
    println!("! {}", ans%100);

}