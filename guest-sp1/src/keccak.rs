// use alloy_primitives::{keccak256, ChainId, TxKind, B256, U256};
#![no_main]
harness::entrypoint!(main);

#[sp1_derive::cycle_tracker]

fn main() {
    let mut v = Vec::new();
    let b = 2349230;
    let mut a = 123123 + b;
    println!("cycle-tracker-start: setup");
    for _ in 0..10000 {
        a *= 5;
        v.push(a);
    }
    println!("cycle-tracker-end: setup");
    // println!("{:?}", v);
}

#[sp1_derive::cycle_tracker]
pub fn expensive_function(x: usize) -> usize {
    let mut y = 1;
    for _ in 0..100 {
        y *= x;
        y %= 7919;
    }
    y
}
