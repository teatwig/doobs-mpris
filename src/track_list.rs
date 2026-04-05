// SPDX-License-Identifier: MPL-2.0
use std::collections::BTreeMap;
use std::ops::Deref;

use zbus::Connection;
use zbus::names::OwnedBusName;

use crate::binding::TrackListProxy;
use crate::types::{Metadata, TrackId};
use crate::{Error, Result};

/// Simplified access to the track list of an MPRIS media player.
#[derive(Debug, Clone)]
pub struct TrackList {
    proxy: TrackListProxy<'static>,
}

impl TrackList {
    /// Creates a new instance of the `org.mpris.MediaPlayer2.TrackList` interface.
    // TODO this being an OwnedBusName makes it really annoying to create a player
    // (although not as bad since TrackList can be created from the MediaPlayer)
    pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
        TrackListProxy::builder(connection)
            .destination(name)?
            .build()
            .await
            .map(Self::from)
            .map_err(Error::from)
    }

    /// Adds a new track to this track list.
    pub async fn add_track<S: ToString>(
        &self,
        uri: S,
        after: TrackId,
        set_as_current: bool,
    ) -> Result<()> {
        let uri = uri.to_string();
        self.proxy
            .add_track(&uri, after, set_as_current)
            .await
            .map_err(Error::from)
    }

    /// Gets the metadata of the given tracks.
    pub async fn get_tracks_metadata<T: AsRef<[TrackId]>>(
        &self,
        tracks: T,
    ) -> Result<Vec<Metadata>> {
        self.proxy
            .get_tracks_metadata(tracks.as_ref().to_vec())
            .await
            .map_err(Error::from)
    }

    /// Goes to the specified track.
    pub async fn go_to(&self, track: TrackId) -> Result<()> {
        self.proxy.go_to(track).await.map_err(Error::from)
    }

    /// Removes the specified track.
    pub async fn remove(&self, track: TrackId) -> Result<()> {
        self.proxy.remove_track(track).await.map_err(Error::from)
    }

    /// Returns a list of all available [TrackId]s.
    pub async fn tracks(&self) -> Result<Vec<TrackId>> {
        self.proxy
            .tracks()
            .await
            .map(|x| x.into_iter().collect())
            .map_err(Error::from)
    }

    /// Returns a list of all available [TrackId]s and their associated metadata,
    /// in order.
    pub async fn detailed_tracks(&self) -> Result<BTreeMap<TrackId, Metadata>> {
        let tracks = self.tracks().await?;
        let metadata = self.get_tracks_metadata(&tracks).await?;
        Ok(tracks.into_iter().zip(metadata.into_iter()).collect())
    }
}

impl Deref for TrackList {
    type Target = TrackListProxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.proxy
    }
}

impl From<TrackListProxy<'static>> for TrackList {
    fn from(proxy: TrackListProxy<'static>) -> Self {
        Self { proxy }
    }
}
