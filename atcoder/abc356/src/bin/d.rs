use ac_library::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    println!("{}", dfs(2u64.pow(62u32) as usize, 62 ,n + 1, m));
}

fn dfs(t: usize, turn: usize ,n: usize, m: usize) -> ModInt998244353 {
    let mut res = ModInt998244353::new(0);

    if m >> turn & 1 == 1 {
        let nt = ModInt998244353::new(n / (2 * t)) * t;
        res += nt;
        res += if n % (2 * t) <= t {
            0
        } else {
            n % (2 * t) - t
        };
    }

    if turn == 0 {
        return res;
    }

    res + dfs(t / 2, turn - 1 ,n, m)
}
