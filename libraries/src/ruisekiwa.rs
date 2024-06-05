use std::ops::Range;
use num_traits::Num;

struct CumulativeSum<T> {
    sum: Vec<T>
}

impl<T> CumulativeSum<T> where T: Num + Clone {
    fn new(init_vec: &Vec<T>) -> CumulativeSum<T> {
        let size = init_vec.len();

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
    fn get(&self, range: Range<usize>) -> T {
        self.sum[range.end].clone() - self.sum[range.start].clone()
    }
}

// mod tests {
//     use crate::CumulativeSum;
//     fn cumulative_sum_get() {
//         let v = vec![1, 3, 8];
//         let cs = CumulativeSum::new(&v);
//         assert_eq!(1 + 3 + 8, cs.get(0..3));
//         assert_eq!(1 + 3, cs.get(0..2));
//         assert_eq!(3 + 8, cs.get(1..3));
//         assert_eq!(3, cs.get(1..2));
//     }
// }