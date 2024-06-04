## Benchmark Block Cycles
Make sure the guest targets are clean, then update the dependencies since there are multiple patches.
```sh
cd guest-sp1
cargo clean
cargo update
```
Build the elf
```sh
cd driver-sp1
cargo run --bin builder
```
Run the elf
- Mock mode
- Debug print to see cycle counter

```sh
SP1_PROVER=mock RUST_LOG=debug cargo run --bin driver --release -- [name-of-input]
```
For the input of guest, you need for example `mainnet-1234.json` serialized by Raiko native prover, then place this file in `/driver-sp1`. In this case your `[name-of-input]` is `mainnet-1234`.
## Results
The results are being updated since this is a temporary test crate, currently the data (i.e. `mainnet-1234.json`) contains the cycle breakdown for 
- initialize_database
- execute_transactions
  for each tx:
  - signature_hash
  - recover_address_from_prehash
  - evm.transact() => precompiles
    - BnAdd & Mul
    - Blake2
    - Sha256
    - Modexp
    - Ecrecover
    - Kzg
      
which are all placed in [data](https://github.com/CeciliaZ030/block-cycles/tree/master/data)
