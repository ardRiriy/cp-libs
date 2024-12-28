use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; m],
        x: [[u64; m]; n]
    }

    let sum = x.iter().fold(vec![0u64; m], |mut acc, v|
        {
            for (idx, &x) in v.iter().enumerate() {
                acc[idx] += x;
            }
            acc
        }
    );

    println!("{}", if sum.iter().enumerate().all(|(idx, &x)| x >= a[idx]) {
        "Yes"
    } else {
        "No"
    });
}
