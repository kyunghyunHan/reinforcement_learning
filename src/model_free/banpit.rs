use ndarray::Array1;
use ndarray_stats::QuantileExt;
use rand::Rng;

pub struct Bandit {
    rates: Array1<i32>,
}

const ACTION_SIZE: i32 = 10;

impl Bandit {
    // 새로운 Bandit 인스턴스를 생성하는 메서드
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rates: Array1<i32> =
            Array1::from_shape_fn(ACTION_SIZE as usize, |_| rng.gen_range(0..=1));

        Self { rates }
    }

    // 주어진 arm으로 플레이하는 메서드
    pub fn play(&self, arm: i32) -> i32 {
        let rate = self.rates[arm as usize];
        let mut rng = rand::thread_rng();
        let random_value= rng.gen_range(0.0..=1.0);

        if rate  as f64> random_value {
            1
        } else {
            0
        }
    }
}

struct Agent {
    epsilon: f32,
    qs: Array1<f32>,
    ns: Array1<i32>,
}

impl Agent {
    fn new(epsilon: f32) -> Self {
        Self {
            epsilon: epsilon,
            qs: Array1::zeros(ACTION_SIZE as usize),
            ns: Array1::zeros(ACTION_SIZE as usize),
        }
    }

    fn update(&mut self, action: i32, reward: i32) {
        self.ns[action as usize] += 1;
        let alpha = 1.0 / self.ns[action as usize] as f32;
        self.qs[action as usize] += (reward as f32 - self.qs[action as usize]) * alpha;
    }

    fn get_action(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..=1.0);

        if random_value < self.epsilon {
            // 랜덤 액션 선택
            rng.gen_range(0..self.qs.len()) as i32
        } else {
            // 가장 높은 값을 가진 액션 선택
            self.qs.argmax().unwrap() as i32
        }
    }
}
pub fn example() {
    let bandit = Bandit::new(); // Bandit 인스턴스 생성
    let steps = 1000;
    let epsilon = 0.1;

    let mut agent = Agent::new(epsilon);
    let mut total_reward = 0;
    let mut total_rewards = vec![];
    let mut rates = vec![];

    for step in 0..steps {
        let action = agent.get_action();
        let reward = bandit.play(action);
        agent.update(action, reward);
        total_reward += reward;

        total_rewards.push(total_reward);
        rates.push(total_reward / (step + 1));
    }

    println!("총 보상: {}", total_reward);
}
