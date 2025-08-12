use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let mut ok = 1;
    let mut ng = 1000001;

    println!("{}", ok);

    for _ in 0..25 {
        let mid = (ok+ng)>>1;
        println!("{}", mid);
        let res = input.next::<u8>();
        if res == 1 {
            if ok <= mid {
                ok = mid;
            } else {
                ng = mid;
            }
        } else {
            if ok < mid {
                ng = mid;
            } else {
                ok = mid;
            }
        }
    }

}
