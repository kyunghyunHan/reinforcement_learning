use ndarray::Array1;
use rand::Rng;

pub struct Bandit {
    rates: Array1<i32>,
}
const ACTION_SIZE: usize = 10;
impl Bandit {
    // 새로운 Bandit 인스턴스를 생성하는 메서드
    pub fn new(arms: usize) -> Self {
        let mut rng = rand::thread_rng();
        let rates: Array1<i32> = Array1::from_shape_fn(arms, |_| rng.gen_range(0..=1));

        Self { rates }
    }

    // 주어진 arm으로 플레이하는 메서드
    pub fn play(&self, arm: usize) -> i32 {
        let rate = self.rates[arm];
        let mut rng = rand::thread_rng();
        let random_value: i32 = rng.gen_range(0..=1);

        if rate > random_value {
            1
        } else {
            0
        }
    }
}

struct Agent {
    epsilon: i32,
    qs: Array1<usize>,
    ns: Array1<usize>,
}
impl Agent {
    fn new(&mut self, epsilon: i32) {
        self.epsilon = epsilon;
        self.qs = Array1::zeros(ACTION_SIZE);
        self.ns = Array1::zeros(ACTION_SIZE);
    }
    fn update(&mut self, action: usize, reward: usize) {
        self.ns[action] += 1;
        self.qs[action] += (reward - self.qs[action]) / self.ns[action]
    }

    fn get_action(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        let random_value: i32 = rng.gen_range(0..=1);
        if random_value < self.epsilon {
            rng.gen_range(0..=self.qs.len())
        } else {
            0
        }
    }
}
pub fn example() {
    let arms = 10; // 슬롯머신 대수
    let bandit = Bandit::new(arms); // Bandit 인스턴스 생성

    // 0번째 arm으로 3번 플레이
    for _ in 0..3 {
        println!("{}", bandit.play(0));
    }

    let step = 1000;
    let epsilon = 0.1;

    let benfit = Bandit::new(arms);
}
