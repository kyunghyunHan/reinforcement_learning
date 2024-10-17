use ndarray::{arr1, arr2, Array1, Array2};
use std::{collections::HashMap, thread::yield_now};

#[derive(Debug)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}
impl Action {
    fn from_u32(value: u32) -> Option<Action> {
        match value {
            0 => Some(Action::Up),
            1 => Some(Action::Down),
            2 => Some(Action::Left),
            3 => Some(Action::Right),
            _ => None, // 잘못된 값 처리
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Action::Up => "UP",
            Action::Down => "DOWN",
            Action::Left => "LEFT",
            Action::Right => "RIGHT",
        }
    }
}
#[derive(Debug, Clone)]
struct GridWorld {
    action_space: Array1<i32>,
    action_meaning: HashMap<i32, String>,
    reward_map: Array2<Option<f32>>,
    goal_state: (i32, i32),
    wall_state: (i32, i32),
    start_state: (i32, i32),
    agent_state: (i32, i32),
}
struct Meaning {
    one: String,
}
impl GridWorld {
    fn new() -> Self {
        let mut action_meaning = HashMap::new();
        action_meaning.insert(0, "UP".to_uppercase());
        action_meaning.insert(1, "DOWN".to_uppercase());
        action_meaning.insert(2, "LEFT".to_uppercase());
        action_meaning.insert(3, "RIGHT".to_uppercase());
        Self {
            action_space: arr1(&[0, 1, 2, 3]),
            action_meaning,
            reward_map: arr2(&[
                [Some(0.), Some(0.), Some(0.), Some(1.)],
                [Some(0.), None, Some(0.), Some(-1.)],
                [Some(0.), Some(0.), Some(0.), Some(0.)],
            ]),
            goal_state: (0, 3),
            wall_state: (1, 1),
            start_state: (2, 0),
            agent_state: (3, 0),
        }
    }
    fn height(&self) -> usize {
        self.reward_map.dim().0
    }
    fn width(&self) -> usize {
        self.reward_map.shape()[1]
    }
    fn shape(&self) -> Vec<usize> {
        self.reward_map.shape().to_vec()
    }

    fn actions(&self) -> &Array1<i32> {
        &self.action_space
    }

    fn states(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        // 0부터 height까지의 범위와 0부터 width까지의 범위를 이터레이터로 생성
        (0..self.height()).flat_map(move |h| {
            (0..self.width()).map(move |w| {
                (h, w) // 각 상태를 튜플로 반환
            })
        })
    }
}
pub fn example() {
    let env: GridWorld = GridWorld::new();
    println!("{}", &env.height());
    println!("{}", &env.width());
    println!("{:?}", &env.shape());

    for action in env.actions() {
        println!("Action: {}", action);
    }

    for state in env.states() {
        println!("{:?}",state);
    }
}
