
use library::utils::input::Input;

fn f(x: u64) -> u64 {
    if x == 0 {
        return 3;
    }
    3u64.pow(x as u32 +1) + x * 3u64.pow(x as u32-1)
}


fn solve(ip: &mut Input) {
    let (n, k) = ip.pair::<u64,u64>();
    
    let mut v = vec![];
    let mut cur = n;
    while cur > 0 {
        v.push(cur % 3);
        cur /= 3;
    }
    
    let mut sum = v.iter().sum::<u64>();
    if sum > k {
        println!("-1");
        return;
    }
    
    
    for i in (1..v.len()).rev() {
        let mut ok = 0;
        let mut ng = v[i]+1;
        
        while ng-ok>1 {
            let mid = (ok+ng)>>1;
            
            if sum + 2*mid <= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        
        sum += 2 * ok;
        v[i] -= ok;
        v[i-1] += 3*ok; 
    }
     
    let ans = v.iter()
        .enumerate()
        .map(|(i, vi)| f(i as u64) * *vi) 
        .sum::<u64>();

    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
