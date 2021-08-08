use rand::Rng;

pub struct Bandit {
    rates: Vec<i32>,
}

impl Bandit {
    // 새로운 Bandit 인스턴스를 생성하는 메서드
    pub fn new(arms: usize) -> Self {
        let mut rng = rand::thread_rng();
        let rates = (0..arms).map(|_| rng.gen_range(0..=1)).collect::<Vec<i32>>();
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

pub fn example() {
    let arms = 5; // arm의 개수
    let bandit = Bandit::new(arms); // Bandit 인스턴스 생성

    // 0번째 arm으로 3번 플레이
    for _ in 0..3 {
        println!("{}", bandit.play(0));
    }
}
