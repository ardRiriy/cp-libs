#[allow(dead_code)]
pub mod simulated_annealing {
    use std::{marker::PhantomData, ops::Sub};

    use num::ToPrimitive;
    use rand::{thread_rng, Rng};

    use crate::utils::utils::get_time;

    pub trait AnnealingState<T> {
        fn pre_process(&mut self);      // 焼きなまし実行前の準備
        fn eval(&self) -> Option<T>;    // evalした結果得られるスコアを返却。変更は行わない
        fn rollback(&mut self);         // 元に戻す処理など(reject)
        fn post_process(&mut self, new_score:T);     // accept
        fn current_score(&self) -> T;
    }

    pub struct SimulatedAnnealing<S, T>
    where S: AnnealingState<T>
    {
        pub state: S,
        start_temp: f64,
        end_temp: f64,
        _marker: PhantomData<T>, // 型Tを明示する
    }

    impl<S: AnnealingState<T>, T: Copy+ToPrimitive + Sub<Output = T>> SimulatedAnnealing<S, T> {
        pub fn new(
            state: S,
            start_temp: f64,
            end_temp: f64,
        ) -> Self {
            Self {
                state,
                start_temp,
                end_temp,
                _marker: PhantomData
            }
        }

        pub fn execute(&mut self, limit: f64) {
            let mut rng = thread_rng();
            while get_time() < limit {
                self.state.pre_process();
                if let Some(new_score) = self.state.eval() {
                    // TODO: 設定可能に変える
                    let temp = self.start_temp + (self.end_temp - self.start_temp) * (get_time() / limit); 
                    let prob = f64::exp((new_score - self.state.current_score()).to_f64().unwrap() / temp);
                    if prob < rng.gen_range(0.0..1.0) {
                        self.state.post_process(new_score);
                    } else {
                        self.state.rollback();
                    }
                }
            }
        }
    }
}