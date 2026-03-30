# doobs-mpris

A zbus proxy and interface for [org.mpris.MediaPlayer2](https://mpris2.readthedocs.io/en/latest/).

This is based on the [MPRIS bindings from Pop!_OS](https://github.com/pop-os/dbus-settings-bindings/tree/main/mpris2).

## Connecting to an existing player

```rust
let conn = zbus::Connection::session().await?;
let proxy = PlayerProxy::new(&conn, "org.mpris.MediaPlayer2.some_mpris_player").await?;
proxy.play_pause().await?;
```

There are also [examples](./examples/) for automatically discovering players.

## Creating a new player

See the [player example](./examples/player.rs).

## License

Licensed under the [Mozilla Public License Version 2.0](./LICENSE.md).
