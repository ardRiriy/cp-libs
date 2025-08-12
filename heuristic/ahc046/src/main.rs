use std::collections::{BinaryHeap, VecDeque};

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use proconio::input;
use utils::utils::{get_time, DC, DI, DJ};

use crate::simulated_annealing::simulated_annealing::AnnealingState;

mod data_structure;
mod simulated_annealing;
mod utils;

static TIME_LIMIT :f64 = 1.995;
static INF :usize= 1<<30;
static N: usize = 20;
static M: usize = 40;

struct Tour {
    pos_type: u8, // 1: check point, 2: placement
    pos: (usize, usize),
    dir: usize, // only pos_type == 2
}

impl Tour {
    fn new(pos_type: u8, pos: (usize, usize), dir: usize) -> Self {
        Tour { pos_type, pos, dir }
    }

    fn is_check_point(&self) -> bool {
        self.pos_type == 1
    }
}

// sa
struct SimulatedAnnealingState {
    // 状態を保持するフィールド
    current_score: i32,
    tour: Vec<Tour>,
}

impl AnnealingState<i32> for SimulatedAnnealingState {
    fn pre_process(&mut self) {
        todo!()
    }

    fn eval(&self) -> Option<i32> {
        todo!()
    }

    fn rollback(&mut self) {
        todo!()
    }

    fn post_process(&mut self, new_score:i32) {
        todo!()
    }

    fn current_score(&self) -> i32 {
        todo!()
    }
}

fn main() {
    get_time();
    input! {
        _: usize,
        _: usize,
        p: [(usize, usize); M],
    }

    let mut tour = p.iter()
        .map(|&(i,j)| Tour::new(1, (i, j), 0))
        .collect_vec();

}