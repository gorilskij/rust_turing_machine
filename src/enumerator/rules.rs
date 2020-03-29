use std::collections::{HashMap, HashSet};
use crate::enumerator::Dir;
use std::collections::hash_map::Entry;
use std::ops::Deref;
use bimap::BiMap;
use super::bientry::GetBiEntry;

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

pub enum Tresult<T, E, W> {
    Ok(T),
    Err(E),
    Warn(W),
}

// todo warn about inconsistent direction notation (</l/L and >/r/R)
impl Rules {
    pub fn insert(
        &mut self,
        from_state: String,
        read: String,
        write: String,
        go: String,
        to_state: String,
    ) {
        let dbg = (from_state.clone(), read.clone());


        let from_state = {
            let new_state_idx = self.states.len();
            *self.states
                .entry(from_state)
                .or_insert_by_left(new_state_idx)
        };

        let to_state = {
            let new_state_idx = self.states.len();
            *self.states
                .entry(to_state)
                .or_insert_by_left(new_state_idx)
        };

        let read = if &read == "_" {
            0
        } else {
            let new_symbol_idx = self.symbols.len() + 1;
            *self.symbols
                .entry(read)
                .or_insert_by_left(new_symbol_idx)
        };

        let write = if &write == "_" {
            0
        } else {
            let new_symbol_idx = self.symbols.len() + 1;
            *self.symbols
                .entry(write)
                .or_insert_by_left(new_symbol_idx)
        };


        let value = (write, go.parse().unwrap(), to_state);

        let sub_map = self.map
            .entry(from_state)
            .or_insert_with(HashMap::new);

        match sub_map.entry(read) {
            Entry::Occupied(_) => panic!("Duplicate entry {:?}", dbg),
            vacant => vacant.or_insert(value),
        };
    }

    pub fn insert_vec(&mut self, vec: &mut Vec<String>) {
        // when drain is dropped, it silently removes all remaining elements from the vector
        let mut drain = vec.drain(..);
        self.insert(
            drain.next().unwrap(),
            drain.next().unwrap(),
            drain.next().unwrap(),
            drain.next().unwrap(),
            drain.next().unwrap(),
        );
    }

    pub fn lookup(&self, state: State, read: Symbol) -> Option<&(Symbol, Dir, State)> {
        self.map.get(&state)?.get(&read)
    }

    pub fn integrity_check(
        &self,
        start_state: State,
        print_state: State
    ) -> Tresult<(), String, String> {
        let input_states: HashSet<State> = self.map
            .keys()
            .copied()
            .collect();

        if !input_states.contains(&start_state) {
            return Tresult::Err("Input state is not a valid state".to_string())
        }

        let output_states: HashSet<State> = self.map
            .values()
            .flat_map(|m| m
                .values()
                .map(|(_, _, o)| *o))
            .collect();

        if !output_states.contains(&print_state) {
            return if start_state == print_state {
                Tresult::Warn(format!(
                    "Print state '{}' will only be entered once at the start of execution",
                    print_state
                ))
            } else {
                Tresult::Warn(format!("Print state '{}' will never be entered", print_state))
            }
        }

        if input_states != output_states {
            let input_minus_output: HashSet<State> = input_states
                .difference(&output_states)
                .copied()
                .collect();

            if !input_minus_output.is_empty() {
                if input_minus_output.len() == 1 {
                    if input_minus_output.contains(&start_state) {
                        /* ok, start state will only ever be visited once */
                    } else {
                        return Tresult::Warn(format!(
                            "There is an unvisitable input state: {}",
                            input_minus_output.iter().next().unwrap()
                        ))
                    }
                } else {
                    return Tresult::Warn(format!(
                        "There are {} unvisitable input states: {:?}",
                        input_minus_output.len(),
                        input_minus_output,
                    ))
                }
            } else {
                let output_minus_input: HashSet<State> = output_states
                    .difference(&input_states)
                    .copied()
                    .collect();

                // todo account for halting states here
                if !output_minus_input.is_empty() {
                    return Tresult::Err(format!(
                        "More output states than input states, these states will crash the \
                        machine as soon as they're reached: {:?}", output_minus_input
                    ))
                }
            }
        }

        Tresult::Ok(())
    }
}