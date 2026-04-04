// SPDX-License-Identifier: MPL-2.0

use futures_util::Stream;
use futures_util::stream::{self, StreamExt};
use zbus::Connection;
use zbus::fdo::DBusProxy;
use zbus::names::OwnedBusName;

use crate::binding::MPRIS_BUS_NAME_PREFIX;

/// Returned from [Enumerator::receive_changes] when an MPRIS player was added or removed.
#[derive(Clone, Debug)]
pub enum EnumeratorEvent {
    /// MPRIS player with the given bus name was added.
    Add(OwnedBusName),
    /// MPRIS player with the given bus name was removed.
    Remove(OwnedBusName),
}

/// Helper to list MPRIS players on D-Bus bus, and watch for addition/removal.
///
/// Uses `org.freedesktop.DBus` to watch for D-Bus client with starting with
/// `org.mpris.MediaPlayer2.`
pub struct Enumerator {
    proxy: DBusProxy<'static>,
}

impl Enumerator {
    pub async fn new(connection: &Connection) -> zbus::Result<Self> {
        Ok(Self {
            proxy: DBusProxy::builder(connection)
                .path("/org/freedesktop/DBus")?
                .build()
                .await?,
        })
    }

    /// Returns a stream that is signalled when an MPRIS client is added or removed
    pub async fn receive_changes(
        &self,
    ) -> zbus::Result<impl Stream<Item = zbus::Result<EnumeratorEvent>> + Unpin + use<>> {
        let stream = self.proxy.receive_name_owner_changed().await?;
        Ok(stream
            .filter_map(|signal| {
                Box::pin(async move {
                    let args = match signal.args() {
                        Ok(args) => args,
                        Err(err) => {
                            return Some(stream::iter(Some(Err(err)).into_iter().chain(None)));
                        }
                    };
                    if args.name().starts_with(MPRIS_BUS_NAME_PREFIX) {
                        let remove = args
                            .old_owner
                            .as_ref()
                            .map(|_| Ok(EnumeratorEvent::Remove(args.name().to_owned().into())));
                        let add = args
                            .new_owner
                            .as_ref()
                            .map(|_| Ok(EnumeratorEvent::Add(args.name().to_owned().into())));
                        Some(stream::iter(remove.into_iter().chain(add)))
                    } else {
                        None
                    }
                })
            })
            .flatten())
    }

    /// Get names of all MPRIS players currently on the bus
    pub async fn players(&self) -> zbus::Result<Vec<OwnedBusName>> {
        let mut players = self.proxy.list_names().await?;
        players.retain(|name| name.starts_with(MPRIS_BUS_NAME_PREFIX));
        Ok(players)
    }
}
