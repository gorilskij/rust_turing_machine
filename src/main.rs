#[macro_use] extern crate derive_more;

use std::collections::HashMap;
use std::ops::Index;
use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;
use std::fs;

#[derive(Eq, PartialEq, Debug, Display)]
enum Dir { Left, Right, Stay }

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "l" => Ok(Dir::Left),
            "r" => Ok(Dir::Right),
            "-" => Ok(Dir::Stay),
            _ => Err(())
        }
    }
}

struct Enumerator {
    tape: Vec<char>,
    pos: usize,
    states: Vec<String>,
    state: String,
    // { state => { char => (write, go, new_state) } }
    rules: HashMap<String, HashMap<char, (char, Dir, String)>>,
}

impl Enumerator {
    fn new<S0: ToString, S1: ToString>(rules_str: S0, start_state: S1) -> Result<Self, String> {
        // q0: a/b,R b/a/L
        // todo compose regex out of smaller regexes like char and state
        let re =
            Regex::new(r"^q([0-9A-Z]+):(?: +([a-z0-9_])/([a-z0-9_]),([LR-])/q([0-9A-Z]+))+$")
                .expect("failed to create regex");

        let mut rules: HashMap<String, HashMap<char, (char, Dir, String)>> = HashMap::new();
        for line in rules_str.to_string().lines() {
            let captures = re
                .captures(line)
                .ok_or(&format!("Failed to get captures (match regex) on line: {}", line))?;
            let mut match_iter = captures
                .iter()
                .skip(1);

            let state = match_iter
                .next()
                .flatten()
                .map(|s| s.as_str().to_owned())
                .ok_or("Empty line")?;

            let mut state_map = HashMap::new();
            while let Some(chr) = match_iter.next().flatten() {
                let error_parsing_line = |x|
                    format!("Error {} parsing line: {}", x, line);

                let chr = chr
                    .as_str()
                    .chars()
                    .nth(0)
                    .ok_or(&error_parsing_line(0))?;
                let write = match_iter
                    .next()
                    .flatten()
                    .map(|t| t
                        .as_str()
                        .chars()
                        .nth(0))
                    .flatten()
                    .ok_or(error_parsing_line(1))?;
                let dir = match_iter
                    .next()
                    .flatten()
                    .map(|t| t.as_str().parse())
                    .ok_or(error_parsing_line(2))?
                    .map_err(|_| error_parsing_line(3))?;
                let new_state = match_iter
                    .next()
                    .flatten()
                    .map(|f| f.as_str().to_owned())
                    .ok_or(error_parsing_line(4))?;

                assert_eq!(state_map.insert(chr, (write, dir, new_state)), None);
            }
            assert_eq!(rules.insert(state, state_map), None);
        }

        let states: Vec<_> = rules.keys().cloned().collect();
        let start_state = start_state.to_string();
        assert_eq!(start_state.chars().nth(0).unwrap(), 'q');
        assert!(states.contains(&start_state[1..].to_owned()));

        Ok(Self {
            tape: vec![],
            pos: 0,
            states,
            state: start_state,
            rules,
        })
    }
}

fn main() {
    let rules_str = fs::read_to_string("rulesets/0n1n_enumerator_small.txt")
        .expect("failed to read rules file");

    let enumerator = Enumerator::new(rules_str, "qP")
        .expect("failed to create enumerator");
}
