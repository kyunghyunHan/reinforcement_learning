#[derive(Debug, Clone)]
struct Test {
    l1: f64,
    l2: f64,
}

pub fn example() {
    let mut v = Test { l1: 0., l2: 0. };
    let mut new_v = v.clone();

    for i in 0..100 {
        new_v.l1 = 0.5 * (-1. + 0.9 * v.l1) + 0.5 * (1. + 0.9 * v.l2);
        new_v.l2 = 0.5 * (0. + 0.9 * v.l1) + 0.5 * (-1. + 0.9 * v.l2);
        v = new_v.clone();
        println!("{:?}", v);
    }
}
