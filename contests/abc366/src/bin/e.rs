use itertools::Itertools;
use proconio::{input};
fn solve() {
    input!{
        n: usize,
        d: i64,
        point: [(i64, i64); n]
    }
    let point = point
        .iter()
        .map(|&(x, y)| (x + y, x - y))
        .collect_vec();
    let u_p = point.iter()
        .map(|(u, _)| *u)
        .sorted()
        .collect_vec();
    let v_p = point.iter()
        .map(|(_, v)| *v)
        .sorted()
        .collect_vec();

    let temp = u_p[0];
    let u_p_base_s = u_p.iter()
        .map(|x| *x - temp)
        .sum::<i64>();
    let temp = v_p[0];
    let v_p_base_s = v_p.iter()
        .map(|x| *x - temp)
        .sum::<i64>();

    let max = 2 * 1e6 as usize * 2;
    let mut dist_u = vec![INF as i64; max];
    let mut cnt = 0;
    let mut adder = 0;
    for i in u_p[0]..max as i64 {
        let idx = i + max as i64 / 2;
        dist_u[idx as usize] = u_p_base_s + adder;
        while u_p[cnt] == i {
            cnt += 1;
        }
        adder += cnt as i64;
        adder -= n as i64 - cnt as i64;
    }
    let mut dist_v = vec![INF as i64; max];
    for i in v_p[0]..max as i64 {
        let idx = i + max as i64 / 2;
        dist_v[idx as usize] = v_p_base_s + adder;
        while v_p[cnt] == i {
            cnt += 1;
        }
        adder += cnt as i64;
        adder -= n as i64 - cnt as i64;
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
