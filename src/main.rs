#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate getset;

use turing_machine::TM;

mod turing_machine;

fn main() {
    let mut e = TM::from_file("rulesets/0n1n_enumerator_efficient.txt");

    e.run(10);
}
