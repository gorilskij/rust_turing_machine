use std::collections::HashMap;
use lazy_static::lazy_static;

mod parse;
pub(crate) mod without_comments;

#[derive(Eq, PartialEq, Debug, Display)]
enum Dir { Left, Right, Stay }

impl From<char> for Dir {
    fn from(c: char) -> Self {
        use Dir::*;
        match c {
            'l' | 'L' => Left,
            'r' | 'R' => Right,
            '-' => Stay,
            _ => panic!("Invalid character '{}'", c)
        }
    }
}

// todo supercede
// fn line_to_entry(line: &str) -> Result<(String, HashMap<char, (char, Dir, String)>), String> {
//     lazy_static! {
//         static ref STATE_RE: Regex = Regex::new(r"q([0-9A-Z]+)")
//             .map_err(|e| e.to_string())
//             .expect("Failed to build state regex");
//
//         static ref RULE_RE: Regex = Regex::new(r"([a-z0-9_])/([a-z0-9_]),([LR-])/q([0-9A-Z]+)")
//             .map_err(|e| e.to_string())
//             .expect("Failed to build rule regex");
//     }
//
//     let mut iter = line.splitn(2, ": ");
//     let state = STATE_RE
//         .captures(iter
//             .next()
//             .ok_or("No state")?)
//         .map(|c| c
//             .get(1)
//             .map(|m| m
//                 .as_str()
//                 .to_owned()))
//         .flatten()
//         .ok_or("Failed to match state")?;
//     let rest = iter
//         .next()
//         .ok_or(&format!("No rules for state '{}'", state))?;
//
//     let mut state_map = HashMap::new();
//     for rule in rest.split(' ') {
//         let rule_fail = format!("Failed to match rule '{}'", rule);
//
//         let caps = RULE_RE
//             .captures(rule)
//             .ok_or(&rule_fail.clone())?;
//
//         fn extract(caps: &Captures, index: usize, rule_fail: String) -> Result<char, String> {
//             caps
//                 .get(index)
//                 .map(|m| m
//                     .as_str()
//                     .chars()
//                     .nth(0))
//                 .flatten()
//                 .ok_or(rule_fail.clone())
//         }
//
//         let read = extract(&caps, 1, rule_fail.clone())?;
//         let write = extract(&caps, 2, rule_fail.clone())?;
//         let go = extract(&caps, 3, rule_fail.clone())?.into();
//         let new_state = caps
//             .get(4)
//             .map(|m| m
//                 .as_str()
//                 .to_owned())
//             .ok_or(&rule_fail.clone())?;
//
//         let old = state_map.insert(read, (write, go, new_state));
//         if let Some(_) = old {
//             return Err("Duplicate insertion".to_owned())
//         }
//     }
//
//     Ok((state, state_map))
// }

// alphabet and state set are implicit
pub(crate) struct Enumerator {
    tape: Vec<char>,
    pos: usize,
    state: String,
    start_state: String,
    // { state => { char => (write, go, new_state) } }
    rules: HashMap<String, HashMap<char, (char, Dir, String)>>,
}

impl Enumerator {
    // fn new<S0: ToString, S1: ToString>(rules_str: S0, start_state: S1) -> Result<Self, String> {
    //     let mut rules: HashMap<String, HashMap<char, (char, Dir, String)>> = HashMap::new();
    //     for line in rules_str.to_string().lines() {
    //         let (k, v) = line_to_entry(line).unwrap();
    //         rules.insert(k, v);
    //     }
    //
    //     let mut start_state = start_state.to_string();
    //     assert_eq!(start_state.chars().nth(0).unwrap(), 'q');
    //     start_state.remove(0);
    //
    //     // todo run an integrity check
    //
    //     Ok(Self {
    //         tape: vec![],
    //         pos: 0,
    //         state: start_state.clone(),
    //         start_state,
    //         rules,
    //     })
    // }

    // returns number of state transitions performed
    pub(crate) fn run(&mut self, mut iterations: usize) -> usize {
        let mut state_transitions = 0;
        loop {
            if self.state == "P".to_owned() {
                println!("{}", self.tape.iter().collect::<String>().trim());
                if iterations > 1 {
                    iterations -= 1;
                } else {
                    break
                }
            }

            let transition = self.rules
                .get(&self.state)
                .expect(&format!("state '{}' not found, bad TM", self.state));

            let mut chr = self.tape
                .get(self.pos)
                .copied()
                .unwrap_or(' ');

            if chr == ' ' { chr = '_' }
            let (write, dir, new_state) = transition
                .get(&chr)
                .expect(
                    &format!("char '{}' not found for state '{}', bad TM", chr, self.state));

            if self.tape.len() <= self.pos {
                self.tape.resize(self.pos + 1, ' ');
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