use proconio::input;

fn main() {
    input! { 
        n: usize,
        k: usize,
    }
    
    let mut v = vec![0; n+1];

    for i in 2..=n {
        if v[i] != 0 {
            // 素数ではないので無視
            continue;
        }
        let mut j = i;
        while j <= n {
            v[j] += 1;
            j += i;
        }
    }

    println!("{}", v.iter().filter(|&x| *x >= k).count());
}
