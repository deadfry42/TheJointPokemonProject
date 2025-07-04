# TheJointPokemonProject
- A fan-made pokemon fangame, made by a small group of people.
- Written in Rust, using rust-sdl2. (🚀🚀🚀 memory safe 🚀🚀🚀)

> [!WARNING]
> I am learning Rust alongside the development of this project.
> If you see any abnormalities, or just bad code / practices, feel free to open an issue or pr, and correct me!

## compile instructions
##### (note: this has only really been tested on unix-based operating systems. for NT-based (windows), you're on your own.)
1. Requirements
   - [Rust (Rustup)](https://rustup.rs/)
   - [SDL2](https://wiki.libsdl.org/SDL2/Installation) (install with your package manager if possible)
2. Compilation
   - Make sure you have all dependencies installed.
   - If you have [just](https://github.com/casey/just) installed, run `just build`
   - otherwise, run `cargo build --release`
3. Running
   - The binary will be placed in [git repo]/target/release/
   - You can run via `./target/release/tjpp` on unix, `.\target\release\tjpp` on nt (windows)

## cross compilation instructions
##### (note: this has not been properly tested.)
> [!WARNING]
> Compiling __REQUIRES__ SDL2 to be present. This may make Cross Compiling difficult.
1. Requirements
   - [just](https://github.com/casey/just)
   - [Rust](https://rustup.rs/) (Rustup)
   - [Cross.rs](https://github.com/cross-rs/cross) (cargo pkg)
   - [Podman](https://podman.io/) (which requires qemu)
2. Compilation
   - Make sure Cross.rs is installed, you can do this by running `just install_cross`
   - Make sure podman is setup, on macos `just install_cross` automatically sets up podman for you.
   - TODO: Figure out how to install >=SDL2-2.0.36 on Cross container
   - Run the respective just recipe for each platform
     - x86_64-pc-windows-gnu -> `just windows`
     - aarch64-pc-windows-msvc -> `just windows_aarch64`¹ ²
     - x86_64-unknown-linux-gnu -> `just linux`
     - aarch64-unknown-linux-gnu -> `just linux_aarch64`
     - x86_64-apple-darwin -> `just macos` ¹ ² ³
     - aarch64-apple-darwin -> `just macos_aarch64` ¹ ³
   - ¹ This platform (for cross.rs) requires a custom image to work because they cannot ship pre-built images for this platform
   - ² This platform isn't supported by this project, because i can't get them to compile :p
   - ³ This platforms requires extracting the macOS SDK, due to licensing reasons. You must agree to the XCode and Apple SDKs agreement to cross compile for this platform.
3. Running
   - You likely won't be able to run these cross compiled binaries on your current device (with the exception of using emulators, compatibility layers eg. wine, fex etc.)
   - Transfer the contents of [git repo]/target/[platform]/ to another device that supports that target, and attempt to run the program on that device.
