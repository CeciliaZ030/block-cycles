#![no_main]
use risc0_zkvm::guest::env;
harness::entrypoint!(main, tests);
pub mod mem;
pub use mem::*;

// #[sp1_derive::cycle_tracker]
fn main() {
    println!("Hello, world!");
    let mut v = Vec::new();
    let b = 2349230;
    let mut a = 123123 + b;
    for _ in 0..10000 {
        a *= 5;
        v.push(a);
    }
    println!("{:?}", v);
}

harness::zk_suits!(
    pub mod tests {
        #[test]
        pub fn test_build_from_mock_input() {
            // Todo: impl mock input for static unit test
            assert_eq!(1, 1);
        }
    }
);
