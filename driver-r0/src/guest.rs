
pub const GUEST_ELF: &[u8] = include_bytes!("../../guest-r0/target/riscv32im-risc0-zkvm-elf/release/guest");
pub const GUEST_ID: [u32; 8] = [2938486734, 2933427945, 2217151005, 3313904009, 4129134499, 3424583824, 1432306777, 780031773];
pub const GUEST_PATH: &str = r#"/home/ubuntu/block-cycles/guest-r0/target/riscv32im-risc0-zkvm-elf/release/guest"#;
