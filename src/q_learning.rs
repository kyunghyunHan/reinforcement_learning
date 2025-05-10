use rand::prelude::*;
use std::collections::HashMap;
use rand::thread_rng;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State(u8); // 0 ~ 3

fn all_actions() -> Vec<Action> {
    vec![Action::Up, Action::Down, Action::Left, Action::Right]
}

// 상태 전이 함수: (다음 상태, 보상)
fn step(state: State, action: Action) -> (State, f32) {
    match (state.0, action) {
        (0, Action::Right) => (State(1), 0.0),
        (0, Action::Down) => (State(2), 0.0),
        (1, Action::Left) => (State(0), 0.0),
        (1, Action::Down) => (State(3), 0.0),
        (2, Action::Up) => (State(0), 0.0),
        (2, Action::Right) => (State(3), 0.0),
        (3, _) => (State(3), 1.0), // Goal 상태
        _ => (state, 0.0),         // Invalid move: stay
    }
}

pub fn example() {
    let states = vec![State(0), State(1), State(2), State(3)];
    let actions = all_actions();
    let mut q_table: HashMap<(State, Action), f32> = HashMap::new();

    let alpha = 0.1;
    let gamma = 0.9;
    let epsilon = 0.1;

    let mut rng = thread_rng();

    for _episode in 0..500 {
        let mut state = State(0);

        loop {
            // ε-greedy 정책
            let action = if rng.gen::<f32>() < epsilon {
                *actions.choose(&mut rng).unwrap()
            } else {
                // 가장 높은 Q값을 가지는 행동 선택
                *actions
                    .iter()
                    .max_by(|a, b| {
                        q_table
                            .get(&(state, **a))
                            .unwrap_or(&0.0)
                            .partial_cmp(q_table.get(&(state, **b)).unwrap_or(&0.0))
                            .unwrap()
                    })
                    .unwrap()
            };

            let (next_state, reward) = step(state, action);

            let max_next_q = actions
                .iter()
                .map(|a| *q_table.get(&(next_state, *a)).unwrap_or(&0.0))
                .fold(f32::NEG_INFINITY, f32::max);

            let q = q_table.entry((state, action)).or_insert(0.0);
            *q += alpha * (reward + gamma * max_next_q - *q);

            if next_state.0 == 3 {
                break;
            }

            state = next_state;
        }
    }

    // 결과 출력
    for state in &states {
        println!("State {}:", state.0);
        for action in &actions {
            let q = q_table.get(&(*state, *action)).unwrap_or(&0.0);
            println!("  {:?}: {:.3}", action, q);
        }
    }
}
