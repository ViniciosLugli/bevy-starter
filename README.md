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

If you want the extra dev features then you can toggle them:

```
cargo run --features dev
```

## Web

Install:
http://127.0.0.1:8080/

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
-   Starter plugins with game logic in `src/plugins/game.rs`
-   `TLDR.md` for passing to tools like [`aider`](https://aider.chat/) and others
    that helps them get more recent context from Bevy

## Packages

-   [bevy](https://github.com/bevyengine/bevy)
-   [avian](https://github.com/avianphysics/avian)
-   [bevy_enhanced_input](https://github.com/simgine/bevy_enhanced_input)

## Missing

-   Deployment
