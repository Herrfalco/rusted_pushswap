use std::collections::VecDeque;
use std::{cmp, env, fmt, io, process};

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
        Ok(stacks)
    }

    pub fn exec(&mut self, op: &str) -> Result<(), &'static str> {
        match op {
            "sa\n" => self.a.swap(0, 1),
            "sb\n" => self.b.swap(0, 1),
            "ra\n" => self.a.rotate_left(1),
            "rb\n" => self.b.rotate_left(1),
            "rra\n" => self.a.rotate_right(1),
            "rrb\n" => self.b.rotate_right(1),
            "ss\n" => {
                self.a.swap(0, 1);
                self.b.swap(0, 1);
            }
            "rr\n" => {
                self.a.rotate_left(1);
                self.b.rotate_left(1);
            }
            "rrr\n" => {
                self.a.rotate_right(1);
                self.b.rotate_right(1);
            }
            "pa\n" => self.b.push_front(self.a.pop_front().unwrap()),
            "pb\n" => self.a.push_front(self.b.pop_front().unwrap()),
            _ => return Err("unknown operation"),
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

/*
fn get_rev_pair(stack: &VecDeque<i32>) -> Option<usize> {
    for i in 0..stack.len() {
        println!("({}, {})", stack[i], stack[(i + 1) % stack.len()]);
        if stack[i] - stack[(i + 1) % stack.len()] == 1 {
            return Some(i);
        }
    }
    None
}
*/

fn error(msg: &str) {
    println!("Error: {}", msg);
    process::exit(1);
}

fn main() {
    match Stacks::new() {
        Ok(mut stacks) => {
            let mut buff = String::new();

            loop {
                println!("{}", &stacks);
                io::stdin().read_line(&mut buff).unwrap();
                if let Err(msg) = stacks.exec(&buff) {
                    error(msg);
                }
                buff.clear();
            }
        }
        Err(msg) => error(msg),
    }
}
