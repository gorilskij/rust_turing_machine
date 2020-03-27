use std::collections::HashMap;
use lazy_static::lazy_static;
use without_comments::IntoWithoutComments;
use std::str::FromStr;
use crate::enumerator::rules::{Rules, Symbol, State};
use itertools::Itertools;
use std::borrow::Borrow;

mod rules;
mod bientry;

#[derive(Eq, PartialEq, Debug, Display)]
pub enum Dir { Left, Right, Stay }

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        match s {
            "l" | "L" | "<" => Ok(Left),
            "r" | "R" | ">" => Ok(Right),
            "-" => Ok(Stay),
            _ => Err(()),
        }
    }
}

// alphabet and state set are implicit
pub struct Enumerator {
    tape: Vec<Symbol>,
    pos: usize,
    state: State,
    start_state: State,
    print_state: Option<State>,
    // { state => { char => (write, go, new_state) } }
    rules: Rules,
}

impl Enumerator {
    // todo run an integrity check
    pub fn from_string(string: String, start_state: String, print_state: String) -> Self {
        println!("input:\n--\n{}\n--", string);

        let mut rules = Rules::default();

        // [qA, r, w, g, qB] (gets cleared after each iteration)
        let mut partial = Vec::with_capacity(5);
        let mut iter = &mut string.chars().without_comments().peekable();

        'outer: loop {
            for _ in 0..5 {
                let _: String = iter
                    .peeking_take_while(|c| c.is_whitespace())
                    .collect();
                let s: String = iter
                    .peeking_take_while(|&c| !c.is_whitespace())
                    .collect();

                if s.is_empty() {
                    break 'outer
                }

                partial.push(s);
            }
            rules.insert_vec(&mut partial)
        }

        let start_state = *rules.states().get_by_left(&start_state).unwrap();
        let print_state = *rules.states().get_by_left(&print_state).unwrap();

        Self {
            tape: vec![],
            pos: 0,
            state: start_state,
            start_state,
            print_state: Some(print_state),
            rules,
        }
    }

    // returns number of state transitions performed
    pub(crate) fn run(&mut self, mut iterations: usize) -> usize {
        let mut state_transitions = 0;
        loop {
            if let Some(print_state) = self.print_state {
                if self.state == print_state {
                    let tape_string = self.tape
                        .iter()
                        .map(|i| match i {
                            0 => "",
                            j => self.rules
                                .symbols()
                                .get_by_right(j)
                                .unwrap()
                                .borrow(),
                        })
                        .intersperse("|") // todo skip this step if all symbols are 1-char long
                        .collect::<String>();

                    println!("{}", tape_string.trim());
                    if iterations > 1 {
                        iterations -= 1;
                    } else {
                        break
                    }
                }
            }

            let transition = self.rules
                .get(&self.state)
                .expect(&format!("state '{}' not found, bad TM", self.state));

            let mut chr = self.tape
                .get(self.pos)
                .copied()
                .unwrap_or(0);

            let (write, dir, new_state) = transition
                .get(&chr)
                .expect(
                    &format!("char {:?} not found for state '{}', bad TM", chr, self.state));

            if self.tape.len() <= self.pos {
                self.tape.resize(self.pos + 1, 0);
            }
            self.tape[self.pos] = *write;
            use Dir::*;
            match dir {
                Left => if self.pos > 0 { self.pos -= 1 },
                Right => self.pos += 1,
                Stay => (),
            }
            self.state = new_state.clone();

            state_transitions += 1;
        }
        state_transitions
    }

    fn reset(&mut self) {
        self.tape.clear();
        self.pos = 0;
        self.state = self.start_state.clone();
    }
}