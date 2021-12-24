use std::collections::VecDeque;
use std::{cmp, env, fmt, io, process};

struct Solver {
    ops: Vec<&'static str>,
}

struct Stacks {
    a: VecDeque<i32>,
    b: VecDeque<i32>,
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "    A    |    B    \n\
             -------------------"
        )?;
        for i in 0..cmp::max(self.a.len(), self.b.len()) {
            writeln!(
                f,
                "{:^9}|{:^9}",
                Stacks::opt_to_str(self.a.get(i)),
                Stacks::opt_to_str(self.b.get(i)),
            )?;
        }
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

        let mut stacks = Stacks {
            a: VecDeque::with_capacity(stack_cap),
            b: VecDeque::with_capacity(stack_cap),
        };
        for arg in env::args().skip(1) {
            match i32::from_str_radix(&arg, 10) {
                Ok(v) => stacks.a.push_back(v),
                Err(_) => return Err("wrong parameter"),
            }
        }
        Stacks::check_dup(&stacks.a)?;
        Ok(stacks)
    }

    pub fn exec(&mut self, op: &str) -> Result<(), &'static str> {
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
            "pa" => self.b.push_front(self.a.pop_front().unwrap()),
            "pb" => self.a.push_front(self.b.pop_front().unwrap()),
            _ => return Err("unknown operation"),
        }
        Ok(())
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

    fn opt_to_str(o: Option<&i32>) -> String {
        match o {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
}

impl Solver {
    pub fn new() -> Solver {
        Solver { ops: Vec::new() }
    }

    pub fn solve(&mut self, stacks: &mut Stacks) {
        self.sort_til_4(stacks);
    }

    fn get_order(stack: &VecDeque<i32>) -> Vec<usize> {
        let mut copy: Vec<(usize, &i32)> = stack.iter().enumerate().collect();
        let mut result = vec![0; copy.len()];
        let mut i = 0;

        while copy.len() != 0 {
            let (idx, _) =
                copy.swap_remove(copy.iter().enumerate().min_by_key(|x| x.1 .1).unwrap().0);
            result[idx] = i;
            i += 1;
        }
        result
    }

    fn sort_til_4(&mut self, stacks: &mut Stacks) {
        self.ops
            .append(&mut match Solver::get_order(&stacks.a)[..] {
                [1, 0] => vec!["sa"],
                [0, 2, 1] => vec!["sa", "ra"],
                [1, 0, 2] => vec!["sa"],
                [1, 2, 0] => vec!["rra"],
                [2, 0, 1] => vec!["ra"],
                [2, 1, 0] => vec!["sa", "rra"],
                [0, 1, 3, 2] => vec!["rra", "rra", "sa", "ra", "ra"],
                [0, 2, 1, 3] => vec!["ra", "sa", "rra"],
                [0, 2, 3, 1] => vec!["rra", "sa"],
                [0, 3, 1, 2] => vec!["sa", "ra"],
                [0, 3, 2, 1] => vec!["ra", "sa", "rra", "rra", "sa"],
                [1, 0, 2, 3] => vec!["sa"],
                [1, 0, 3, 2] => vec!["sa", "ra", "ra", "sa", "rra", "rra"],
                [1, 2, 0, 3] => vec!["rra", "rra", "sa", "ra"],
                [1, 2, 3, 0] => vec!["rra"],
                [1, 3, 0, 2] => vec!["rra", "sa", "rra"],
                [1, 3, 2, 0] => vec!["ra", "sa", "rra", "rra"],
                [2, 0, 1, 3] => vec!["rra", "sa", "ra", "ra"],
                [2, 0, 3, 1] => vec!["ra", "sa", "ra"],
                [2, 1, 0, 3] => vec!["sa", "ra", "ra", "sa", "ra"],
                [2, 1, 3, 0] => vec!["sa", "rra"],
                [2, 3, 0, 1] => vec!["ra", "ra"],
                [2, 3, 1, 0] => vec!["ra", "ra", "sa"],
                [3, 0, 1, 2] => vec!["ra"],
                [3, 0, 2, 1] => vec!["rra", "sa", "ra", "sa"],
                [3, 1, 0, 2] => vec!["ra", "sa"],
                [3, 1, 2, 0] => vec!["rra", "sa", "ra"],
                [3, 2, 0, 1] => vec!["sa", "rra", "rra"],
                [3, 2, 1, 0] => vec!["sa", "rra", "rra", "sa"],
                _ => vec![],
            });
        for op in &self.ops {
            stacks.exec(op);
        }
    }
}

fn error(msg: &str) {
    println!("Error: {}", msg);
    process::exit(1);
}

fn trim_ret(buff: &str) -> &str {
    if buff.ends_with("\n") {
        return &buff[..buff.len() - 1];
    }
    buff
}

fn main() {
    match Stacks::new() {
        Ok(mut stacks) => {
            let mut solver = Solver::new();

            solver.solve(&mut stacks);
            println!("{}", solver);
            /*
            let mut buff = String::new();
            loop {
                println!("{}", &stacks);
                io::stdin().read_line(&mut buff).unwrap();
                if let Err(msg) = stacks.exec(trim_ret(&buff)) {
                    error(msg);
                }
                buff.clear();
            }
            */
        }
        Err(msg) => error(msg),
    }
}
