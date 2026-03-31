// SPDX-License-Identifier: MPL-2.0
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
