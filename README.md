# Speedy Rust

A GitHub Action and Toolchain Mirror for ultra-fast Rust setup in CI.

This repository serves two purposes:
1. **Mirror**: Automatically mirrors the latest `stable` and `nightly` Rust toolchains (x64 Linux & Windows) into GitHub Releases.
2. **Action**: Provides a composite GitHub Action to download and install these pre-packaged toolchains in seconds.

## Usage

Use this action in your workflow to install Rust significantly faster than `rustup`, as it downloads a pre-built archive from GitHub Releases instead of resolving and downloading components individually.

### Basic Usage

```yaml
steps:
  - uses: actions/checkout@v4
  
  - name: Install Rust
    uses: shaogme/speedy-rust@main
    with:
      toolchain: stable # or 'nightly'
      
  - run: cargo build --release
```

### Inputs

| Input | Description | Default |
|-------|-------------|---------|
| `toolchain` | The Rust toolchain channel (`stable` or `nightly`). | `stable` |
| `repository` | The repository hosting the releases. Change this if you are forking. | `shaogme/speedy-rust` |

## How it works

1. A scheduled workflow (`rust-toolchain.yml`) checks for updates to Rust stable/nightly.
2. If an update is found, it downloads the toolchain, packages the sysroot, and uploads it to a GitHub Release tagged `stable` or `nightly`.
3. The `action.yml` in this repo downloads the appropriate archive for your OS, extracts it, and adds it to `PATH`.

## License

MIT / Apache-2.0
