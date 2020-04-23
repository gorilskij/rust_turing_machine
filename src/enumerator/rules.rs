use super::entry::GetEntry;
use crate::enumerator::Dir;
use bimap::BiMap;
use no_comment::IntoWithoutComments;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

pub type State = usize;
pub type Symbol = usize; // 'empty' is always 0
type RulesMap = HashMap<State, HashMap<Symbol, (Symbol, Dir, State)>>;

// { from_state => { read => (write, go, to_state) } }
#[derive(Debug, Default, Getters)]
pub struct Rules {
    map: RulesMap,
    // usize used for lookup
    #[getset(get = "pub")]
    states: BiMap<String, State>,
    #[getset(get = "pub")]
    symbols: BiMap<String, Symbol>,
}

impl Deref for Rules {
    type Target = RulesMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

#[derive(PartialEq, Debug)]
pub enum Tresult<T, E, W> {
    Ok(T),
    Err(E),
    Warn(W),
}

// todo warn about inconsistent direction notation (</l/L and >/r/R)
impl Rules {
    fn get_state_name(&self, state: State) -> &str {
        self.states
            .get_by_right(&state)
            .unwrap_or_else(|| panic!("failed to get name of state {}", state))
            .as_str()
    }

    pub fn insert_slice(&mut self, slice: &[&str]) {
        if let [from_state, read, write, go, to_state] = *slice {
            let dbg = (from_state, read);

            let from_state = {
                let new_state_idx = self.states.len();
                *self
                    .states
                    .entry_by_left(from_state.to_string())
                    .or_insert(new_state_idx)
            };

            let to_state = {
                let new_state_idx = self.states.len();
                *self
                    .states
                    .entry_by_left(to_state.to_string())
                    .or_insert(new_state_idx)
            };

            let read = if read == "_" {
                0
            } else {
                let new_symbol_idx = self.symbols.len() + 1;
                *self
                    .symbols
                    .entry_by_left(read.to_string())
                    .or_insert(new_symbol_idx)
            };

            let write = if write == "_" {
                0
            } else {
                let new_symbol_idx = self.symbols.len() + 1;
                *self
                    .symbols
                    .entry_by_left(write.to_string())
                    .or_insert(new_symbol_idx)
            };

            let value = (
                write,
                go.parse()
                    .unwrap_or_else(|_| panic!("failed to parse go value: {:?}", go)),
                to_state,
            );

            let sub_map = self.map.entry(from_state).or_insert_with(HashMap::new);

            match sub_map.entry(read) {
                Entry::Occupied(_) => panic!("Duplicate entry {:?}", dbg),
                vacant => vacant.or_insert(value),
            };
        } else {
            panic!("expected vec of len 5, got {}", slice.len());
        }
    }

    pub fn from_string(mut string: String) -> (Self, State, Option<State>) {
        string = string.chars().without_comments().collect::<String>();
        let mut rules = Self::default();
        let mut start_state = None;
        let mut print_state = None;
        let start_prefix = "start_state: ";
        let print_prefix = "print_state: ";
        // line: qA, r, w, g, qB
        for line in string.lines() {
            // let mut partial = Vec::with_capacity(5);
            // for _ in 0..5 {
            //     let _: String = iter.peeking_take_while(|c| c.is_whitespace()).collect();
            //     let s: String = iter.peeking_take_while(|&c| !c.is_whitespace()).collect();
            //
            //     partial.push(s);
            // }

            // if line.is_empty() {
            //     break 'outer;
            // }

            if line.starts_with(start_prefix) {
                if start_state.is_some() {
                    panic!("duplicate \"start_state\" statement")
                }
                let val_str = line.chars().skip(start_prefix.len()).collect::<String>();
                start_state = Some(val_str);
            } else if line.starts_with(print_prefix) {
                if print_state.is_some() {
                    panic!("duplicate \"start_state\" statement")
                }
                let val_str = line.chars().skip(print_prefix.len()).collect::<String>();
                print_state = Some(val_str);
            } else if let slice @ [_, _, _, _, _] =
                line.split_whitespace().collect::<Vec<_>>().as_slice()
            {
                rules.insert_slice(slice)
            }
        }

        let start_state = {
            let next_state = rules.states.len();
            *rules
                .states
                .entry_by_left(
                    start_state.unwrap_or_else(|| panic!("missing \"start_state\" statement")),
                )
                .or_insert(next_state)
        };
        let print_state = {
            let next_state = rules.states.len();
            print_state.map(|s| *rules.states.entry_by_left(s).or_insert(next_state))
        };

        (rules, start_state, print_state)
    }

    pub fn lookup(&self, state: State, read: Symbol) -> Option<&(Symbol, Dir, State)> {
        self.map.get(&state)?.get(&read)
    }

    pub fn integrity_check(
        &self,
        start_state: State,
        opt_print_state: Option<State>,
    ) -> Tresult<(), String, String> {
        let input_states: HashSet<State> = self.map.keys().copied().collect();

        if !input_states.contains(&start_state) {
            return Tresult::Err("Input state is not a valid state".to_string());
        }

        let output_states: HashSet<State> = self
            .map
            .values()
            .flat_map(|m| m.values().map(|(_, _, o)| *o))
            .collect();

        if let Some(print_state) = opt_print_state {
            if !output_states.contains(&print_state) {
                return if start_state == print_state {
                    Tresult::Warn(format!(
                        "Print state '{}' will only be entered once at the start of execution",
                        self.get_state_name(print_state),
                    ))
                } else {
                    Tresult::Warn(format!(
                        "Print state '{}' will never be entered",
                        self.get_state_name(print_state),
                    ))
                };
            }
        }

        if input_states != output_states {
            let input_minus_output: HashSet<State> =
                input_states.difference(&output_states).copied().collect();

            if !input_minus_output.is_empty() {
                if input_minus_output.len() == 1 {
                    if input_minus_output.contains(&start_state) {
                        /* ok, start state will only ever be visited once */
                    } else {
                        return Tresult::Warn(format!(
                            "There is an unvisitable input state: {}",
                            input_minus_output.iter().next().unwrap()
                        ));
                    }
                } else {
                    return Tresult::Warn(format!(
                        "There are {} unvisitable input states: {:?}",
                        input_minus_output.len(),
                        input_minus_output,
                    ));
                }
            } else {
                let output_minus_input: HashSet<State> =
                    output_states.difference(&input_states).copied().collect();

                // todo account for halting states here
                if !output_minus_input.is_empty() {
                    let mut msg = "More output states than input states, these states will crash the machine as soon as they're reached:\n".to_string();
                    for x in output_minus_input {
                        msg.push_str(&format!("    {}", x))
                    }
                    return Tresult::Err(msg);
                }
            }
        }

        Tresult::Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::enumerator::rules::{Rules, State, Tresult};

    fn from_file(path: &str) -> (Rules, State, Option<State>) {
        let string = std::fs::read_to_string(path).unwrap();
        Rules::from_string(string)
    }

    #[test]
    fn test_print_state_entered_only_once() {
        let (rules, start_state, print_state) =
            from_file("rulesets/test/test_print_state_entered_only_once.txt");
        let tresult = rules.integrity_check(start_state, print_state);
        assert_eq!(
            tresult,
            Tresult::Warn(
                "Print state 'qP' will only be entered once at the start of execution".to_string()
            )
        );
    }

    #[test]
    fn test_print_state_never_entered() {
        let (rules, start_state, print_state) =
            from_file("rulesets/test/test_print_state_never_entered.txt");
        let tresult = rules.integrity_check(start_state, print_state);
        assert_eq!(
            tresult,
            Tresult::Warn("Print state 'qP' will never be entered".to_string())
        );
    }
}
