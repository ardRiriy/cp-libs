use std::collections::VecDeque;

use cps::linked_list::Item;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut items = (0..n)
        .map(|i| Item::new(Some(i)))
        .collect_vec();

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                items[x].update_next(Some(y));
                items[y].update_prev(Some(x));
            },
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                items[x].update_next(None);
                items[y].update_prev(None);
            },
            3 => {
                input! {
                    x: Usize1,
                }
                let mut que = VecDeque::new();
                que.push_front(x+1);
                // 前に戻る
                let mut now = x;
                while let Some(i) = items[now].previous() {
                    que.push_front(i+1);
                    now = i;
                }
                let mut now = x;
                while let Some(i) = items[now].next() {
                    que.push_back(i+1);
                    now = i;
                }

                println!("{} {}", que.len(), que.iter().join(" "));
            }
            _ => { unreachable!(); }
        }
    }
}

