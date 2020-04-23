use bimap::BiMap;
use std::hash::Hash;

/// A very limited implementation of the entry API for bimap::BiMap.
pub enum LEntry<'a, L, R> {
    Vacant(&'a mut BiMap<L, R>, L),
    Occupied(&'a mut BiMap<L, R>, L),
}

impl<'a, L, R> LEntry<'a, L, R>
where
    L: Eq + Hash,
    R: Eq + Hash,
{
    pub fn or_insert(self, value: R) -> &'a R
    where
        L: Clone,
    {
        match self {
            LEntry::Vacant(map, left) => {
                map.insert(left.clone(), value);
                &map.get_by_left(&left).unwrap()
            }
            LEntry::Occupied(map, left) => &map.get_by_left(&left).unwrap(),
        }
    }
}

pub trait GetEntry<'a, L, R> {
    fn entry_by_left(&'a mut self, key: L) -> LEntry<'a, L, R>;
}

impl<'a, L, R> GetEntry<'a, L, R> for BiMap<L, R>
where
    L: Eq + Hash,
    R: Eq + Hash,
{
    fn entry_by_left(&'a mut self, left: L) -> LEntry<'a, L, R> {
        match self.get_by_left(&left) {
            None => LEntry::Vacant(self, left),
            Some(_) => LEntry::Occupied(self, left), // keeping right would be efficient but dangerous
        }
    }
}
