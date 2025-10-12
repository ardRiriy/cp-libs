use std::{collections::BTreeSet, iter::repeat};

use library::utils::{chlibs::ChLibs, input::Input, iterlibs::{collect::CollectIter, strs::StrUtilIter}};

#[allow(dead_code)]
fn culc(a: &[usize]) -> Vec<usize> {
    let mut cur = 0;

    let mut s = vec![];

    for ai in a.iter() {
        cur.chmax(*ai);
        s.push(cur);
    }
    
    let mut t = vec![];
    cur = 1<<60;
    for ai in a.iter().rev() {
        cur.chmin(*ai);
        t.push(cur);
    }
    let mut cnt = vec![0; a.len()];
    for i in 0..a.len() {
        cnt[s[i]-1] += 1;
        cnt[t[i]-1] += 1;
    }
    return cnt;

}

#[allow(dead_code)]
fn checker(a: &[usize], f: &[usize]) {
    let cnt = culc(a);
    assert_eq!(f, cnt, "given f=[{}]\noutput A = [{}]\nf calculated from output A = [{}]\n", f.iter().join(" "), a.iter().join(" "), cnt.iter().join(" "));
}

#[allow(dead_code)]
fn checker2(f: &[usize]) {
    fn rec(n: usize, cur: &mut Vec<usize>, ret: &mut BTreeSet<Vec<usize>>) {
        if cur.len() == n {
            ret.insert(cur.clone());
            return;
        }

        for i in 1..=n {
            cur.push(i);    
            rec(n, cur, ret);
            cur.pop();
        }
    }
    let mut v =  BTreeSet::new();
    rec(f.len(), &mut vec![], &mut v);

    for v in v.iter() {
        assert!(culc(&v) != f, 
            "There is valid answer: {} \nfor given f=[{}]",
            v.iter().join(" "),
            f.iter().join(" "),
        );
    }
}

fn solve(ip: &mut Input) {
    let n = ip.next();
    let mut a = ip.vector::<usize>(n);
    if n == 1 {
        println!("1");
        return;
    }
    a[0] -= n;

    if a.iter().filter(|ai| ai>&&0).last().unwrap() == &1 {
        println!("-1");
        // checker2(&a);
        return;
    }

    let mut ans = vec![];
    for (i, ai) in a.iter().enumerate() {
        ans.append(&mut repeat(i+1).take(*ai).collect_vec());
    }
    ans[n-1] = 1;
    println!("{}", ans.iter().join(" "));
    
    // debug
    // a[0] += n;
    // checker(&ans, &a);
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
