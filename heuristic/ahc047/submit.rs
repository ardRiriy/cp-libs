use std::{cmp::Reverse, collections::HashMap};
use itertools::Itertools;
use num_integer::Roots;
use proconio::input;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use utils::utils::get_time;

mod utils {
    #[allow(dead_code)]
    pub mod utils {
        #[inline]
        pub fn get_time() -> f64 {  // sec
            static mut STIME: f64 = -1.0;
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();
            let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
            unsafe {
                if STIME < 0.0 {
                    STIME = ms;
                }
                ms - STIME
            }
        }
    }
}

static TIME_LIMIT: f64 = 1.95;

// Function to compute word probability (from the attached scoring code)
fn compute_word_probability(word: &Vec<char>, L: usize, C: &Vec<char>, A: &Vec<Vec<i32>>) -> f64 {
    let M = C.len();
    // Enumerate the automaton states (matching length, vertex)
    let mut n = 0;
    let mut states = HashMap::new();
    for j in 0..M {
        states.insert((0, j), n);
        n += 1;
        for i in 0..word.len() - 1 {
            if word[i] == C[j] {
                states.insert((i + 1, j), n);
                n += 1;
            }
        }
    }
    // Construct the state transition matrix
    let mut X = vec![0.0; n * n];
    for (&(len, u), &j) in &states {
        for v in 0..M {
            let mut next = word[..len].to_vec();
            next.push(C[v]);
            let mut s = 0;
            while next[s..] != word[..next.len() - s] {
                s += 1;
            }
            if next.len() - s != word.len() {
                let i = states[&(next.len() - s, v)];
                X[i * n + j] += A[u][v] as f64 / 100.0;
            }
        }
    }
    // Compute Y=X^(L-1)
    let mut power = L - 1;
    let mut Y = vec![0.0; n * n];
    for i in 0..n {
        Y[i * n + i] = 1.0;
    }
    while power > 0 {
        if power & 1 != 0 {
            Y = mul(&Y, &X, n);
        }
        X = mul(&X, &X, n);
        power >>= 1;
    }
    // Compute the probability
    let init = if C[0] == word[0] {
        states[&(1, 0)]
    } else {
        states[&(0, 0)]
    };
    let mut ret = 1.0;
    for i in 0..n {
        ret -= Y[i * n + init];
    }
    ret.clamp(0.0, 1.0)
}

// Matrix multiplication helper (from the attached scoring code)
fn mul(a: &Vec<f64>, b: &Vec<f64>, n: usize) -> Vec<f64> {
    let mut c = vec![0.0; n * n];
    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
    c
}

// Function to compute the total score
fn compute_total_score(strings: &Vec<Vec<char>>, preferences: &Vec<usize>, L: usize, C: &Vec<char>, A: &Vec<Vec<i32>>) -> f64 {
    let mut total_score = 0.0;
    for (word, &score) in strings.iter().zip(preferences) {
        let prob = compute_word_probability(word, L, C, A);
        total_score += prob * score as f64;
    }
    total_score
}

fn main() {
    get_time();
    input! {
        n: usize,
        m: usize,
        l: usize,
        lovely: [(String, usize); n],
    }
    let mut rng = thread_rng();
    let strings: Vec<Vec<char>> = lovely.iter().map(|(s, _)| s.chars().collect()).collect();
    let preferences: Vec<usize> = lovely.iter().map(|(_, p)| *p).collect();
    
    let mut best_score = 0.0;
    let mut best_c = vec!['a'; m];
    let mut best_v = vec![vec![0i32; m]; m];
    
    // Try each string as the starting point
    while get_time() < TIME_LIMIT {
        for (start_idx, (start_str, start_weight)) in lovely
            .iter()
            .sorted_by_key(|(_, w)| Reverse(w))
            .take(17)
            .enumerate() {
            if get_time() > TIME_LIMIT { break; }
            let mut used = vec![false; n];
            used[start_idx] = true;
            
            let mut res_c = vec!['a'; m];
            for i in 0..m {
                res_c[i] = (b'a' + rng.gen_range(0..6) as u8) as char;
            }

            let mut res_v = vec![vec![0i32; m]; m];
            
            // Initialize characters from the starting string
            for (i, c) in start_str.char_indices().take(m) {
                res_c[i] = c;
            }
            
            // Initialize a simple cycle for transitions
            for i in 0..m {
                res_v[i][(i+1) % m] = 100;
            }
            
            // Improve transitions by considering other strings
            for ply in 0..(n/7) {
                let mut max_score = 0;
                let mut max_idx = !0;
                let mut trace = vec![];
                
                'lovely: for (i, (s, w)) in lovely.iter().enumerate() {
                    if used[i] { continue; }
                    
                    let mut cur = 0;
                    let sv = s.chars().collect_vec();
                    let mut trace_state = vec![];
                    
                    while cur < s.len() {
                        let mut max_match_length = 0;
                        let mut idx = !0;
                        
                        for i in 0..m {
                            let mut vertex = i;
                            let mut cur_on_sv = cur;
                            let mut cnt = 0;
                            
                            while cur_on_sv < s.len() && sv[cur_on_sv] == res_c[vertex] {
                                cnt += 1;
                                cur_on_sv += 1;
                                
                                // Find the next state with highest probability
                                let mut max_prob = 0;
                                let mut max_prob_idx = 0;
                                
                                for j in 0..m {
                                    if res_v[vertex][j] > max_prob {
                                        max_prob = res_v[vertex][j];
                                        max_prob_idx = j;
                                    }
                                }
                                
                                vertex = max_prob_idx;
                            }
                            
                            if max_match_length < cnt {
                                max_match_length = cnt;
                                idx = i;
                            }
                        }
                        
                        if idx == !0 { continue 'lovely; }
                        
                        trace_state.push((cur, idx, max_match_length));
                        cur += max_match_length;
                    }
                    
                    let score = (*w / 3) / trace_state.len();
                    
                    if max_score < score {
                        max_score = score;
                        trace = trace_state;
                        max_idx = i;
                    }
                }
                
                if max_idx == !0 {
                    break;
                }
                
                used[max_idx] = true;
                
                // Add new transitions
                let mut prev = !0;
                
                for (c, idx, len) in trace {
                    if prev != !0 {
                        if res_v[prev][(prev+1)%m] >= 20 {
                            let val = if 25 > ply { 25 - ply } else { 1 };
                            res_v[prev][((prev+1)%m) as usize] -= val as i32;
                            res_v[prev][idx] += val as i32;
                        }
                    }
                    
                    prev = (idx + len - 1) % m;
                }
            }
            
            let current_score = compute_total_score(&strings, &preferences, l, &res_c, &res_v);

            if current_score > best_score {
                best_score = current_score;
                best_c = res_c;
                best_v = res_v;
            }
        }
    }
   
    
    // Output the best solution
    for i in 0..m {
        println!("{} {}", best_c[i], best_v[i].iter().join(" "));
    }
}
