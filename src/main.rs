#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate getset;

use enumerator::Enumerator;

mod enumerator;

fn main() {
    let mut e = Enumerator::from_file("rulesets/0n1n_enumerator_efficient.txt");

    e.run(10);
}
