use std::{io::{stdin, stdout, BufReader, Write}, iter::{self, repeat}};

use itertools::Itertools;
use proconio::{input, source::{self, line::LineSource}};
use rand::{thread_rng, Rng};

use crate::{simulated_annealing::simulated_annealing::{AnnealingState, SimulatedAnnealing}, utils::utils::get_time};

mod data_structure;
mod simulated_annealing;
mod utils;

static TIME_LIMIT :f64 = 1.99;
static N :usize = 500;
static M :usize = 50;
static L :u64 = 1e15 as u64 -2e12 as u64;

struct CardAssignmentState {
    cards: Vec<u64>,
    targets: Vec<u64>,
    assignment: Vec<usize>,
    pile_sums: Vec<u64>,
    current_score: i64, // 現在のスコア（誤差の合計、小さいほど良い）

    // enumを使わずに近傍操作を管理
    // 0: No move, 1: Change, 2: Swap
    pending_move_type: u8,
    // For Change: (card_idx, old_pile, new_pile)
    // For Swap:   (card1_idx, card2_idx, unused)
    pending_move_params: (usize, usize, usize),
}

impl CardAssignmentState {
    fn new(cards: Vec<u64>, targets: Vec<u64>) -> Self {
        let m = targets.len();
        let n = cards.len();
        let mut assignment = vec![0; n];
        let mut pile_sums = vec![0; m + 1];

        // --- 初期割当: 貪欲法 ---
        let mut sorted_cards: Vec<(usize, u64)> = cards.iter().cloned().enumerate().collect();
        sorted_cards.sort_by_key(|&(_, v)| v);
        let mut used = vec![false; n];

        for (pile_idx, &target) in targets.iter().enumerate() {
            let mut best_card_idx = !0;
            let mut min_diff = u64::MAX;

            for &(card_idx, card_val) in &sorted_cards {
                if !used[card_idx] {
                    let diff = (card_val as i64 - target as i64).abs() as u64;
                    if diff < min_diff {
                        min_diff = diff;
                        best_card_idx = card_idx;
                    }
                }
            }
            if best_card_idx != !0 {
                 assignment[best_card_idx] = pile_idx + 1;
                 pile_sums[pile_idx + 1] += cards[best_card_idx];
                 used[best_card_idx] = true;
            }
        }
        
        // --- 初期スコア計算 ---
        let current_score = pile_sums[1..]
            .iter()
            .zip(targets.iter())
            .map(|(&sum, &target)| (sum as i64 - target as i64).abs())
            .sum();

        Self {
            cards,
            targets,
            assignment,
            current_score,
            pile_sums,
            pending_move_type: 0,
            pending_move_params: (0, 0, 0),
        }
    }

    // カード移動によるスコア改善量を計算 (正の値なら改善)
    fn calculate_change_delta(&self, card_idx: usize, old_pile: usize, new_pile: usize) -> i64 {
        let card_value = self.cards[card_idx];
        let mut delta: i64 = 0;

        if old_pile > 0 {
            let p_idx = old_pile - 1;
            let sum = self.pile_sums[old_pile];
            let target = self.targets[p_idx];
            let old_err = (sum as i64 - target as i64).abs();
            let new_err = ((sum - card_value) as i64 - target as i64).abs();
            delta += old_err - new_err;
        }

        if new_pile > 0 {
            let p_idx = new_pile - 1;
            let sum = self.pile_sums[new_pile];
            let target = self.targets[p_idx];
            let old_err = (sum as i64 - target as i64).abs();
            let new_err = ((sum + card_value) as i64 - target as i64).abs();
            delta += old_err - new_err;
        }
        delta
    }

    // カード交換によるスコア改善量を計算 (正の値なら改善)
    fn calculate_swap_delta(&self, card1_idx: usize, card2_idx: usize) -> i64 {
        let pile1 = self.assignment[card1_idx];
        let pile2 = self.assignment[card2_idx];
        let card1_val = self.cards[card1_idx];
        let card2_val = self.cards[card2_idx];
        let mut delta: i64 = 0;

        // 同じ山に所属するカード同士の交換は意味がない
        if pile1 == pile2 { return 0; }

        if pile1 > 0 {
            let p_idx = pile1 - 1;
            let sum = self.pile_sums[pile1];
            let target = self.targets[p_idx];
            let old_err = (sum as i64 - target as i64).abs();
            let new_err = ((sum - card1_val + card2_val) as i64 - target as i64).abs();
            delta += old_err - new_err;
        }

        if pile2 > 0 {
            let p_idx = pile2 - 1;
            let sum = self.pile_sums[pile2];
            let target = self.targets[p_idx];
            let old_err = (sum as i64 - target as i64).abs();
            let new_err = ((sum - card2_val + card1_val) as i64 - target as i64).abs();
            delta += old_err - new_err;
        }
        delta
    }
}

impl AnnealingState<i64> for CardAssignmentState {
    fn pre_process(&mut self) {
        let mut rng = thread_rng();
        let n = self.cards.len();
        let m = self.targets.len();
        self.pending_move_type = 0;

        let operation = rng.gen_range(0..2);
        match operation {
            0 => { // 操作1: カードを別の山に移動 (捨てる/拾うも含む)
                let card_idx = rng.gen_range(0..n);
                let old_pile = self.assignment[card_idx];
                let new_pile = rng.gen_range(0..=m);
                if old_pile != new_pile {
                    self.pending_move_type = 1;
                    self.pending_move_params = (card_idx, old_pile, new_pile);
                }
            }
            _ => { // 操作2: 2枚のカードを交換
                let card1_idx = rng.gen_range(0..n);
                let card2_idx = rng.gen_range(0..n);
                if card1_idx != card2_idx && self.assignment[card1_idx] != self.assignment[card2_idx] {
                    self.pending_move_type = 2;
                    self.pending_move_params = (card1_idx, card2_idx, 0);
                }
            }
        }
    }

    fn eval(&self) -> Option<i64> {
        let delta = match self.pending_move_type {
            1 => {
                let (idx, old, new) = self.pending_move_params;
                self.calculate_change_delta(idx, old, new)
            },
            2 => {
                let (idx1, idx2, _) = self.pending_move_params;
                self.calculate_swap_delta(idx1, idx2)
            },
            _ => return None,
        };
        // 新しいスコア(誤差) = 現在の誤差 - 改善量
        // SAライブラリは最大化問題なので、-1を掛ける
        Some(-(self.current_score - delta))
    }

    fn rollback(&mut self) {
        self.pending_move_type = 0;
    }

    fn post_process(&mut self, new_score: i64) {
        match self.pending_move_type {
            1 => {
                let (card_idx, old_pile, new_pile) = self.pending_move_params;
                let card_value = self.cards[card_idx];
                if old_pile > 0 { self.pile_sums[old_pile] -= card_value; }
                if new_pile > 0 { self.pile_sums[new_pile] += card_value; }
                self.assignment[card_idx] = new_pile;
            },
            2 => {
                let (card1_idx, card2_idx, _) = self.pending_move_params;
                let pile1 = self.assignment[card1_idx];
                let pile2 = self.assignment[card2_idx];
                let card1_val = self.cards[card1_idx];
                let card2_val = self.cards[card2_idx];

                if pile1 > 0 { self.pile_sums[pile1] = self.pile_sums[pile1] - card1_val + card2_val; }
                if pile2 > 0 { self.pile_sums[pile2] = self.pile_sums[pile2] - card2_val + card1_val; }
                self.assignment.swap(card1_idx, card2_idx);
            },
            _ => {}
        }
        self.current_score = -new_score; // 新しい誤差を正しくセット
        self.pending_move_type = 0;
    }

    fn current_score(&self) -> i64 {
        -self.current_score
    }
}



fn generate_cards() -> Vec<u64> {
    let mut cards = Vec::with_capacity(N);
    for _ in 0..50 {
        cards.push(L);
    }
    
    let mut cur = 2e12 as u64;

    while cards.len() < N {
        for _ in 0..25 {
            cards.push(cur);
            if cards.len() == N {
                break;
            }
        }
        cur /= 3;
        cur *= 2;
    }
    
    assert_eq!(cards.len(), N);
    
    cards
}

fn main() {
//    println!("{:?}", generate_cards());
    get_time();
    let mut source = LineSource::new(BufReader::new(stdin()));
    input! {
        from &mut source,
        _n: usize,
        m: usize,
        _l: u64,
        _u: u64
    }

    let a = generate_cards();
    println!("{}", a.iter().join(" "));
    stdout().flush().unwrap();

    input!{
       from &mut source,
        b: [u64; m],
    }
    
    let state = CardAssignmentState::new(a, b);
    let start_temp = 5e10;
    let end_temp = 2e8;
    
    let mut sa = SimulatedAnnealing::new(state, start_temp, end_temp);
    sa.execute(TIME_LIMIT-0.3);
    sa.info();

    let mut accepted = 0;
    let mut iteration = 0;
    let mut rng = thread_rng();
    while get_time() < TIME_LIMIT {
       let n = sa.state.cards.len();
       let m = sa.state.targets.len();

       // 操作1: カードを別の山に移動 (捨てる/拾うも含む)
       let card_idx = rng.gen_range(0..n);
       let old_pile = sa.state.assignment[card_idx];
       let new_pile = rng.gen_range(0..=m);

       if old_pile == new_pile {
           continue;
       }
       iteration += 1;

       // 移動によるスコアの改善量を計算
       let delta = sa.state.calculate_change_delta(card_idx, old_pile, new_pile);

       // 改善する場合のみ、その移動を適用する
       if delta > 0 {
           accepted += 1;
           let card_value = sa.state.cards[card_idx];
           if old_pile > 0 {
               sa.state.pile_sums[old_pile] -= card_value;
           }
           if new_pile > 0 {
               sa.state.pile_sums[new_pile] += card_value;
           }
           sa.state.assignment[card_idx] = new_pile;
           sa.state.current_score -= delta;
       }
    }

    //eprintln!("Final improvement-only phase: iteration: {}, accepted: {}, rate: {:.3}", iteration, accepted, accepted as f64 / iteration as f64);
    println!("{}", sa.state.assignment.iter().join(" "));
    stdout().flush().unwrap();
}

