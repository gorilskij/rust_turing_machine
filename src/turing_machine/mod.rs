use crate::turing_machine::rules::{Rules, State, Symbol, Tresult};
use itertools::Itertools;
use std::borrow::Borrow;
use std::path::Path;
use std::str::FromStr;

mod entry;
mod rules;

#[derive(Eq, PartialEq, Debug, Display)]
pub enum Dir {
    Left,
    Right,
    Stay,
}

impl FromStr for Dir {
    type Err = (); // eh

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        match s {
            "l" | "L" | "<" => Ok(Left),
            "r" | "R" | ">" => Ok(Right),
            "-" | "_" => Ok(Stay),
            _ => Err(()),
        }
    }
}

// alphabet and state set are implicit
pub struct TM {
    tape: Vec<Symbol>,
    pos: usize,
    state: State,
    start_state: State,
    print_state: Option<State>,
    // { state => { char => (write, go, new_state) } }
    rules: Rules,
}

#[allow(dead_code)]
impl TM {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let string = std::fs::read_to_string(path).unwrap();
        let (rules, start_state, print_state) = Rules::from_string(string);
        match rules.integrity_check(start_state, print_state) {
            Tresult::Ok(()) => (),
            Tresult::Err(s) => panic!("{}", s),
            Tresult::Warn(s) => eprintln!("Warning: {}", s),
        }
        Self {
            tape: vec![],
            pos: 0,
            state: start_state,
            start_state,
            print_state,
            rules,
        }
    }

    // returns number of state transitions performed
    pub fn run(&mut self, mut iterations: usize) -> usize {
        let mut state_transitions = 0;
        loop {
            if let Some(print_state) = self.print_state {
                if self.state == print_state {
                    let tape_string = self
                        .tape
                        .iter()
                        .map(|i| match i {
                            0 => "",
                            j => self.rules.symbols().get_by_right(j).unwrap().borrow(),
                        })
                        .intersperse("|")
                        .collect::<String>();

                    println!("{}", tape_string.trim());
                    if iterations > 1 {
                        iterations -= 1;
                    } else {
                        break;
                    }
                }
            }

            let read = self.tape.get(self.pos).copied().unwrap_or(0);

            let (write, dir, new_state) =
                self.rules.lookup(self.state, read).unwrap_or_else(|| {
                    panic!(
                        "char {:?} not found for state '{}', bad TM",
                        read, self.state
                    )
                });

            if self.tape.len() <= self.pos {
                self.tape.resize(self.pos + 1, 0);
            }
            self.tape[self.pos] = *write;
            use Dir::*;
            match dir {
                Left => {
                    if self.pos > 0 {
                        self.pos -= 1
                    }
                }
                Right => self.pos += 1,
                Stay => (),
            }
            self.state = *new_state;

            state_transitions += 1;
        }
        state_transitions
    }

    pub fn reset(&mut self) {
        self.tape.clear();
        self.pos = 0;
        self.state = self.start_state;
    }
}
