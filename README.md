# geode-rs

Rust Bindings for the [Geode SDK](https://geode-sdk.org)

## Usage

TODO
TLDR: define entry with
```rs
geode::entry! {
/* your code goes here */
} 
```
and do hooks n stuff in there

## Compilation

Due to the fact that Geometry Dash is a 32-bit application, mods for it have to be compiled as 32-bit.
You can do this by running `cargo +nightly-i686-pc-windows-msvc build` on Windows (nightly is required because geode-rs uses some unstable features).
If you are not using Windows I recommend using [cargo xwin](https://github.com/rust-cross/cargo-xwin) for cross compilation targeting Windows.
Once you have your DLL you will have to add it to a ZIP file that contains a mod.json, logo.png and about.md.

## License

BSL-1.0
