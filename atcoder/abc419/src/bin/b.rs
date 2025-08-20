
use library::{data_structure::multiset::MultiSet, utils::input::Input};

fn solve(ip: &mut Input) {
    let mut set = MultiSet::new();
    let q = ip.next::<usize>();
    
    for _ in 0..q {
        let t = ip.next::<usize>();
        if t == 1 {
            let x = ip.next::<u64>();
            set.add(x, 1);
        } else {
            if let Some((&k, &_)) = set.min_key() {
                set.remove(k, 1);
                println!("{}", k);
            }
        }
    }

}

fn main() {
    static IS_MULTI_TEST_CASE :bool = false;
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
