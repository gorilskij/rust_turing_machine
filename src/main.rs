#[macro_use] extern crate derive_more;
#[macro_use] extern crate getset;

use std::fs;
use enumerator::Enumerator;

mod enumerator;

fn main() {
    let rules = fs::read_to_string("rulesets/0n1n_enumerator_small.txt").unwrap();
    let mut e = Enumerator::from_string(
        rules,
        "qP".to_owned(),
        "qP".to_owned()
    );

    e.run(10);
}
