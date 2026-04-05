// SPDX-License-Identifier: MPL-2.0
use std::future::Future;

use zbus::interface;
use zbus::object_server::SignalEmitter;

use crate::types::{LoopStatus, Metadata, MprisDuration, PlaybackStatus, TrackId};

/// Implement this trait to provide the functionality of [Player].
///
/// Most functions have a default implementation to make it easier to create a simple player that
/// only exposes the current playback status.
/// When implementing additional functions make sure read the MPRIS specification.
#[allow(unused_variables)]
pub trait PlayerProvider {
    /// Skips to the next track in the tracklist.
    ///
    /// If there is no next track (and endless playback and track repeat are both off), stop playback.
    ///
    /// If playback is paused or stopped, it remains that way.
    ///
    /// If *CanGoNext* is `false`, attempting to call this method should have no effect.
    fn next(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Skips to the previous track in the tracklist.
    ///
    /// If there is no previous track (and endless playback and track repeat are both off), stop playback.
    ///
    /// If playback is paused or stopped, it remains that way.
    ///
    /// If *CanGoPrevious* is `false`, attempting to call this method should have no effect.
    fn previous(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Pauses playback.
    ///
    /// If playback is already paused, this has no effect.
    ///
    /// Calling *Play* after this should cause playback to start again from the same position.
    ///
    /// If *CanPause* is `false`, attempting to call this method should have no effect.
    fn pause(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Toggles the payback state.
    ///
    /// If playback is already paused, resumes playback.
    ///
    /// If playback is stopped, starts playback.
    ///
    /// If *CanPause* is `false`, attempting to call this method should have no effect and raise an error.
    fn play_pause(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Stops playback.
    ///
    /// If playback is already stopped, this has no effect.
    ///
    /// Calling *Play* after this should cause playback to start again from the beginning of the track.
    ///
    /// If *CanControl* is `false`, attempting to call this method should have no effect and raise an error.
    fn stop(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Starts or resumes playback.
    ///
    /// If already playing, this has no effect.
    ///
    /// If paused, playback resumes from the current position.
    ///
    /// If there is no track to play, this has no effect.
    ///
    /// If *CanPlay* is `false`, attempting to call this method should have no effect.
    fn play(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    // Seeks forward in the current track by the specified duration.
    //
    // A negative value seeks back.
    // If this would mean seeking back further than the start of the track, the position is set to `0`.
    //
    // If the value passed in would mean seeking beyond the end of the track, acts like a call to Next.
    //
    // If the *CanSeek* property is `false`, this has no effect.
    fn seek(&self, offset: MprisDuration) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Sets the current track position.
    ///
    /// If the `position` argument is less than `0`, do nothing.
    ///
    /// If the `position` argument is greater than the track length, do nothing.
    ///
    /// If the *CanSeek* property is `false`, this has no effect.
    fn set_position(
        &self,
        track_id: TrackId,
        position: MprisDuration,
    ) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Opens the Uri given as an argument.
    ///
    /// If the playback is stopped, starts playing
    ///
    /// If the uri scheme or the mime-type of the uri to open is not supported, this method does nothing and may raise an error.
    /// In particular, if the list of available uri schemes is empty, this method may not be implemented.
    ///
    /// Clients should not assume that the Uri has been opened as soon as this method returns. They should wait until the `mpris:trackid` field in the *Metadata* property changes.
    ///
    /// If the media player implements the [TrackList][crate::binding::TrackList] interface, then the opened track should be made part of the tracklist, the *org.mpris.MediaPlayer2.TrackList.TrackAdded* or *org.mpris.MediaPlayer2.TrackList.TrackListReplaced* signal should be fired, as well as the *org.freedesktop.DBus.Properties.PropertiesChanged* signal on the tracklist interface.
    fn open_uri(&self, uri: &str) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async {
            Err(zbus::fdo::Error::NotSupported(
                "not implemented: OpenUri".to_string(),
            ))
        }
    }

    /// The current playback status.
    fn playback_status(&self) -> impl Future<Output = zbus::fdo::Result<PlaybackStatus>> + Send;

    /// The current loop / repeat status
    fn loop_status(&self) -> impl Future<Output = zbus::fdo::Result<LoopStatus>> + Send {
        async { Ok(LoopStatus::None) }
    }
    /// Sets the current loop / repeat status.
    ///
    /// If *CanControl* is `false`, attempting to set this property should have no effect and raise an error.
    fn set_loop_status(&self, value: LoopStatus) -> impl Future<Output = zbus::Result<()>> + Send {
        async { Err(zbus::Error::Unsupported) }
    }

    /// The current playback rate.
    ///
    /// The value must fall in the range described by *MinimumRate* and *MaximumRate*, and must not be `0.0`.
    /// If playback is paused, the *PlaybackStatus* property should be used to indicate this.
    ///
    /// If the media player has no ability to play at speeds other than the normal playback rate, this must still be implemented, and must return `1.0`.
    /// The *MinimumRate* and *MaximumRate* properties must also be set to `1.0`.
    fn rate(&self) -> impl Future<Output = zbus::fdo::Result<f64>> + Send {
        async { Ok(1.0) }
    }
    // Sets the current playback rate.
    //
    /// A value of `0.0` should not be set by the client.
    /// If it is, the media player should act as though *Pause* was called.
    ///
    /// Not all values may be accepted by the media player.
    /// It is left to media player implementations to decide how to deal with values they cannot use; they may either ignore them or pick a "best fit" value.
    /// Clients are recommended to only use sensible fractions or multiples of 1 (eg: `0.5`, `0.25`, `1.5`, `2.0`, etc).
    fn set_rate(&self, value: f64) -> impl Future<Output = zbus::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Whether shuffle is enabled.
    ///
    /// A value of `false` indicates that playback is progressing linearly through a playlist, while `true` means playback is progressing through a playlist in some other order.
    fn shuffle(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }
    /// Set whether shuffle is enabled.
    ///
    /// If *CanControl* is `false`, attempting to set this property should have no effect and raise an error.
    fn set_shuffle(&self, value: bool) -> impl Future<Output = zbus::Result<()>> + Send {
        async { Err(zbus::Error::Unsupported) }
    }

    /// The metadata of the current element.
    ///
    /// If there is a current track, this must have a "mpris:trackid" entry at the very least, which contains a D-Bus path that uniquely identifies this track.
    ///
    /// See [Metadata] for more details.
    fn metadata(&self) -> impl Future<Output = zbus::fdo::Result<Metadata>> + Send;

    /// The volume level.
    fn volume(&self) -> impl Future<Output = zbus::fdo::Result<f64>> + Send {
        async { Ok(1.0) }
    }
    /// Set the volume level.
    ///
    /// When setting, if a negative value is passed, the volume should be set to `0.0`.
    ///
    /// If *CanControl* is `false`, attempting to set this property should have no effect and raise an error.
    fn set_volume(&self, value: f64) -> impl Future<Output = zbus::Result<()>> + Send {
        async { Err(zbus::Error::Unsupported) }
    }

    /// The current track position, between `0` and the 'mpris:length' metadata entry (see [Metadata]).
    ///
    /// Note: If the media player allows it, the current playback position can be changed either by the *SetPosition* method or the *Seek* method on this interface.
    /// If this is not the case, the *CanSeek* property is `false`, and setting this property has no effect and can raise an error.
    ///
    /// If the playback progresses in a way that is inconsistent with the *Rate* property, the *Seeked* signal is emitted.
    fn position(&self) -> impl Future<Output = zbus::fdo::Result<MprisDuration>> + Send;

    /// The minimum value which the *Rate* property can take.
    ///
    /// Clients should not attempt to set the *Rate* property below this value.
    ///
    /// Note that even if this value is `0.0` or negative, clients should not attempt to set the *Rate* property to `0.0`.
    ///
    /// This value should always be `1.0` or less.
    fn minimum_rate(&self) -> impl Future<Output = zbus::fdo::Result<f64>> + Send {
        async { Ok(1.0) }
    }

    /// The maximum value which the *Rate* property can take.
    ///
    /// Clients should not attempt to set the Rate property above this value.
    ///
    /// This value should always be `1.0` or greater.
    fn maximum_rate(&self) -> impl Future<Output = zbus::fdo::Result<f64>> + Send {
        async { Ok(1.0) }
    }

    /// Whether the client can call the *Next* method on this interface and expect the current track to change.
    ///
    /// If it is unknown whether a call to *Next* will be successful (for example, when streaming tracks), this property should be set to `true`.
    ///
    /// If *CanControl* is `false`, this property should also be `false`.
    fn can_go_next(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether the client can call the *Previous* method on this interface and expect the current track to change.
    ///
    /// If it is unknown whether a call to *Previous* will be successful (for example, when streaming tracks), this property should be set to `true`.
    ///
    /// If *CanControl* is `false`, this property should also be `false`.
    fn can_go_previous(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether playback can be started using *Play* or *PlayPause*.
    ///
    /// Note that this is related to whether there is a "current track": the value should not depend on whether the track is currently paused or playing.
    /// In fact, if a track is currently playing (and *CanControl* is `true`), this should be `true`.
    ///
    /// If *CanControl* is `false`, this property should also be `false`.
    fn can_play(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether playback can be paused using *Pause* or *PlayPause*.
    ///
    /// Note that this is an intrinsic property of the current track: its value should not depend on whether the track is currently paused or playing.
    /// In fact, if playback is currently paused (and *CanControl* is `true`), this should be `true`.
    ///
    /// If *CanControl* is `false`, this property should also be `false`.
    fn can_pause(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether the client can control the playback position using *Seek* and *SetPosition*.
    ///
    /// This may be different for different tracks.
    ///
    /// If *CanControl* is `false`, this property should also be `false`.
    fn can_seek(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether the media player may be controlled over this interface.
    ///
    /// This property is not expected to change, as it describes an intrinsic capability of the implementation.
    ///
    /// If this is `false`, clients should assume that all properties on this interface are read-only (and will raise errors if writing to them is attempted), no methods are implemented and all other properties starting with "*Can*" are also `false`.
    fn can_control(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }
}

/// This interface implements the methods for querying and providing basic control over what is currently playing.
///
/// It delegates the D-Bus calls to its provider.
pub struct Player<P>(P);

impl<P> Player<P>
where
    P: PlayerProvider + Send + Sync + 'static,
{
    /// Creates a new MPRIS player that delegates to the given [PlayerProvider].
    pub fn new(provider: P) -> Self {
        Self(provider)
    }

    /// The reference to the underlying [PlayerProvider].
    pub fn inner(&self) -> &P {
        &self.0
    }

    /// The mutable reference to the underlying [PlayerProvider].
    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.0
    }
}

#[interface(
    name = "org.mpris.MediaPlayer2.Player",
    proxy(default_path = "/org/mpris/MediaPlayer2")
)]
impl<P> Player<P>
where
    P: PlayerProvider + Send + Sync + 'static,
{
    /// Next method
    async fn next(&self) -> zbus::fdo::Result<()> {
        self.0.next().await
    }

    /// Previous method
    async fn previous(&self) -> zbus::fdo::Result<()> {
        self.0.previous().await
    }

    /// Pause method
    async fn pause(&self) -> zbus::fdo::Result<()> {
        self.0.pause().await
    }

    /// PlayPause method
    async fn play_pause(&self) -> zbus::fdo::Result<()> {
        self.0.play_pause().await
    }

    /// Stop method
    async fn stop(&self) -> zbus::fdo::Result<()> {
        self.0.stop().await
    }

    /// Play method
    async fn play(&self) -> zbus::fdo::Result<()> {
        self.0.play().await
    }

    /// Seek method
    async fn seek(&self, offset: MprisDuration) -> zbus::fdo::Result<()> {
        self.0.seek(offset).await
    }

    /// SetPosition method
    async fn set_position(
        &self,
        track_id: TrackId,
        position: MprisDuration,
    ) -> zbus::fdo::Result<()> {
        self.0.set_position(track_id, position).await
    }

    /// OpenUri method
    async fn open_uri(&self, uri: &str) -> zbus::fdo::Result<()> {
        self.0.open_uri(uri).await
    }

    /// Seeked signal
    ///
    /// Indicates that the track position has changed in a way that is inconsistent with the current playing state.
    ///
    /// This signal does not need to be emitted when playback starts or when the track changes, unless the track is starting at an unexpected position.
    /// An expected position would be the last known one when going from *Paused* to *Playing*, and `0` when going from *Stopped* to *Playing*.
    #[zbus(signal)]
    async fn seeked(emitter: &SignalEmitter<'_>, position: MprisDuration) -> zbus::Result<()>;

    /// PlaybackStatus property
    #[zbus(property)]
    async fn playback_status(&self) -> zbus::fdo::Result<PlaybackStatus> {
        self.0.playback_status().await
    }

    /// LoopStatus property
    #[zbus(property)]
    async fn loop_status(&self) -> zbus::fdo::Result<LoopStatus> {
        self.0.loop_status().await
    }
    #[zbus(property)]
    async fn set_loop_status(&self, value: LoopStatus) -> zbus::Result<()> {
        self.0.set_loop_status(value).await
    }

    /// Rate property
    #[zbus(property)]
    async fn rate(&self) -> zbus::fdo::Result<f64> {
        self.0.rate().await
    }
    #[zbus(property)]
    async fn set_rate(&self, value: f64) -> zbus::Result<()> {
        self.0.set_rate(value).await
    }

    /// Shuffle property
    #[zbus(property)]
    async fn shuffle(&self) -> zbus::fdo::Result<bool> {
        self.0.shuffle().await
    }
    #[zbus(property)]
    async fn set_shuffle(&self, value: bool) -> zbus::Result<()> {
        self.0.set_shuffle(value).await
    }

    /// Metadata property
    #[zbus(property)]
    async fn metadata(&self) -> zbus::fdo::Result<Metadata> {
        self.0.metadata().await
    }

    /// Volume property
    #[zbus(property)]
    async fn volume(&self) -> zbus::fdo::Result<f64> {
        self.0.volume().await
    }
    #[zbus(property)]
    async fn set_volume(&self, value: f64) -> zbus::Result<()> {
        self.0.set_volume(value).await
    }

    /// Position property
    #[zbus(property(emits_changed_signal = "false"))]
    async fn position(&self) -> zbus::fdo::Result<MprisDuration> {
        self.0.position().await
    }

    /// MinimumRate property
    #[zbus(property)]
    async fn minimum_rate(&self) -> zbus::fdo::Result<f64> {
        self.0.minimum_rate().await
    }

    /// MaximumRate property
    #[zbus(property)]
    async fn maximum_rate(&self) -> zbus::fdo::Result<f64> {
        self.0.maximum_rate().await
    }

    /// CanGoNext property
    #[zbus(property)]
    async fn can_go_next(&self) -> zbus::fdo::Result<bool> {
        self.0.can_go_next().await
    }

    /// CanGoPrevious property
    #[zbus(property)]
    async fn can_go_previous(&self) -> zbus::fdo::Result<bool> {
        self.0.can_go_previous().await
    }

    /// CanPlay property
    #[zbus(property)]
    async fn can_play(&self) -> zbus::fdo::Result<bool> {
        self.0.can_play().await
    }

    /// CanPause property
    #[zbus(property)]
    async fn can_pause(&self) -> zbus::fdo::Result<bool> {
        self.0.can_pause().await
    }

    /// CanSeek property
    #[zbus(property)]
    async fn can_seek(&self) -> zbus::fdo::Result<bool> {
        self.0.can_seek().await
    }

    /// CanControl property
    #[zbus(property(emits_changed_signal = "false"))]
    async fn can_control(&self) -> zbus::fdo::Result<bool> {
        self.0.can_control().await
    }
}
