# Bevy Starter

This repo is a minimal starter for Bevy `0.18`

## Inspiration

-   [`bevy_space`](https://github.com/perlindgren/bevy-space)
-   [`bevy_new_2d`](https://github.com/TheBevyFlock/bevy_new_2d)
-   [`sobevy`](https://codeberg.org/doomy/sobevy)
-   [`Mischief in miniature`](https://github.com/alice-i-cecile/mischief-in-miniature)

## Setup

-   Guide: https://bevy.org/learn/quick-start/getting-started/setup/
-   Optional [`.cargo/config.toml`](.cargo/config.toml) (Linux fast link), check the file and follow instructions to install based on your OS.

## Building

You can build your game

```
cargo run
```

Dev features (`dev_native`) are enabled by default, including hot reload and dev tools.

To disable dev features for a clean build:

```
cargo run --no-default-features
```

## Web

Install:

```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Run:

```
trunk serve
```

Open: http://127.0.0.1:8080/

## Features

-   Cargo configured according to Bevy guide with build optimizations
-   [Avian](https://github.com/avianphysics/avian) physics
-   [bevy_enhanced_input](https://github.com/simgine/bevy_enhanced_input) input
-   Starter plugins with game logic in [`src/plugins/game/`](src/plugins/game/)
-   `TLDR.md` for passing to tools like [`aider`](https://aider.chat/) and others
    that helps them get more recent context from Bevy

## Packages

-   [bevy](https://github.com/bevyengine/bevy)
-   [avian](https://github.com/avianphysics/avian)
-   [bevy_enhanced_input](https://github.com/simgine/bevy_enhanced_input)

## CI/CD

-   `ci.yml` - tests, clippy, fmt (Windows/Linux/macOS)
-   `web-cd.yaml` - GitHub Pages deployment (manual trigger)
-   `build-cd.yaml` - release builds on tag push
