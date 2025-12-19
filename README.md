# Ugaris Rust Demo Mod

A native mod written in Rust demonstrating the Ugaris Client mod API. This showcases Rust's memory safety, zero-cost abstractions, and excellent cross-platform support.

## Features

- **Memory Safe** - No undefined behavior, guaranteed by Rust's ownership system
- **Zero-Cost Abstractions** - High-level code with low-level performance
- **Cross-Platform** - Single codebase for Windows, macOS (Universal), and Linux

## Commands

| Command | Description |
|---------|-------------|
| `#hello` | Display available commands |
| `#stats` | Show current player stats |
| `#overlay` | Toggle the HUD overlay |

## Installation

### Via Ugaris Launcher

1. Open the Ugaris Launcher
2. Go to **Options > Developer > Enable Mod Manager**
3. Navigate to the **Mods** section
4. Click **Install from URL**
5. Enter: `ugaris/ugaris-rust-demo-mod`

## Building from Source

### Requirements

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build Commands

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Cross-compile for specific target
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
```

### Output Locations

- Windows: `target/release/bmod.dll`
- macOS: `target/release/libbmod.dylib` (rename to `bmod.dylib`)
- Linux: `target/release/libbmod.so` (rename to `bmod.so`)

## Project Structure

```
ugaris-rust-demo-mod/
├── .github/workflows/build.yml  # CI/CD pipeline
├── src/
│   └── lib.rs                   # Main mod implementation
├── Cargo.toml                   # Rust package manifest
├── mod.json                     # Mod metadata
├── README.md
└── LICENSE
```

## Rust-Specific Considerations

### FFI Safety

The mod uses `extern "C"` functions to interface with the C client:

```rust
#[no_mangle]
pub extern "C" fn amod_version() -> *const c_char {
    cstr!("Rust Demo Mod 1.0.0")
}
```

### String Handling

C strings require null termination. Use the `cstr!` macro or manual formatting:

```rust
// Static string
cstr!("Hello World")

// Dynamic string (must include \0)
let text = format!("HP: {}\0", hp);
```

### Global State

Use atomic types for thread-safe global state:

```rust
static SHOW_OVERLAY: AtomicBool = AtomicBool::new(false);
```

## Why Rust for Mods?

1. **Safety** - No buffer overflows, use-after-free, or data races
2. **Performance** - Compiles to native code, no garbage collection
3. **Tooling** - Cargo makes dependency management and building easy
4. **Cross-Platform** - Excellent support for all target platforms

## License

MIT License - See [LICENSE](LICENSE) file for details.
