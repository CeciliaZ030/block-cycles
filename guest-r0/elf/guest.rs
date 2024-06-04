
pub const GUEST_ELF: &[u8] = include_bytes!("../target/riscv32im-risc0-zkvm-elf/release/guest");
pub const GUEST_ID: [u32; 8] = [77926986, 2850513027, 2040922900, 227359998, 3116200887, 1133682119, 3644320779, 4147010277];
pub const GUEST_PATH: &str = r#"/home/ubuntu/1blk-test/guest-r0/target/riscv32im-risc0-zkvm-elf/release/guest"#;
