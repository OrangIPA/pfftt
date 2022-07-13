# Pfftt
A platformer game written in Rust Programming Language with Bevy Engine.

## Build
To build this, you need to have rust installed. If you doesn't have rust installed, go to [Rust Official Website](https://rust-lang.org/) for installation instruction  
If you already have rust installed, you can use 
```
cargo build
```
to build Pfftt or you can build and run with
```
cargo run
```
Alternatively, you can use bevy dynamic linking features with
```
cargo [run|build] --features bevy/dynamic
```
However it's not recommended to use this feature in the final build.
To build the release version, use
```
cargo build --release
```