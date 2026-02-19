# Speedy Rust

A GitHub Action and Toolchain Mirror for ultra-fast Rust setup in CI.

This repository serves two purposes:
1. **Mirror**: Automatically mirrors the latest `stable` and `nightly` Rust toolchains into GitHub Releases.
2. **Action**: Provides a composite GitHub Action to download and install these pre-packaged toolchains in seconds.

## Supported Platforms

| OS | Architecture |
|----|--------------|
| Linux | x64, ARM64 |
| Windows | x64, ARM64 |
| macOS | x64 (Intel), ARM64 (Apple Silicon) |

## Usage

Use this action in your workflow to install Rust significantly faster than `rustup`, as it downloads a pre-built archive from GitHub Releases instead of resolving and downloading components individually.

### Basic Usage

```yaml
steps:
  - uses: actions/checkout@v6
  
  - name: Install Rust
    uses: shaogme/speedy-rust@main
    with:
      toolchain: stable # or 'nightly'
      
  - run: cargo build --release
```

### With Components

You can optionally install additional components like `rustfmt`, `clippy`, `rust-src`, or `rust-docs`.

```yaml
steps:
  - uses: actions/checkout@v6
  
  - name: Install Rust with Components
    uses: shaogme/speedy-rust@main
    with:
      toolchain: nightly
      components: rustfmt, clippy
      
  - run: cargo fmt --check
  - run: cargo clippy
```

### Inputs

| Input | Description | Default |
|-------|-------------|---------|
| `toolchain` | The Rust toolchain channel (`stable` or `nightly`). | `stable` |
| `components` | Comma-separated list of extra components to install (e.g., `rustfmt`, `clippy`, `rust-src`, `rust-docs`). | *None* |
| `repository` | The repository hosting the releases. Change this if you are forking. | `shaogme/speedy-rust` |

## How it works

1. A scheduled workflow (`rust-toolchain.yml`) checks for updates to Rust stable/nightly.
2. If an update is found, it downloads the toolchain, packages the sysroot, and uploads it to a GitHub Release tagged `stable` or `nightly`.
3. The `action.yml` in this repo downloads the appropriate archive for your OS and Architecture, extracts it, and adds it to `PATH`.

## License

MIT / Apache-2.0
