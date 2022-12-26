# Basic Bevy Template

This is a simple template repo for creating [Bevy](https://bevyengine.org/) projects in Rust. It contains only what I consider to be the absolute essentials for getting started.

## Features

- [Bevy](https://bevyengine.org/) and [bevy-inspector-egui](https://crates.io/crates/bevy-inspector-egui) pre-added to Cargo.toml
- Bevy's recommended [performance optimisations](https://bevyengine.org/learn/book/getting-started/setup/#compile-with-performance-optimizations)
- A blank-slate game window, pre-configured with the inspector plugin and a 3D or 2D camera

## Requirements

Make sure you have [cargo-generate](https://crates.io/crates/cargo-generate) installed. You can do this by running:

```bash
cargo install cargo-generate
```

## Usage

To create a new project in a new directory, use:

```bash
cargo generate https://github.com/Teg64/Basic-Bevy-Template
```

Alternatively, to apply the template to an existing directory, use:

```bash
cargo generate --init https://github.com/Teg64/Basic-Bevy-Template
```

## Streamlining the process

If you frequently use this template (or any others for that matter), you may want to add it to your favorites and default values.

To do this, add the following to `$CARGO_HOME/cargo-generate.toml`:

```toml
[values]
gh-username = "<YOUR GITHUB USERNAME HERE>"

[favorites.bevy]
git = "https://github.com/Teg64/Basic-Bevy-Template"
```

After doing this, you can apply the template using `cargo generate bevy`. You will then only need to specify your project name and whether you want to use the 2D or 3D template.

Optionally, you can also add `camera-type = "2D"` or `camera-type = "3D"` under the `values` section, if you only work with one of 2D or 3D.

For more details, see the [Cargo Generate Documentation](https://cargo-generate.github.io/cargo-generate/favorites.html).

### Can't find `$CARGO_HOME`?

By default, `$CARGO_HOME` is located at `$HOME/.cargo`. `cargo-generate` will look in this folder by default, so if it doesn't already exist, you can add the `cargo-generate.toml` file to this directory manually.
