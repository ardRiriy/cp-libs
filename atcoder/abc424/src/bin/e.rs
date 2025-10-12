use std::collections::BinaryHeap;

use library::utils::input::Input;

# [derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn solve(ip: &mut Input) {

    let (n, k, x) = ip.triple();
    let a = ip.vector::<f64>(n);
    
    let mut pq  = BinaryHeap::new();
    for &ai in a.iter() {
        pq.push((Total(ai), 1));
    }
    
    let mut left = k;
    while let Some((Total(l), cnt)) = pq.pop() {
        if left > cnt {
            left -= cnt;
            pq.push((Total(l/2.), cnt*2));
        } else {
            pq.push((Total(l/2.), left*2));
            pq.push((Total(l), cnt-left));
            break;
        }
    }
    
    let mut left = x;
    while let Some((Total(val), cnt)) = pq.pop() {
        if left > cnt {
            left -= cnt;
        } else {
            println!("{}", val);
            return;
        }
    }
    
    
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
