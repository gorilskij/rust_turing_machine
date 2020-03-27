use std::marker::PhantomData;
use bimap::BiMap;
use std::hash::Hash;

pub enum BiEntry<'a, L, R> {
    Vacant(&'a mut BiMap<L, R>, L),
    Occupied(&'a mut BiMap<L, R>, L),
    Phantom(PhantomData<R>),
}

impl<'a, L, R> BiEntry<'a, L, R>
    where
        L: Eq + Hash,
        R: Eq + Hash,
{
    pub fn or_insert_by_left(self, value: R) -> &'a R where L: Clone {
        match self {
            BiEntry::Vacant(map, left) => {
                map.insert(left.clone(), value);
                &map.get_by_left(&left).unwrap()
            }
            BiEntry::Occupied(map, left) => {
                &map.get_by_left(&left).unwrap()
            },
            BiEntry::Phantom(..) => unreachable!(),
        }
    }
}

pub trait GetBiEntry<'a, L, R> {
    fn entry(&'a mut self, key: L) -> BiEntry<'a, L, R>;
}

impl<'a, L, R> GetBiEntry<'a, L, R> for BiMap<L, R>
    where
        L: Eq + Hash,
        R: Eq + Hash,
{
    fn entry(&'a mut self, left: L) -> BiEntry<'a, L, R> {
        match self.get_by_left(&left) {
            None => BiEntry::Vacant(self, left),
            Some(right) => BiEntry::Occupied(self, left),
        }
    }
}