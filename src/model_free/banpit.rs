// use ndarray::s;
// use ndarray::{Array1, Array2};
// use ndarray_stats::QuantileExt;
// use rand::Rng;
// pub struct Bandit {
//     rates: Array1<usize>,
// }

// const ACTION_SIZE: usize = 10;

// impl Bandit {
//     // 새로운 Bandit 인스턴스를 생성하는 메서드
//     pub fn new() -> Self {
//         let mut rng = rand::thread_rng();
//         let rates: Array1<usize> =
//             Array1::from_shape_fn(ACTION_SIZE as usize, |_| rng.gen_range(0..=1));

//         Self { rates }
//     }

//     // 주어진 arm으로 플레이하는 메서드
//     pub fn play(&self, arm: usize) -> usize {
//         let rate = self.rates[arm as usize];
//         let mut rng = rand::thread_rng();
//         let random_value = rng.gen_range(0.0..=1.0);

//         if rate as f64 > random_value {
//             1
//         } else {
//             0
//         }
//     }
// }

// struct Agent {
//     epsilon: f32,
//     qs: Array1<f32>,
//     ns: Array1<usize>,
// }

// impl Agent {
//     fn new(epsilon: f32) -> Self {
//         Self {
//             epsilon: epsilon,
//             qs: Array1::zeros(ACTION_SIZE as usize),
//             ns: Array1::zeros(ACTION_SIZE as usize),
//         }
//     }

//     fn update(&mut self, action: usize, reward: usize) {
//         self.ns[action as usize] += 1;
//         let alpha = 1.0 / self.ns[action as usize] as f32;
//         self.qs[action as usize] += (reward as f32 - self.qs[action as usize]) * alpha;
//     }

//     fn get_action(&mut self) -> usize {
//         let mut rng = rand::thread_rng();
//         let random_value = rng.gen_range(0.0..=1.0);

//         if random_value < self.epsilon {
//             // 랜덤 액션 선택
//             rng.gen_range(0..self.qs.len())
//         } else {
//             // 가장 높은 값을 가진 액션 선택
//             self.qs.argmax().unwrap()
//         }
//     }
// }
// pub fn example() {
//     let runs = 200;
//     let steps = 1000;
//     let epsilon = 0.1;
//     let mut all_rates: Array2<usize> = Array2::zeros((runs, steps));

//     // let mut total_rewards = vec![];

//     for run in 0..runs {
//         let mut total_reward = 0;

//         let bandit = Bandit::new(); // Bandit 인스턴스 생성
//         let mut agent = Agent::new(epsilon);
//         let mut rates = vec![];

//         for step in 0..steps {
//             let action = agent.get_action();
//             let reward = bandit.play(action);
//             agent.update(action, reward);
//             total_reward += reward;

//             // total_rewards.push(total_reward);
//             rates.push(total_reward / (step + 1));
//         }
//         all_rates
//             .slice_mut(s![run, ..])
//             .assign(&Array1::from_vec(rates));

//         all_rates[run]= rates
//     }
//     // let average_axis0 = all_rates.mean_axis(ndarray::Axis(0)).unwrap();
//     // println!("축 0을 따라 계산된 평균: {:?}", average_axis0);
//     println!("총 보상: {:?}", all_rates.mean());
// }
