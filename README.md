# TheJointPokemonProject
- A fan-made pokemon fangame, made by 3 people.
- Written in Rust, compilable for:
  - x86_64-pc-windows-gnu
  - aarch64-pc-windows-msvc
  - x86_64-unknown-linux-gnu
  - aarch64-unknown-linux-gnu
  - x86_64-apple-darwin
  - aarch64-apple-darwin

## compile instructions
##### this has only really been tested on unix-based operating systems. for NT-based (windows), you're on your own.
1. Requirements
   - Rust (obviously), get [Rustup](https://rustup.rs/) for this
2. Compilation
   - If you have [just](https://github.com/casey/just) installed, run `just build`
   - otherwise, run `cargo build --release`
3. Running
   - The binary will be placed in [git repo]/target/release/
   - You can run via `./target/release/tjpp` on unix, `.\target\release\tjpp` on nt (windows)
