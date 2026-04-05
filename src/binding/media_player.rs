// SPDX-License-Identifier: MPL-2.0
use zbus::interface;

/// Implement this trait to provide the functionality of [MediaPlayer2].
#[allow(unused_variables)]
pub trait MediaPlayer2Provider {
    /// Brings the media player's user interface to the front using any appropriate mechanism available.
    ///
    /// The media player may be unable to control how its user interface is displayed, or it may not have a graphical user interface at all.
    /// In this case, the *CanRaise* property is `false` and this method does nothing.
    fn raise(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Causes the media player to stop running.
    ///
    /// The media player may refuse to allow clients to shut it down.
    /// In this case, the *CanQuit* property is `false` and this method does nothing.
    ///
    /// Note: Media players which can be D-Bus activated, or for which there is no sensibly easy way to terminate a running instance (via the main interface or a notification area icon for example) should allow clients to use this method.
    /// Otherwise, it should not be needed.
    ///
    /// If the media player does not have a UI, this should be implemented.
    fn quit(&self) -> impl Future<Output = zbus::fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Whether the media player can quit.
    ///
    /// If `false`, calling *Quit* will have no effect, and may raise a [zbus::fdo::Error::NotSupported] error.
    /// If `true`, calling *Quit* will cause the media application to attempt to quit (although it may still be prevented from quitting by the user, for example).
    fn can_quit(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether the media player is set to fullscreen.
    ///
    /// This is typically used for videos.
    /// A value of `true` indicates that the media player is taking up the full screen.
    ///
    /// Media centre software may well have this value fixed to `true`.
    fn fullscreen(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }
    /// Changes the fullscreen state of the media player.
    ///
    /// If *CanSetFullscreen* is `true`, clients may set this property to `true` to tell the media player to enter fullscreen mode, or to `false` to return to windowed mode.
    ///
    /// If *CanSetFullscreen* is `false`, then attempting to set this property should have no effect, and may raise an error. However, even if it is `true`, the media player may still be unable to fulfil the request, in which case attempting to set this property will have no effect (but should not raise an error).
    fn set_fullscreen(&self, value: bool) -> impl Future<Output = zbus::Result<()>> + Send {
        async { Ok(()) }
    }

    /// Whether the media player can be set to fullscreen.
    ///
    /// If `false`, attempting to set *Fullscreen* will have no effect, and may raise an error.
    /// If `true`, attempting to set *Fullscreen* will not raise an error, and (if it is different from the current value) will cause the media player to attempt to enter or exit fullscreen mode.
    ///
    /// Note that the media player may be unable to fulfil the request.
    /// In this case, the value will not change.
    /// If the media player knows in advance that it will not be able to fulfil the request, however, this property should be `false`.
    fn can_set_fullscreen(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Whether the media player can be brought to the front.
    ///
    /// If `false`, calling *Raise* will have no effect, and may raise a [zbus::fdo::Error::NotSupported] error.
    /// If `true`, calling *Raise will cause the *media application to attempt to bring its user interface to the front, although it may be prevented from doing so (by the window manager, for example).
    fn can_raise(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// Indicates whether the [MediaPlayer2] object implements the [TrackList][crate::binding::TrackList] interface.
    fn has_track_list(&self) -> impl Future<Output = zbus::fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// A friendly name to identify the media player to users.
    ///
    /// This should usually match the name found in .desktop files (eg: "VLC media player").
    fn identity(&self) -> impl Future<Output = zbus::fdo::Result<String>> + Send;

    /// The basename of an installed .desktop file which complies with the [Desktop entry specification](https://standards.freedesktop.org/desktop-entry-spec/latest/), with the ".desktop" extension stripped.
    ///
    /// Example: The desktop entry file is "/usr/share/applications/vlc.desktop", and this property contains "vlc"
    fn desktop_entry(&self) -> impl Future<Output = zbus::fdo::Result<String>> + Send;

    /// The URI schemes supported by the media player.
    ///
    /// This can be viewed as protocols supported by the player in almost all cases.
    /// Almost every media player will include support for the "file" scheme.
    /// Other common schemes are "http" and "rtsp".
    ///
    /// Note that URI schemes should be lower-case.
    fn supported_uri_schemes(&self) -> impl Future<Output = zbus::fdo::Result<Vec<String>>> + Send {
        async { Ok(Vec::new()) }
    }

    /// The mime-types supported by the media player.
    ///
    /// Mime-types should be in the standard format (eg: `audio/mpeg` or `application/ogg`).
    fn supported_mime_types(&self) -> impl Future<Output = zbus::fdo::Result<Vec<String>>> + Send {
        async { Ok(Vec::new()) }
    }
}

/// D-Bus interface for an MPRIS media player.
///
/// It delegates the D-Bus calls to its provider.
pub struct MediaPlayer2<P>(P);

impl<P> MediaPlayer2<P>
where
    P: MediaPlayer2Provider + Send + Sync + 'static,
{
    /// Creates a new MPRIS media player that delegates to the given [MediaPlayer2Provider].
    pub fn new(provider: P) -> Self {
        Self(provider)
    }

    /// The reference to the underlying [MediaPlayer2Provider].
    pub fn inner(&self) -> &P {
        &self.0
    }

    /// The mutable reference to the underlying [MediaPlayer2Provider].
    pub fn inner_mut(&mut self) -> &mut P {
        &mut self.0
    }
}

#[interface(
    interface = "org.mpris.MediaPlayer2",
    proxy(default_path = "/org/mpris/MediaPlayer2")
)]
impl<P> MediaPlayer2<P>
where
    P: MediaPlayer2Provider + Send + Sync + 'static,
{
    /// Raise method
    async fn raise(&self) -> zbus::fdo::Result<()> {
        self.0.raise().await
    }

    /// Quit method
    async fn quit(&self) -> zbus::fdo::Result<()> {
        self.0.quit().await
    }

    /// CanQuit property
    #[zbus(property)]
    async fn can_quit(&self) -> zbus::fdo::Result<bool> {
        self.0.can_quit().await
    }

    /// Fullscreen property
    #[zbus(property)]
    async fn fullscreen(&self) -> zbus::fdo::Result<bool> {
        self.0.fullscreen().await
    }
    #[zbus(property)]
    async fn set_fullscreen(&self, value: bool) -> zbus::Result<()> {
        self.0.set_fullscreen(value).await
    }

    /// CanSetFullscreen property
    #[zbus(property)]
    async fn can_set_fullscreen(&self) -> zbus::fdo::Result<bool> {
        self.0.can_set_fullscreen().await
    }

    /// CanRaise property
    #[zbus(property)]
    async fn can_raise(&self) -> zbus::fdo::Result<bool> {
        self.0.can_raise().await
    }

    /// HasTrackList property
    #[zbus(property)]
    async fn has_track_list(&self) -> zbus::fdo::Result<bool> {
        self.0.has_track_list().await
    }

    /// Identity property
    #[zbus(property)]
    async fn identity(&self) -> zbus::fdo::Result<String> {
        self.0.identity().await
    }

    /// DesktopEntry property
    #[zbus(property)]
    async fn desktop_entry(&self) -> zbus::fdo::Result<String> {
        self.0.desktop_entry().await
    }

    /// SupportedUriSchemes property
    #[zbus(property)]
    async fn supported_uri_schemes(&self) -> zbus::fdo::Result<Vec<String>> {
        self.0.supported_uri_schemes().await
    }

    /// SupportedMimeTypes property
    #[zbus(property)]
    async fn supported_mime_types(&self) -> zbus::fdo::Result<Vec<String>> {
        self.0.supported_mime_types().await
    }
}
