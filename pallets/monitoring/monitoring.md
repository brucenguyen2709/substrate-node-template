License: Unlicense

https://docs.substrate.io/v3/runtime/benchmarking/
https://github.com/paritytech/substrate/tree/master/frame/benchmarking


./target/release/node-template benchmark \
    --chain dev \               # Configurable Chain Spec
    --execution wasm \          # Always test with Wasm
    --wasm-execution compiled \ # Always used `wasm-time`
    --pallet pallet_template \   # Select the pallet
    --extrinsic '\*' \          # Select the benchmark case name, using '*' for all
    --steps 20 \                # Number of steps across component ranges
    --repeat 10 \               # Number of times we repeat a benchmark
    --raw \                     # Optionally output raw benchmark data to stdout
    --output ./                 # Output results into a Rust file



cargo build --features runtime-benchmarks
cargo test -p pallet-monitoring --features runtime-benchmarks
