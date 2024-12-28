use proconio::input;

fn rec(i: usize, v: &Vec<u32>) -> u32 {
    if v.len() == 1 {
        return 0;
    }

    let mut vec = vec![vec![]; 2];

    for x in v.iter() {
        vec[(*x as usize >> i) & 1].push(*x); 
    }

    let mut res;
    if i != 0 {
        if vec[0].len() > 0 && vec[1].len() > 0 {
            res = 1 << i;
            res += rec(i-1, &vec[0]).min(rec(i-1, &vec[1]));
        } else {
            if vec[0].len() == 0 {
                vec.swap(0, 1);
            }
            res = rec(i-1, &vec[0]);
        }
    } else {
        if vec[0].len() > 0 && vec[1].len() > 0 {
            return 1;
        } else {
            return 0;
        }
    }
   res
}

fn main() {
    input!{
        n: usize,
        v: [u32; n],
    }
    println!("{}", rec(30, &v));
}

