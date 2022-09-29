# libryzenadj-sys

This crate provides auto-generated unsafe Rust bindings, through [bindgen](https://github.com/rust-lang/rust-bindgen/), to C functions provided by [ryzenadj](https://github.com/FlyGoat/RyzenAdj), C interface for adjusting various settings in Ryzen CPU-s.

This crate should be used by most devs through the safe bindings provieded by [libryzenadj]https://crates.io/crates/libryzenadj
# WARNING: Use at your own risk!
Adjusting values provided by that lib can lead to system instabilty/crashes or even break you hardware