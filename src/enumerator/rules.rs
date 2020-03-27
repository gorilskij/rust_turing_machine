use std::collections::HashMap;
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
}