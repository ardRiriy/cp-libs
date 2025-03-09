use proconio::input;

fn main() {
    input!{
        n: usize,
        c: u32,
        ops: [(u8, u32); n],
    }

    let mut zero = 0;
    let mut one = !0;
    let mut cur = c;

    for (t,  val) in ops {
        // zero, oneに対して操作を行う
        if t == 1 {
            zero &= val;
            one &= val;
        } else if t == 2 {
            zero |= val;
            one |= val;
        } else {
            zero ^= val;
            one ^= val;
        }
        let mut ans = 0;
        for i in 0..=30 {
            if (cur >> i) & 1 == 0 {
                ans |= (1 << i) & zero;
            } else {
                ans |= (1 << i) & one;
            }
        }
        println!("{ans}");
        cur = ans;
    }
}

