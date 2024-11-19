# Flash Algorithm for cc23xx

This is a flash algorithm for the TI cc23xx devices. It is intended
for use with `probe-rs`, but since it follows the CMSIS-PACK standard it could
also be used with other debuggers such as PyOCD. That is not tested.

![CI](https://github.com/seanmlyons22/cc23xx-flash/actions/workflows/ci.yml/badge.svg)[![chat](https://img.shields.io/badge/chat-probe--rs%3Amatrix.org-brightgreen)](https://matrix.to/#/#probe-rs:matrix.org)

## Dependencies

Run the following requirements:

```bash
cargo install cargo-generate cargo-binutils target-gen
rustup component add llvm-tools-preview
```

Ensure you have the right toolchain:

```
rustup target add thumbv6m-none-eabi
```

## Developing the algorithm

Just run `cargo run`. It spits out the flash algo in the probe-rs YAML format and downloads it onto a target and makes a test run.
You will also be able to see RTT messages.

You can find the generated YAML in `target/definition.yaml`.

## License

This is licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
