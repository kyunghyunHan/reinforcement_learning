#[derive(Debug, Clone)]
struct Test {
    l1: f64,
    l2: f64,
}

pub fn example() {
    let mut v = Test { l1: 0., l2: 0. };
    let mut new_v = v.clone();
    let mut cnt = 0; //갱신 횟수 기록
    loop {
        let mut t = 0.5 * (-1. + 0.9 * v.l1) + 0.5 * (1. + 0.9 * v.l2);
        let mut delta = (t - v.l1).abs();
        v.l1 = t;
        t = 0.5 * (0. + 0.9 * v.l1) + 0.5 * (-1. + 0.9 * v.l2);
        // new_v.l1 = 0.5 * (-1. + 0.9 * v.l1) + 0.5 * (1. + 0.9 * v.l2);
        // new_v.l2 = 0.5 * (0. + 0.9 * v.l1) + 0.5 * (-1. + 0.9 * v.l2);

        //갱신된 양의최댓값
        // let mut delta = (new_v.l1 - v.l1).abs();
        // delta = f64::max(delta, (new_v.l2 - v.l2).abs());
        delta = f64::max(delta, (t - v.l2).abs());

        // v = new_v.clone();
        v.l2 = t;
        cnt += 1;

        if delta < 0.0001 {
            println!("{:?}", v);
            println!("갱신 횟수{}", cnt);
            break;
        }
    }
}
