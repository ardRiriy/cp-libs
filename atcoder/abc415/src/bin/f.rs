use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};
use ac_library::{Monoid, Segtree};
/* 
うーん与えられた区間s[l..r]の連続部分文字列で、文字種がすべて同じであるようなl,rであって、r-lが最大になるものを求める
これをオンラインで(1点変更クエリあり)
そんなのできるんですか？

区間MAXをとるTreapを色々頑張るとできそうですね？私は持っていません

区間の状態が変わるのは変わった文字列の左右ぐらいでめっちゃ影響が少ない(高々2,3区間？) これを利用したい気持ち

えー、わかりません

区間数は多くてもN個かぁ なんか、iから開始する区間の長さを持つみたいなのでできない？
端点は適当に処理することにすれば、これはrange-maxをとるだけでよい

端点の処理は？
=> とるべきl,rをいい感じにしてあげればいいですね？
あとは: 更新をどうするか？

端点が微妙にずれるみたいなのは適当に処理できるけど、区間をぶった切るような更新はどうするか？
それより左の非ゼロ位置を持っておく(O(log N))と、そこが「ぶった切る」かどうかはわかって、ホニャホニャするとできそう？

*/

struct RangeMax;

impl Monoid for RangeMax {
    type S = u64;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

fn main() {
    input!{
        n: usize,
        s: String,
        q: usize,
    }
    let mut start_pos = BTreeSet::new();
    let mut end_pos = BTreeSet::new();
    let mut seg: Segtree<RangeMax> = Segtree::new(n);
    let mut prev = '-';
    let mut cnt = 0;
    for (i, c) in s.chars().enumerate() {
        if c==prev {
            cnt+=1;
        } else {
            if cnt > 0 {
                start_pos.insert(i - cnt);
                end_pos.insert(i-1);
                seg.set(i-cnt, cnt as u64);
            }
            prev = c;
            cnt = 1;
        }
    }
    start_pos.insert(n - cnt);
    end_pos.insert(n-1);
    seg.set(n - cnt, cnt as u64);

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                c: char,
            }
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }



        }
    }
}

