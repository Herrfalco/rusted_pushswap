use super::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

fn rand_nb_utils(size: i32, space: i32) {
    let mut values: Vec<i32> = (size / 2 - size..size / 2).map(|x| x * space).collect();
    values.shuffle(&mut thread_rng());
    let mut stacks = Stacks {
        a: VecDeque::from_iter(values),
        b: VecDeque::new(),
    };
    let mut solver = Solver::new();

    solver.solve(&mut stacks);
    assert_eq!(
        (1..stacks.a.len()).all(|i| stacks.a[i] > stacks.a[i - 1]),
        true
    );
    assert_eq!(stacks.b.len(), 0);
}

#[test]
#[test]
#[test]
#[test]
#[test]
fn rand_nb() {
    for i in 0..100 {
        rand_nb_utils(i, thread_rng().gen_range(1..5));
    }
}
