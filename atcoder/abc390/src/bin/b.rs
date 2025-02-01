use itertools::Itertools;
use num_rational::Ratio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    println!(
        "{}",
        if a.iter()
            .tuple_windows()
            .map(|(&a, &b)| Ratio::new(a, b))
            .all_equal()
        {
            "Yes"
        } else {
            "No"
        }
    );
}
