## Build and test

Please make sure Rust is [installed](https://www.rust-lang.org/learn/get-started).

**Basic**

The following commands only build the Fe -> Yul compiler components.

- build the CLI: `cargo build`
- test: `cargo test --workspace`

**Full**

The Fe compiler depends on the Solidity compiler for transforming Yul IR to EVM bytecode. We currently use [solc-rust](https://github.com/fe-lang/solc-rust) to perform this. In order to compile solc-rust, the following must be installed on your system:

- cmake
- boost(1.65+)
- libclang


### Installing Boost

You'll need at least Boost 1.65+ to compile solc-rust. Most package managers should have this version, so you can likely do something like the following.

```bash
# Macos
brew install boost

# Ubuntu
sudo apt install libboost-all-dev
```

If your package manager has an outdated version, you could instead download a distribution from SourceForge and install it manually. See the boost documentation [here for more](https://www.boost.org/doc/libs/1_85_0/more/getting_started/unix-variants.html).

Once these have been installed, you may run the full build. This is enabled using the *solc-backend* feature.

- build the CLI: `cargo build --features solc-backend`
- test: `cargo test --workspace --features solc-backend`

## Running the CLI

Once you have everything built, you'll likely want to give `fe` a spin! From the workspace root, you can install `fe` like so, and try running `fe --help` to make sure everything worked.

```bash
cargo install --path crates/fe --locked
fe --help
```


