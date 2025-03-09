use std::collections::BTreeMap;
use ac_library::ModInt1000000007 as Mint;
use proconio::input;

// 総和がsになるよう数列の個数
fn rec(s: u64, memo: &mut BTreeMap<u64, Mint>) -> Mint {
    if s < 3 {
        return Mint::new(0);
    } else if s == 3 {
        return Mint::new(1);
    } else if let Some(val) = memo.get(&s) {
        return *val;
    }

    let mut res = Mint::new(1);

    for i in 3..=s {
        res += rec(s-i, memo);
    }

    memo.insert(s, res);
    res
}

fn main() {
    input!{
        s: u64,
    }
    println!("{}", rec(s, &mut BTreeMap::new()));
}

