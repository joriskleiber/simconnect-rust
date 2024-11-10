![crates.io](https://img.shields.io/crates/v/simconnect)

# SimConnect Bindings for Rust

## Requirements

- [CLang](https://clang.llvm.org/get_started.html) (See the [Rust Bindgen Documentation](https://rust-lang.github.io/rust-bindgen/requirements.html))
- MSVC x64 Rust build (`x86_64-pc-windows-msvc`, see [The rustup book](https://rust-lang.github.io/rustup/installation/windows.html))
- [MSFS SDK](https://docs.flightsimulator.com/html/Introduction/SDK_Overview.htm)

## Using

Add this to your `Cargo.toml`

```toml
[dependencies]
simconnect = "0.4.0"
```

## Building

1. run `cargo build`
2. Add `use simconnect` at the top of your file

## Example

Read float position data

```
cargo run --example aircraft_updates
```

Requests tagged data with thresholds from SimConnect and reads floats/strings

```
cargo run --example aircraft_updates_on_change
```

_You must have the MSFS SDK installed_

### Remarks

Not every single function from the api has been tested. If you find an error, feel free to make an issue or a pull request.
