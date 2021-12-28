use std::collections::VecDeque;
use std::{env, fmt, process};

#[cfg(test)]
mod test_mod;

struct Solver {
    ops: Vec<&'static str>,
}

struct Stacks {
    a: VecDeque<i32>,
    b: VecDeque<i32>,
}

enum Direction {
    Ab,
    Ba,
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "A: {:?}", self.a)?;
        writeln!(f, "B: {:?}", self.b)?;
        Ok(())
    }
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for op in &self.ops {
            writeln!(f, "{}", op)?;
        }
        Ok(())
    }
}

impl Stacks {
    pub fn new() -> Result<Stacks, &'static str> {
        let stack_cap = env::args().skip(1).len();

        let stacks = Stacks {
            a: match env::args()
                .skip(1)
                .map(|arg| i32::from_str_radix(&arg, 10))
                .collect::<Result<VecDeque<i32>, _>>()
            {
                Ok(stack) => stack,
                Err(_) => return Err("wrong parameter"),
            },
            b: VecDeque::with_capacity(stack_cap),
        };
        Stacks::check_dup(&stacks.a)?;
        Ok(stacks)
    }

    pub fn exec(&mut self, op: &str) {
        match op {
            "sa" => self.a.swap(0, 1),
            "sb" => self.b.swap(0, 1),
            "ra" => self.a.rotate_left(1),
            "rb" => self.b.rotate_left(1),
            "rra" => self.a.rotate_right(1),
            "rrb" => self.b.rotate_right(1),
            "ss" => {
                self.a.swap(0, 1);
                self.b.swap(0, 1);
            }
            "rr" => {
                self.a.rotate_left(1);
                self.b.rotate_left(1);
            }
            "rrr" => {
                self.a.rotate_right(1);
                self.b.rotate_right(1);
            }
            "pa" => self.a.push_front(self.b.pop_front().unwrap()),
            "pb" => self.b.push_front(self.a.pop_front().unwrap()),
            _ => (),
        };
    }

    fn check_dup(stack: &VecDeque<i32>) -> Result<(), &'static str> {
        let mut copy: Vec<&i32> = stack.iter().collect();

        copy.sort();
        copy.dedup();
        if copy.len() != stack.len() {
            return Err("input duplicate");
        }
        Ok(())
    }
}

impl Solver {
    pub fn new() -> Solver {
        Solver { ops: Vec::new() }
    }

    pub fn solve(&mut self, stacks: &mut Stacks) {
        self.big_sort(stacks, Direction::Ab, 5);
        self.sort_upto_5(stacks);
        self.big_sort(stacks, Direction::Ba, 0);
        self.final_rot(stacks);
    }

    fn get_order(stack: &VecDeque<i32>) -> Vec<usize> {
        let mut copy: Vec<(usize, &i32)> = stack.iter().enumerate().collect();
        let mut result = vec![0; copy.len()];
        let mut i = 0;

        while copy.len() != 0 {
            let (idx, _) = copy.swap_remove(
                copy.iter()
                    .enumerate()
                    .min_by_key(|(_, (_, k))| k)
                    .unwrap()
                    .0,
            );
            result[idx] = i;
            i += 1;
        }
        if result.len() > 0 {
            let len = result.len();
            let delta = len - result[0];

            for val in &mut result {
                *val = (*val + delta) % len;
            }
        }
        result
    }

    fn sort_upto_5(&mut self, stacks: &mut Stacks) {
        let mut ops = match Solver::get_order(&stacks.a)[..] {
            [0, 1, 2, 4, 3] | [0, 1, 3, 2] => vec!["rra", "rra", "sa"],
            [0, 1, 3, 2, 4] => vec!["ra", "ra", "sa"],
            [0, 1, 3, 4, 2] => vec!["rra", "sa", "ra", "sa"],
            [0, 1, 4, 2, 3] => vec!["ra", "sa", "rra", "sa"],
            [0, 1, 4, 3, 2] => vec!["rra", "rra", "sa", "rra", "sa", "ra", "sa"],
            [0, 2, 1, 3, 4] | [0, 2, 1, 3] => vec!["ra", "sa"],
            [0, 2, 1, 4, 3] => vec!["ra", "sa", "ra", "ra", "sa"],
            [0, 2, 3, 1, 4] => vec!["ra", "ra", "sa", "rra", "sa"],
            [0, 2, 3, 4, 1] | [0, 2, 3, 1] => vec!["rra", "sa"],
            [0, 2, 4, 1, 3] => vec!["sa", "ra", "sa", "rra", "rra", "sa"],
            [0, 2, 4, 3, 1] => vec!["rra", "sa", "rra", "rra", "sa"],
            [0, 3, 1, 2, 4] => vec!["ra", "sa", "ra", "sa"],
            [0, 3, 1, 4, 2] => vec!["sa", "rra", "rra", "sa", "ra", "sa"],
            [0, 3, 2, 1, 4] => vec!["sa", "ra", "ra", "sa", "ra", "ra", "sa"],
            [0, 3, 2, 4, 1] => vec!["ra", "sa", "rra", "rra", "sa"],
            [0, 3, 4, 1, 2] => vec!["sa", "ra", "sa"],
            [0, 3, 4, 2, 1] => vec!["rra", "sa", "rra", "sa", "ra", "sa"],
            [0, 4, 1, 2, 3] | [0, 3, 1, 2] | [0, 2, 1] => vec!["sa"],
            [0, 4, 1, 3, 2] | [0, 3, 2, 1] => vec!["sa", "rra", "rra", "sa"],
            [0, 4, 2, 1, 3] => vec!["rra", "sa", "rra", "sa", "rra", "sa", "ra", "sa"],
            [0, 4, 2, 3, 1] => vec!["sa", "rra", "rra", "sa", "rra", "sa"],
            [0, 4, 3, 1, 2] => vec!["sa", "ra", "sa", "rra", "sa"],
            [0, 4, 3, 2, 1] => vec!["sa", "ra", "ra", "sa", "ra", "sa", "rra", "sa"],
            _ => vec![],
        };
        for op in &ops {
            stacks.exec(op);
        }
        self.ops.append(&mut ops);
    }

    fn push_exec(&mut self, stacks: &mut Stacks, op: &'static str) {
        self.ops.push(op);
        stacks.exec(op);
    }

    fn get_places(stacks: &Stacks, dir: &Direction) -> Vec<usize> {
        match dir {
            Direction::Ab => {
                let (src, dst) = (&stacks.a, &stacks.b);
                let (def, _) = dst
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, &k)| k)
                    .unwrap_or((0, &0));
                src.iter()
                    .map(|x| {
                        match dst
                            .iter()
                            .enumerate()
                            .filter(|(_, v)| v < &x)
                            .max_by_key(|(_, &k)| k)
                        {
                            Some((i, _)) => i,
                            None => def,
                        }
                    })
                    .collect()
            }
            Direction::Ba => {
                let (src, dst) = (&stacks.b, &stacks.a);
                let (def, _) = dst
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, &k)| k)
                    .unwrap_or((0, &0));
                src.iter()
                    .map(|x| {
                        match dst
                            .iter()
                            .enumerate()
                            .filter(|(_, v)| v > &x)
                            .min_by_key(|(_, &k)| k)
                        {
                            Some((i, _)) => i,
                            None => def,
                        }
                    })
                    .collect()
            }
        }
    }

    fn op_conv(ops: [usize; 6], dir: &Direction) -> Vec<&'static str> {
        let [r1, rr1, r2, rr2, rr, rrr] = ops;
        let mut result = Vec::with_capacity(ops.iter().sum::<usize>() + 1);

        result.append(&mut vec!["rr"; rr]);
        result.append(&mut vec!["rrr"; rrr]);
        match dir {
            Direction::Ab => {
                result.append(&mut vec!["ra"; r1]);
                result.append(&mut vec!["rra"; rr1]);
                result.append(&mut vec!["rb"; r2]);
                result.append(&mut vec!["rrb"; rr2]);
                result.push("pb");
            }
            Direction::Ba => {
                result.append(&mut vec!["rb"; r1]);
                result.append(&mut vec!["rrb"; rr1]);
                result.append(&mut vec!["ra"; r2]);
                result.append(&mut vec!["rra"; rr2]);
                result.push("pa");
            }
        };
        result
    }

    fn get_best_ops(stacks: &Stacks, dir: &Direction) -> Vec<Vec<&'static str>> {
        let (src, dst) = match &dir {
            Direction::Ab => (&stacks.a, &stacks.b),
            Direction::Ba => (&stacks.b, &stacks.a),
        };
        let places = Solver::get_places(stacks, dir);
        let best_ops = src
            .iter()
            .zip(places.iter())
            .enumerate()
            .map(|(i, (_, j))| (i, src.len() - i, j, dst.len() - j))
            .map(|(ra, rra, rb, rrb)| {
                let mut op_count = [
                    [0; 6],
                    if ra < *rb {
                        [0, 0, *rb - ra, 0, ra, 0]
                    } else {
                        [ra - *rb, 0, 0, 0, *rb, 0]
                    },
                    if rra < rrb {
                        [0, 0, 0, rrb - rra, 0, rra]
                    } else {
                        [0, rra - rrb, 0, 0, 0, rrb]
                    },
                ];
                if ra < rra {
                    op_count[0][0] = ra
                } else {
                    op_count[0][1] = rra
                };
                if *rb < rrb {
                    op_count[0][2] = *rb
                } else {
                    op_count[0][3] = rrb
                };
                op_count
                    .iter()
                    .min_by_key(|k| k.iter().sum::<usize>())
                    .unwrap()
                    .clone()
            });
        let mut result = Vec::with_capacity(src.len());
        for ops in best_ops {
            result.push(Solver::op_conv(ops, &dir));
        }
        result
    }

    fn big_sort(&mut self, stacks: &mut Stacks, dir: Direction, until: usize) {
        while match &dir {
            Direction::Ab => &stacks.a,
            Direction::Ba => &stacks.b,
        }
        .len()
            > until
        {
            for op in Solver::get_best_ops(stacks, &dir)
                .iter()
                .min_by_key(|ops| ops.len())
                .unwrap()
            {
                self.push_exec(stacks, op);
            }
        }
    }

    fn final_rot(&mut self, stacks: &mut Stacks) {
        if let Some((min, _)) = stacks.a.iter().enumerate().min_by_key(|(_, k)| *k) {
            let rots = if min < stacks.a.len() - min {
                vec!["ra"; min]
            } else {
                vec!["rra"; stacks.a.len() - min]
            };
            for op in rots {
                self.push_exec(stacks, op);
            }
        }
    }
}

fn error(msg: &str) {
    println!("Error: {}", msg);
    process::exit(1);
}

fn main() {
    match Stacks::new() {
        Ok(mut stacks) => {
            let mut solver = Solver::new();

            solver.solve(&mut stacks);
            print!("{}", solver);
        }
        Err(msg) => error(msg),
    }
}
