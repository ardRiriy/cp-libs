use library::utils::input::Input;

fn main() {
    let mut input = Input::new();
    let (q, k) = input.pair::<usize,usize>();

    if q>=6 {
        if k<5 {
            println!("No");
            return;
        } else {
            println!("Yes");
            println!("1 2\n2 3\n1 4\n1 5\n1 5");
            for _ in 0..q-5 {
                println!("6 7")
            }
        }
    } else {
        if q==1 {
            if k>=2 {
                println!("Yes");
                println!("1 2");
            } else {
                println!("No");
            }
        } else if q==2 {
            if k>=3 {
                println!("Yes");
                println!("1 2");
                println!("2 3");
            } else {
                println!("No");
            }
        } else if q==3 {
            if k >= 3 {
                println!("Yes");
                println!("1 2");
                println!("2 3");
                println!("1 4");
            } else {
                println!("No");
            }
        } else if q==4 {
            if k >= 4 {
                println!("Yes");
                println!("1 2");
                println!("2 3");
                println!("1 4");
                println!("3 5");
            } else {
                println!("No");
            }
        } else if q==5 {
            if k >= 4 {
                println!("Yes");
                println!("1 2");
                println!("2 3");
                println!("1 4");
                println!("3 5");
                println!("4 6");
            } else {
                println!("No");
            }
        }
    }

}

