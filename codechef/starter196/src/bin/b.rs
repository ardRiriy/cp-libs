use library::utils::{chlibs::ChLibs, input::Input, iterlibs::dedup::RleItertor};

fn solve(input: &mut Input) {
    let n = input.next::<usize>();
    let s = input.next::<String>().chars().collect::<Vec<char>>();
    if n <= 3 {
        println!("{}", n);
        return;
    }
    let rle = s.iter().rle().collect::<Vec<_>>();
    let mut ans = (rle.iter().max_by_key(|(x, _)| *x).unwrap().0 + 1).min(n).max(3);
    for (i, &(len, _)) in rle.iter().enumerate() {
        if len==1 {
            let l = if i>0 { rle[i-1].0 } else { 0 };
            let r = if i+1<rle.len() { rle[i+1].0 } else { 0 };
            ans.chmax(l+r+len);
        }
    }
    
    let is_good4 = |v: &[char]| {
        v == &['0','0','1','1'] ||
        v == &['1','1','0','0'] ||
        v == &['1','0','0','1'] ||
        v == &['0','1','1','0']
    };
    if s.windows(4).any(|w| !is_good4(w)) {
        ans.chmax(4);
    }

    println!("{}", ans);
}

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}
