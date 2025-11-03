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

use crate::model_free::bandit;

struct Bandit {
    rates: Array1<f64>,
}

impl Bandit {
    fn new(arms: usize) -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let rates = Array1::from_shape_fn(arms, |_| rng.gen_range(0.0..1.0));
        Self { rates }
    }

    fn play(&self, arm: usize) -> usize {
        let mut rng = rand::thread_rng();
        let rate = self.rates[arm];

        if rate > rng.gen_range(0.0..1.0) {
            1
        } else {
            0
        }
    }
}
struct Agent {
    epsilon: f64,
    qs: Array1<f64>,
    ns: Array1<f64>,
}

impl Agent {
    fn new(epsilon: f64, action_size: usize) -> Self {
        Self {
            epsilon,
            qs: Array1::zeros(action_size),
            ns: Array1::zeros(action_size),
        }
    }

    fn update(&mut self, action: usize, reward: f64) {
        self.ns[action] += 1.0;
        self.qs[action] += (reward - self.qs[action]) / self.ns[action];
    }
    fn get_action(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random_value: f64 = rng.gen_range(0.0..1.0);

        if random_value < self.epsilon {
            // 무작위 행동
            rng.gen_range(0..self.qs.len())
        } else {
            // argmax 대신 f64 비교 안전하게 max_by 사용
            self.qs
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0
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
    // test();

    test4();
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

fn test2() {
    // numpy random seed(0) 과 같은 효과
    let mut rng = StdRng::seed_from_u64(0);

    let mut q: f64 = 0.0;

    for n in 1..=10 {
        // np.random.rand()
        let reward: f64 = rng.gen_range(0.0..1.0);

        // Q = Q + (reward - Q) / n
        q = q + (reward - q) / n as f64;

        println!("step {n}: Q = {q}");
    }
}

fn test3() {
    let bandit = Bandit::new(10);
    for i in 0..3 {
        let reward = bandit.play(0);
        println!("{}", reward);
    }
}

fn test4() {
    // let runs = 200;
    let steps = 1000;
    let epsilon = 0.1;
    let mut bandit = Bandit::new(20);
    let mut agent = Agent::new(epsilon, 10);

    let mut total_reward = 0;
    let mut total_reawrds = vec![];
    let mut reates = vec![];
    for step in 0..steps {
        let action = agent.get_action();
        let reward = bandit.play(action);
        agent.update(action, reward as f64);
        total_reward += reward;

        total_reawrds.push(total_reward);
        reates.push(total_reward / (step + 1))
    }
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
    println!("{}", total_reward);
}
