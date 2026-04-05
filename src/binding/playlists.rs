// SPDX-License-Identifier: MPL-2.0
use zbus::interface;
use zbus::object_server::SignalEmitter;

use crate::types::{Playlist, PlaylistId, PlaylistOrdering};

/// Implement this trait to provide the main functionality for [Playlists].
#[allow(unused_variables)]
pub trait PlaylistsProvider {
    /// Starts playing the given playlist.
    ///
    /// Note that this must be implemented.
    /// If the media player does not allow clients to change the playlist, it should not implement this interface at all.
    ///
    /// It is up to the media player whether this completely replaces the current tracklist, or whether it is merely inserted into the tracklist and the first track starts.
    /// For example, if the media player is operating in a "jukebox" mode, it may just append the playlist to the list of upcoming tracks, and skip to the first track in the playlist.
    fn activate_playlist(
        &self,
        playlist_id: PlaylistId,
    ) -> impl Future<Output = zbus::fdo::Result<()>> + Send;

    /// Gets a set of playlists.
    fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> impl Future<Output = zbus::fdo::Result<Vec<Playlist>>> + Send;

    /// The number of playlists available.
    fn playlist_count(&self) -> impl Future<Output = zbus::fdo::Result<u32>> + Send;

    /// The available orderings.
    ///
    /// At least one must be offered.
    fn orderings(&self) -> impl Future<Output = zbus::fdo::Result<Vec<PlaylistOrdering>>> + Send;

    /// The currently-active playlist as well as a boolean indicating if it is valid.
    ///
    /// If there is no currently-active playlist, the structure's `valid` field will be false, and the [Playlist] details are undefined.
    ///
    /// Note that this may not have a value even after *ActivatePlaylist* is called with a valid [PlaylistId] as *ActivatePlaylist* implementations have the option of simply inserting the contents of the playlist into the current tracklist.
    fn active_playlist(&self) -> impl Future<Output = zbus::fdo::Result<(bool, Playlist)>> + Send;
}

/// Provides access to the media player's playlists.
///
/// It delegates the D-Bus calls to its provider.
pub struct Playlists<P>(P);

impl<P> Playlists<P>
where
    P: PlaylistsProvider + Send + Sync + 'static,
{
    /// Creates a new MPRIS player that delegates to the given [PlaylistsProvider].
    pub fn new(provider: P) -> Self {
        Self(provider)
    }

    /// The reference to the underlying [PlaylistsProvider].
    pub fn inner(&self) -> &P {
        &self.0
    }

    /// The mutable reference to the underlying [PlaylistsProvider].
    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.0
    }
}

#[interface(
    interface = "org.mpris.MediaPlayer2.Playlists",
    proxy(default_path = "/org/mpris/MediaPlayer2")
)]
impl<P> Playlists<P>
where
    P: PlaylistsProvider + Send + Sync + 'static,
{
    /// ActivatePlaylist method
    async fn activate_playlist(&self, playlist_id: PlaylistId) -> zbus::fdo::Result<()> {
        self.0.activate_playlist(playlist_id).await
    }

    /// GetPlaylists method
    async fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> zbus::fdo::Result<Vec<Playlist>> {
        self.0
            .get_playlists(index, max_count, order, reverse_order)
            .await
    }

    /// PlaylistChanged signal
    ///
    /// Indicates that either the `name` or `icon` attribute of a playlist has changed.
    ///
    /// Client implementations should be aware that this signal may not be implemented.
    #[zbus(signal)]
    async fn playlist_changed(emitter: &SignalEmitter<'_>, playlist: Playlist) -> zbus::Result<()>;

    /// PlaylistCount property
    #[zbus(property)]
    async fn playlist_count(&self) -> zbus::fdo::Result<u32> {
        self.0.playlist_count().await
    }

    /// Orderings property
    #[zbus(property)]
    async fn orderings(&self) -> zbus::fdo::Result<Vec<PlaylistOrdering>> {
        self.0.orderings().await
    }

    /// ActivePlaylist property
    #[zbus(property)]
    async fn active_playlist(&self) -> zbus::fdo::Result<(bool, Playlist)> {
        self.0.active_playlist().await
    }
}
