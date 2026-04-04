// SPDX-License-Identifier: MPL-2.0

//! This crate provides [zbus](https://github.com/z-galaxy/zbus/) bindings for the [MPRIS D-Bus Interface Specification](https://specifications.freedesktop.org/mpris/latest/).
//!
//! It can be used to connect to and discover existing media players that implement the MPRIS specification, as well as adding MPRIS-support to your own media player.
//!
//! The main goal is to make working with MPRIS as simple as possible by providing custom types and default implementations:
//!
//! * LoopStatus, PlaybackStatus and PlaylistOrdering are exposed as enums instead of strings
//! * durations (playback position and seeked amount) are exposed as `SignedDuration`s instead of microseconds, so you can easily convert them to and from your desired type
//! * instead of directly implementing the zbus interfaces there is a `Provider` trait for each D-Bus interface
//!   * these contain default implementations for all D-Bus methods and properties that aren't necessary for a read-only media player to make it easy to get started quickly
//!
//! # Connecting to an existing player
//!
//! ```no_run
//! # use doobs_mpris::binding::PlayerProxy;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let conn = zbus::Connection::session().await?;
//! let proxy = PlayerProxy::new(&conn, "org.mpris.MediaPlayer2.some_mpris_player").await?;
//! proxy.play_pause().await?;
//! # Ok(())
//! # }
//! ```
//!
//! There are also [examples](https://github.com/teatwig/doobs-mpris/tree/main/examples/) for automatically discovering players.
//!
//! # Creating a new player
//!
//! See the [player example](https://github.com/teatwig/doobs-mpris/tree/main/examples/player.rs).

pub mod binding;

mod enumerator;
pub use enumerator::*;

mod error;
pub use error::*;

mod media_player;
pub use media_player::*;

mod player;
pub use player::*;

mod playlists;
pub use playlists::*;

mod track_list;
pub use track_list::*;

pub mod types;

pub(crate) fn handle_optional<T>(input: zbus::Result<T>) -> error::Result<Option<T>> {
    match input {
        Ok(input) => Ok(Some(input)),
        Err(zbus::Error::FDO(fdo_error))
            if matches!(*fdo_error, zbus::fdo::Error::NotSupported(_)) =>
        {
            Ok(None)
        }
        Err(err) => Err(error::Error::from(err)),
    }
}
