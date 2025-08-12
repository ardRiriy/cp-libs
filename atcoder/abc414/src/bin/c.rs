use proconio::input;

static M : usize = 1000001;
fn main() {
    input!{
        a: usize,
        n: usize,
    }

    let mut v = vec![];
    for i in 1..=n.min(M) {
        let mut k = i;

        let mut vv = vec![];
        while k > 0 {
            vv.push(k%10);
            k /= 10;
        }

        let mut k1 = i;
        for x in vv.iter() {
            k1 *= 10;
            k1 += x;
        }
        let res1 = k1.to_string();
        if k1 <= n && res1.chars().eq(res1.chars().rev()) {
            v.push(k1);
        }

        let mut k1 = i;
        for x in vv[1..].iter() {
            k1 *= 10;
            k1 += x;
        }
        let res1 = k1.to_string();
        if k1 <= n && res1.chars().eq(res1.chars().rev()) {
            v.push(k1);
        }
    }

    // /dbg!(&v);

    let ans = v.iter()
        .filter(|x| {
            let mut k = **x;
            let mut v = vec![];
            while k > 0 {
                v.push(k%a);
                k /= a;
            }
            v.iter().eq(v.iter().rev())
        })
        .sum::<usize>();
    println!("{}", ans);
}

