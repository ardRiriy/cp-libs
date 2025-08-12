
use library::{data_structure::multiset::MultiSet, utils::input::Input};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair::<usize, usize>();
    let a = ip.vector::<u64>(n);
    let b = ip.vector::<u64>(m);

    let mut mset = MultiSet::new();
    for ai in a {
        mset.add(ai, 1);
    }

    for bi in b {
        mset.remove(bi, 1);
    }

    let ans = mset
        .iter()
        .map(|(k, v)| {
            std::iter::repeat_n(*k, *v)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
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
