// SPDX-License-Identifier: MPL-2.0

//! Bindings for all the MPRIS interfaces (Version 2.2).
//!
//! Parts of the documentation were adapted from the
//! [MPRIS D-Bus Interface Specification][mpris-spec],
//! licensed under the LGPL 2.1 (or later).
//!
//! [mpris-spec]: https://specifications.freedesktop.org/mpris/latest/index.html

mod media_player;
pub use media_player::*;

mod player;
pub use player::*;

mod playlists;
pub use playlists::*;

mod track_list;
pub use track_list::*;
use zbus::names::OwnedWellKnownName;

/// All D-Bus connections implementing the MPRIS specification *must* request a unique bus name
/// starting with this prefix.
pub const MPRIS_BUS_NAME_PREFIX: &str = "org.mpris.MediaPlayer2.";

/// D-Bus object path that all MPRIS interfaces should be served at.
pub const MPRIS_OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";

/// Creates a new bus name that is prefixed with the [MPRIS_BUS_NAME_PREFIX].
///
/// This allows clients like [Enumerator](crate::Enumerator) to list available media players.
pub fn create_mpris_bus_name(player_name: &str) -> zbus::names::Result<OwnedWellKnownName> {
    OwnedWellKnownName::try_from(format!("{MPRIS_BUS_NAME_PREFIX}{player_name}"))
}
