// SPDX-License-Identifier: MPL-2.0
use zbus::interface;
use zbus::object_server::SignalEmitter;

use crate::types::{Metadata, TrackId};

/// Implement this trait to provide the main functionality for a [TrackList].
#[allow(unused_variables)]
pub trait TrackListProvider {
    /// Gets all the metadata available for a set of tracks.
    ///
    /// Each set of metadata must have a `mpris:trackid` entry at the very least, which contains a string that uniquely identifies this track within the scope of the tracklist.
    fn get_tracks_metadata(
        &self,
        track_ids: Vec<TrackId>,
    ) -> impl Future<Output = zbus::fdo::Result<Vec<Metadata>>> + Send;

    /// Adds a URI in the TrackList.
    ///
    /// If the *CanEditTracks* property is false, this has no effect.
    ///
    /// Note: Clients should not assume that the track has been added at the time when this method returns.
    /// They should wait for a *TrackAdded* (or *TrackListReplaced*) signal.
    fn add_track(
        &self,
        uri: &str,
        after_track: TrackId,
        set_as_current: bool,
    ) -> impl Future<Output = zbus::fdo::Result<()>> + Send;

    /// Removes an item from the TrackList.
    ///
    /// If the track is not part of this tracklist, this has no effect.
    ///
    /// If the *CanEditTracks* property is false, this has no effect.
    ///
    /// Note: Clients should not assume that the track has been removed at the time when this method returns.
    /// They should wait for a *TrackRemoved* (or *TrackListReplaced*) signal.
    fn remove_track(&self, track_id: TrackId)
    -> impl Future<Output = zbus::fdo::Result<()>> + Send;

    /// Skip to the specified TrackId.
    ///
    /// If the track is not part of this tracklist, this has no effect.
    ///
    /// If this object is not `/org/mpris/MediaPlayer2`, the current TrackList's tracks should be replaced with the contents of this TrackList, and the *TrackListReplaced* signal should be fired from `/org/mpris/MediaPlayer2`.
    fn go_to(&self, track_id: TrackId) -> impl Future<Output = zbus::fdo::Result<()>> + Send;

    /// An array which contains the identifier of each track in the tracklist, in order.
    ///
    /// The *org.freedesktop.DBus.Properties.PropertiesChanged* signal is emitted every time this property changes, but the signal message does not contain the new value.
    /// Client implementations should rather rely on the *TrackAdded*, *TrackRemoved* and *TrackListReplaced* signals to keep their representation of the tracklist up to date.
    fn tracks(&self) -> impl Future<Output = zbus::fdo::Result<Vec<TrackId>>> + Send;

    /// Whether the client can edit the track list.
    ///
    /// If `false`, calling *AddTrack* or *RemoveTrack* will have no effect, and may raise a [zbus::fdo::Error::NotSupported] error.
    fn can_edit_tracks(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send;
}

/// Provides access to a short list of tracks which were recently played or will be played shortly.
///
/// This is intended to provide context to the currently-playing track, rather than giving complete access to the media player's playlist.
///
/// It delegates the D-Bus calls to its provider.
pub struct TrackList<P>(P);

impl<P> TrackList<P>
where
    P: TrackListProvider + Send + Sync + 'static,
{
    /// Creates a new MPRIS player that delegates to the given [TrackListProvider].
    pub fn new(provider: P) -> Self {
        Self(provider)
    }

    /// The reference to the underlying [TrackListProvider].
    pub fn inner(&self) -> &P {
        &self.0
    }

    /// The mutable reference to the underlying [TrackListProvider].
    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.0
    }
}

#[interface(
    interface = "org.mpris.MediaPlayer2.TrackList",
    proxy(default_path = "/org/mpris/MediaPlayer2")
)]
impl<P> TrackList<P>
where
    P: TrackListProvider + Send + Sync + 'static,
{
    /// GetTracksMetadata method
    async fn get_tracks_metadata(
        &self,
        track_ids: Vec<TrackId>,
    ) -> zbus::fdo::Result<Vec<Metadata>> {
        self.0.get_tracks_metadata(track_ids).await
    }

    /// AddTrack method
    async fn add_track(
        &self,
        uri: &str,
        after_track: TrackId,
        set_as_current: bool,
    ) -> zbus::fdo::Result<()> {
        self.0.add_track(uri, after_track, set_as_current).await
    }

    /// RemoveTrack method
    async fn remove_track(&self, track_id: TrackId) -> zbus::fdo::Result<()> {
        self.0.remove_track(track_id).await
    }

    /// GoTo method
    async fn go_to(&self, track_id: TrackId) -> zbus::fdo::Result<()> {
        self.0.go_to(track_id).await
    }

    /// TrackListReplaced signal
    ///
    /// Indicates that the entire tracklist has been replaced.
    ///
    /// It is left up to the implementation to decide when a change to the track list is invasive enough that this signal should be emitted instead of a series of *TrackAdded* and *TrackRemoved* signals.
    ///
    /// `/org/mpris/MediaPlayer2/TrackList/NoTrack` indicates that there is no current track.
    #[zbus(signal)]
    async fn track_list_replaced(
        emitter: &SignalEmitter<'_>,
        tracks: Vec<TrackId>,
        current_track: TrackId,
    ) -> zbus::Result<()>;

    /// TrackAdded signal
    ///
    /// Indicates that a track has been added to the track list.
    #[zbus(signal)]
    async fn track_added(
        emitter: &SignalEmitter<'_>,
        metadata: Metadata,
        after_track: TrackId,
    ) -> zbus::Result<()>;

    /// TrackRemoved signal
    ///
    /// Indicates that a track has been removed from the track list.
    #[zbus(signal)]
    async fn track_removed(emitter: &SignalEmitter<'_>, track: TrackId) -> zbus::Result<()>;

    /// TrackMetadataChanged signal
    ///
    /// Indicates that the metadata of a track in the tracklist has changed.
    ///
    /// This may indicate that a track has been replaced, in which case the `mpris:trackid` metadata entry is different from the `track` argument.
    #[zbus(signal)]
    async fn track_metadata_changed(
        emitter: &SignalEmitter<'_>,
        track: TrackId,
        metadata: Metadata,
    ) -> zbus::Result<()>;

    /// Tracks property
    #[zbus(property)]
    async fn tracks(&self) -> zbus::fdo::Result<Vec<TrackId>> {
        self.0.tracks().await
    }

    /// CanEditTracks property
    #[zbus(property)]
    async fn can_edit_tracks(&self) -> zbus::fdo::Result<bool> {
        self.0.can_edit_tracks().await
    }
}
