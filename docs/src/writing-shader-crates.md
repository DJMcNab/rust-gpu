# Writing Shader Crates

This is section is going to walk you through writing a shader in Rust and
setting up your shader crate.

Be aware that this project is in a very early phase, please [file an
issue](https://github.com/EmbarkStudios/rust-gpu/issues) if there's something
not working or unclear.

## Online

You can now test out and try building shaders with rust-gpu from the browser!

- [SHADERed] A shader IDE which has a lite version, which allows you to build
  and run shaders on the web.
- [Shader Playground] A playground for building and checking the output of
  shader code similar to godbolt or play.rust-lang.org.

[SHADERed]: https://shadered.org/template
[shader playground]: http://shader-playground.timjones.io/9d744d5893beb6a8f129fda50ad4aeeb

## Local Setup
There are two main ways to setup your shader project locally.

1. Using the `spirv-builder` crate.
   The `spirv-builder` is a crate designed to automate the process of building
   and linking the `rust-gpu` to be able to compile SPIR-V shaders into your
   main Rust crate.
2. Using `.cargo/config`.
   Alternatively if you're willing to do the setup yourself you can manually set
   flags in your cargo configuration to enable you to run `cargo build` in your
   shader crate.


### Using `spirv-builder`
If you're writing a bigger application and you want to integrate SPIR-V shader
crates to display, it's recommended to use `spirv-builder` in a build script.

1. Copy the [`rust-toolchain`] file to your project. (You must use the same
   version of Rust as `rust-gpu`.)
2. Reference `spirv-builder` in your Cargo.toml:
    ```toml
    [build-dependencies]
    spirv-builder = { git = "https://github.com/EmbarkStudios/rust-gpu" }
    ```
    (we currently do not publish spirv-builder on crates.io)
3. Create a `build.rs` in your project root.

#### `build.rs`
Paste the following into the `main` for your build script.

```rust,no_run
SpirvBuilder::new(path_to_shader, target)
        .print_metadata(MetadataPrintout::Full)
        .build()?;
```

The values available for the `target` parameter are available
[here](./platform-support.md).  For example, if building for vulkan 1.1, use
`"spirv-unknown-vulkan1.1"`.

The `SpirvBuilder` struct has numerous configuration options available, see
rustdoc for documentation.

#### `main.rs`
```rust,no_run
const SHADER: &[u8] = include_bytes!(env!("<shader_name>.spv"));
```

Keep in mind that by default, build-dependencies are built in debug mode. This
means that the rust-gpu compiler (`rustc_codegen_spirv`) will be built in debug
mode, and will be *incredibly* slow. You can solve this by placing this bit of
configuration in your workspace `Cargo.toml`:

```toml
# Compile build-dependencies in release mode with
# the same settings as regular dependencies.
[profile.release.build-override]
opt-level = 3
codegen-units = 16
[profile.dev.build-override]
opt-level = 3
```

Keep in mind this will optimize *all* build script dependencies as release,
which may slow down full rebuilds a bit. Please read [this
issue](https://github.com/EmbarkStudios/rust-gpu/issues/448) for more
information, there's a few important caveats to know about this.

### Using `.cargo/config`

> **Note** This method will require manually rebuilding `rust-gpu` each
  time there has been changes to the repository.

If you just want to build a shader crate, and don't need to automatically
compile the SPIR-V binary at build time, you can use `.cargo/config` to set the
necessary flags. Before you can do that however you need to do a couple of steps
first to build the compiler backend.

1. Clone the `rust-gpu` repository
3. `cargo build --release` in `rust-gpu`.

Now you should have a `librustc_codegen_spirv` dynamic library available in
`target/release`. You'll need to keep this somewhere stable that you can
reference from your shader project.

Now we need to add our `.cargo/config` file. This is a configuration file that
tells cargo how to build for SPIR-V. You need provide the target you're
compiling for (see [platform support](./platform-support.md)) and provide a path
to your built `rustc_codegen_spirv` dynamic library. We have to also provide
`-Zbuild-std`.

```toml
[build]
target = "spirv-unknown-spv1.3"
rustflags = [
   "-Zcodegen-backend=<path_to_librustc_codegen_spirv>",
   "-Zsymbol-mangling-version=v0"
]

[unstable]
build-std=["core"]
build-std-features=["compiler-builtins-mem"]
```

Now we can build our crate with cargo as normal.
```bash
cargo build
```

Now you should have `<project_name>.spv` SPIR-V file in `target/debug` that you
can give to a renderer.

[`rust-toolchain`]: https://github.com/EmbarkStudios/rust-gpu/blob/main/rust-toolchain
