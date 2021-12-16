# Wavepool
A game engine built in rust

It's not done yet.
___
## Features
* ECS (Entity Component System) allows for modular and nested component stacks and data-linked entity instancing
___
## Building
Wavepool uses the default Cargo build system for Rust.
### Windows
Currently, the build works best with the MSVC toolchain.

```rustup default stable-msvc```

The SDL windowing system does not build properly using the gcc build toolchain.
### Apple
The build works for both Intel and M1 systems.

Intel:

```rustup default stable-x86_64```


M1/Apple architecture:

```rustup default stable-aarch64```


Building has not been tested on nightly build toolchains, although it should work.
___
## Future Development
### General
* Implementation of Worlds/Scenes to host entities
* Read scripts (like Python) for game logic instead of Rust
* Separate builds for a headless .lib version and a GUI .exe
### Networking
* Server and Client implementation
* Automatic system to send data from entities
### Graphics:
* WebGPU implementation for cross-platform support (CUDA, Vulkan, Metal)
* Support for both 2D and 3D asset rendering
* Shader nodes
