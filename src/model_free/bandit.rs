/*
Bandit

Multi_armed bandit problem
- 슬롯머신이 여러대인 문제
- 슬롯머신 - 환경
- 플레이어 - 에이전트
- 좋은 슬롯 머신이란
*/
use ndarray::s;
use ndarray::{Array1, Array2};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
pub struct Bandit {
    rates: Array1<f32>,
}

const ACTION_SIZE: usize = 10;

impl Bandit {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rates: Array1<f32> = Array1::from_shape_fn(ACTION_SIZE, |_| rng.gen_range(0.0..=1.));
        Self { rates }
    }

    pub fn play(&self, arm: usize) -> usize {
        let rate = self.rates[arm];
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..=1.0);

        if rate as f64 > random_value {
            1
        } else {
            0
        }
    }
}

struct Agent {
    epsilon: f32,
    qs: Array1<f32>,
    ns: Array1<usize>,
}

impl Agent {
    fn new(epsilon: f32) -> Self {
        Self {
            epsilon,
            qs: Array1::zeros(ACTION_SIZE),
            ns: Array1::zeros(ACTION_SIZE),
        }
    }

    fn update(&mut self, action: usize, reward: usize) {
        self.ns[action] += 1;
        let alpha = 1.0 / self.ns[action] as f32;
        self.qs[action] += (reward as f32 - self.qs[action]) * alpha;
    }

    fn get_action(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..=1.0);

        if random_value < self.epsilon {
            rng.gen_range(0..self.qs.len())
        } else {
            let mut best_index = 0;
            let mut best_value = f32::NEG_INFINITY;

            for (idx, &value) in self.qs.iter().enumerate() {
                if value > best_value {
                    best_index = idx;
                    best_value = value;
                }
            }

            best_index
        }
    }
}

pub fn example() {
    // let runs = 200;
    // let steps = 1000;
    // let epsilon = 0.1;
    // let mut all_rates: Array2<f32> = Array2::zeros((runs, steps)); // ✅ f32로 변경

    // for run in 0..runs {
    //     let mut total_reward = 0;
    //     let bandit = Bandit::new();
    //     let mut agent = Agent::new(epsilon);
    //     let mut rates = vec![];

    //     for step in 0..steps {
    //         let action = agent.get_action();
    //         let reward = bandit.play(action);
    //         agent.update(action, reward);
    //         total_reward += reward;
    //         rates.push(total_reward as f32 / (step + 1) as f32); // ✅ 실수 연산
    //     }

    //     all_rates
    //         .slice_mut(s![run, ..])
    //         .assign(&Array1::from_vec(rates));
    // }

    // println!("총 보상 평균: {:?}", all_rates.mean());
    test();
}

fn test() {
        // 시드 고정 (파이썬 np.random.seed(0)과 동일)
        let mut rng = StdRng::seed_from_u64(0);

        let mut rewards = Vec::new();

        for _ in 1..=10 {
            // np.random.rand() ~ [0.0, 1.0) 난수
            let reward: f64 = rng.gen_range(0.0..1.0);
            rewards.push(reward);
            println!("reward = {}", reward);
        }

        println!("rewards = {:?}", rewards);
    
}
