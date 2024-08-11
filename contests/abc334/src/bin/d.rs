use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        q: usize,
        mut r: [u64;n],
        x: [u64; q]
    }

    r.sort_unstable();
    let cs = CumulativeSum::new(&r);
    println!("{}", x.iter()
            .map(
                |qi| match cs.sum.binary_search(qi) {
                    Ok(i) => i,
                    Err(i) => i-1,
                }
            )
            .join("\n")
    );
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
#[derive(Debug)]
pub struct CumulativeSum<T> {
    sum: Vec<T>
}

impl<T> CumulativeSum<T> where T: num_traits::Num + Clone {
    pub fn new(init_vec: &Vec<T>) -> CumulativeSum<T> {
        let sum = init_vec
            .iter()
            .enumerate()
            .fold(vec![T::zero()], |mut sum, (idx, x)| {
                let next :T = sum[idx].clone() + x.clone();
                sum.push(next);
                sum
            });
        CumulativeSum { sum }
    }

    // 区間[l, r)の総和を答える
    pub fn get<R>(&self, range: R) -> T
    where
        R: std::ops::RangeBounds<usize>
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.sum.len(),
        };

        self.sum[end].clone() - self.sum[start].clone()
    }
}

mod tests {
    #[test]
    fn cumulative_sum_get() {
        use super::CumulativeSum;
        let v = vec![1, 3, 8];
        let cs = CumulativeSum::new(&v);
        assert_eq!(1 + 3 + 8, cs.get(0..3));
        assert_eq!(1 + 3, cs.get(0..2));
        assert_eq!(3 + 8, cs.get(1..3));
        assert_eq!(3, cs.get(1..2));
        assert_eq!(1 + 3 + 8, cs.get(0..=2));
    }
}
