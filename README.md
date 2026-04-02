# doobs-mpris

[![Crates.io Version](https://img.shields.io/crates/v/doobs-mpris)](https://crates.io/crates/doobs-mpris)
[![API Documentation](https://docs.rs/doobs-mpris/badge.svg)](https://docs.rs/doobs-mpris)
[![License](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![CI](https://github.com/teatwig/doobs-mpris/actions/workflows/ci.yaml/badge.svg)](https://github.com/teatwig/doobs-mpris/actions/workflows/ci.yaml)

This crate provides [zbus](github.com/z-galaxy/zbus/) bindings for the [MPRIS D-Bus Interface Specification](https://specifications.freedesktop.org/mpris/latest/).

It can be used to connect to and discover existing media players that implement the MPRIS specification, as well as adding MPRIS-support to your own media player.

The main goal is to make working with MPRIS as simple as possible by providing custom types and default implementations:

* LoopStatus, PlaybackStatus and PlaylistOrdering are exposed as enums instead of strings
* durations (playback position and seeked amount) are exposed as `SignedDuration`s instead of microseconds, so you can easily convert them to and from your desired type
* instead of directly implementing the zbus interfaces there is a `Provider` trait for each D-Bus interface
  * these contain default implementations for all D-Bus methods and properties that aren't necessary for a read-only media player to make it easy to get started quickly

## Connecting to an existing player

```rust
let conn = zbus::Connection::session().await?;
let proxy = PlayerProxy::new(&conn, "org.mpris.MediaPlayer2.some_mpris_player").await?;
proxy.play_pause().await?;
```

There are also [examples](./examples/) for automatically discovering players.

## Creating a new player

See the [player example](./examples/player.rs).

## Acknowledgements

This started as a fork of the [MPRIS bindings from Pop!_OS](https://github.com/pop-os/dbus-settings-bindings/tree/main/mpris2).

## License

Licensed under the [Mozilla Public License Version 2.0](./LICENSE.md).
