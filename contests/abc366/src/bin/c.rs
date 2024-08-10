use std::collections::{BTreeMap, BTreeSet};

use proconio::{input};
fn solve() {
    input!{
        q: usize,
    }

    let mut cnt = BTreeMap::new();
    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
            t: u32,
        }

        match t {
            1 => {
                input! {
                    x: u32,
                }
                set.insert(x);
                *cnt.entry(x).or_insert(0u64) += 1;
            },
            2 => {
                input! {
                    x: u32,
                }
                if cnt[&x] == 1 {
                    cnt.insert(x, 0);
                    set.remove(&x);
                } else {
                    let val = cnt[&x];
                    cnt.insert(x, val - 1);
                }
            }
            3 => {
                println!("{}", set.len());
            },
            _ => { unreachable!(); }
        }
    }

}

/*
鹿のardririy, arDeeriy
　　　　　　　　　　　　 　 　 　 　 　 　 ／
　　　　　　　　　　　　　　　　　 　 　 //
　　　　　　　　　 　 　 　 　 　 　 　 //　 　 　 　 |:
　　　　　　　　　　　　　　　　　 　 // 　 　 　 　 .|i
　　　　　　　　　 　 　 　 　 　 　 //　 　 　 　 　 ||
　　　　　　　　　　　　　　　　　　l i　　　　　　 　 ||
　　　　　.　´￣￣｀ｰ　、　　　　 | l　 　 　 　 　 ∥
　　　 ／　 . 　 　 　_＞─- ､ 　 l |　　　　　　 .∥
　　　i　 〆　 　 ／　 　 　 　 ＼.i | 　 　 　 　 ∥
　　　| /　 　 ／ 　 　 　 　 　 　 l |　ﾊ　　　　.′
　 　 ; :　 　 .′　　　　　　　　　 ヾゝi !　　　/.′
　　 《　　　 :　　　 　 　 r─-､　　 i ﾘ:l　　 //　　　　 　 　 、
　 　 |　　　　 　 　 　 　 ｀ヾ.　＼ r‐’:ﾚ=‐' .ゝ...ノ＿＿＼＿))
　　 弋　 　 　 　 　 　 　 　 ∨ソ`ー''`ー---一　‐─‐-､)¨´
　　 　 } 、　　　　　　　　　　 i_..ノ _,　　　　ハ'
　　 　 ∨＞､　iゝ.. 　 　 　 　 　 弋);;,ゞ..ノ　.′
　　　　 ∨::::| 7!:::..ヾ.　　　　.′ヽ.　` 　 　 /
　　　　　∨::|.′::::::ixxr, 　 /　　　:　　　　 I.
　 　 　 　 }:::|..:ｉ..::::::|　i,,　　"'' ´ヾ.i 　 　 　 ﾊ
　　　　 　 ::::|:::|::::::/　 }ヾ.　　　　ﾐゝ.　 ,、..:::::)
　　　　 　 i:::|ヾ:::::i　 /.::::|｀ヾ..,,　..ノ＼ヾ..＿/
　 　 　 　 |:::l　}:. {　 |::::..′　　　　　　 `ー''
　 　 　 　 |:::| /.:/l　 !.:::l
　　　　　 ﾉ..:ﾚ.:::ｉ::l　ﾉ..::|
　　　 　 (人7.::::|:::V.::::::|
　　　　　`''/ .:::::!ｰ:::::::::ﾊ
　　　　　 /..::::::::| (_/＼__)
　　　 　 (__/(,＿)
*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
