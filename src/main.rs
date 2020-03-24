#![feature(option_expect_none)]

#[macro_use] extern crate derive_more;

use std::collections::HashMap;
use regex::{Regex, Captures};
use lazy_static::lazy_static;
use std::fs;
use gag::Gag;
use enumerator::Enumerator;
use enumerator::without_comments::IntoWithoutComments;

mod enumerator;



fn test_run(enumerator: &mut Enumerator, msg: &str, iterations: usize) {
    println!("--- start ({}) <{}> ---", msg, iterations);
    let transitions = enumerator.run(iterations);
    println!("--- end ---");
    println!("Took {} transitions", transitions);
}

fn main() {
    let enumerator_rules = fs::read_to_string("rulesets/comments_test_file.txt")
        .expect("failed to read file");

    let z = enumerator_rules
        .chars()
        .without_comments()
        .map(|r| match r {
            Err(e) => panic!("Parse error: {}", e),
            Ok(c) => c,
        })
        .collect::<String>();
    println!("--start--\n{}\n--end--", z);


    // let enumerator_rules = fs::read_to_string("rulesets/0n1n_enumerator_small.txt")
    //     .expect("failed to read file");
    // let mut enumerator = Enumerator::new(enumerator_rules, "qP")
    //     .expect("failed to build enumerator");
    //
    // enumerator.run(20);

    // compare efficiency
    // let small = fs::read_to_string("rulesets/0n1n_enumerator_small.txt")
    //     .expect("failed to read file 1");
    //
    // let efficient = fs::read_to_string("rulesets/0n1n_enumerator_efficient.txt")
    //     .expect("failed to read file 2");
    //
    // let mut enumerator_small = Enumerator::new(small, "qP")
    //     .expect("failed to create enumerator 1");
    //
    // let mut enumerator_efficient = Enumerator::new(efficient, "qP")
    //     .expect("failed to create enumerator 2");
    // note both functions grow kind-of quadratically (the simple one is more easily approximated)
    //  simple and efficient are equal at (0), 1, 2, and 4, otherwise (even at 3), efficient is faster
    // println!("{:>2}: {:>4} | {:>4}", "it", "smol", "fcnt");
    // for i in 0..500 {
    //     enumerator_small.reset();
    //     enumerator_efficient.reset();
    //     let (s, e);
    //     {
    //         let _gag = Gag::stdout().unwrap();
    //         s = enumerator_small.run(i);
    //         e = enumerator_efficient.run(i);
    //     }
    //     // println!("{:>2}: {:>4} | {:>4}", i, s, e);
    //     println!("{}, {}, {}", i, s, e);
    // }
}
