// SPDX-License-Identifier: MPL-2.0
use std::ops::Deref;

use zbus::Connection;
use zbus::names::OwnedBusName;

use crate::binding::{MediaPlayer2Proxy, PlayerProxy};
use crate::types::{LoopStatus, Metadata, MprisDuration, PlaybackStatus, TrackId};
use crate::{Error, MediaPlayer, Result, handle_optional};

/// Simplified access to the controls of an MPRIS media player.
#[derive(Debug, Clone)]
pub struct Player {
    proxy: PlayerProxy<'static>,
}

impl Player {
    /// Creates a new instance of the `org.mpris.MediaPlayer2.Player` interface.
    // TODO this being an OwnedBusName makes it really annoying to create a player
    pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
        PlayerProxy::builder(connection)
            .destination(name)?
            .build()
            .await
            .map(Self::from)
            .map_err(Error::from)
    }

    /// Returns this player's `org.mpris.MediaPlayer2` instance
    pub async fn media_player(&self) -> Result<MediaPlayer> {
        let proxy = MediaPlayer2Proxy::builder(self.proxy.inner().connection())
            .destination(self.proxy.inner().destination().to_owned())?
            .build()
            .await?;
        Ok(proxy.into())
    }

    /// Seeks the specified duration.
    pub async fn seek<D>(&self, duration: D) -> Result<bool>
    where
        D: Into<MprisDuration>,
    {
        if self.proxy.can_seek().await? {
            self.proxy.seek(duration.into()).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Sets the current track position.
    ///
    /// If `track` does not match the id of the currently-playing track, the call is ignored as "stale".
    pub async fn set_position<D>(&self, track: TrackId, position: D) -> Result<()>
    where
        D: Into<MprisDuration>,
    {
        self.proxy
            .set_position(track, position.into())
            .await
            .map_err(Error::from)
    }

    /// How far into the current track the player is.
    ///
    /// Not all players support this, and it will return None if this is the case.
    pub async fn position(&self) -> Result<Option<MprisDuration>> {
        handle_optional(self.proxy.position().await)
    }

    /// Gets the current playback status of the player.
    pub async fn playback_status(&self) -> Result<PlaybackStatus> {
        self.proxy.playback_status().await.map_err(Error::from)
    }

    /// Returns the current rate of playback.
    ///
    /// Not all players support this, and it will return None if this is the case.
    pub async fn rate(&self) -> Result<Option<f64>> {
        handle_optional(self.proxy.rate().await)
    }

    /// Sets the current rate of playback.
    pub async fn set_rate(&self, value: f64) -> Result<()> {
        handle_optional(self.proxy.set_rate(value).await).map(|_| ())
    }

    /// Returns the minimum supported rate for the player.
    ///
    /// Not all players support this, and it will return None if this is the case.
    pub async fn minimum_rate(&self) -> Result<Option<f64>> {
        handle_optional(self.proxy.minimum_rate().await)
    }

    /// Returns the minimum supported rate for the player.
    ///
    /// Not all players support this, and it will return None if this is the case.
    pub async fn maximum_rate(&self) -> Result<Option<f64>> {
        handle_optional(self.proxy.maximum_rate().await)
    }

    /// Returns the range of playback rates available for the player.
    ///
    /// Not all players support this, and it will return None if this is the case.
    pub async fn available_rates(&self) -> Result<Option<std::ops::RangeInclusive<f64>>> {
        let minimum = match self.minimum_rate().await? {
            Some(min) => min,
            None => return Ok(None),
        };
        let maximum = match self.maximum_rate().await? {
            Some(max) => max,
            None => return Ok(None),
        };
        Ok(Some(minimum..=maximum))
    }

    /// Returns the metadata for the player.
    pub async fn metadata(&self) -> Result<Metadata> {
        self.proxy.metadata().await.map_err(Error::from)
    }

    /// Whether the current playlist is shuffled or not.
    ///
    /// A value of false indicates that playback is progressing linearly through a playlist,
    /// while true means playback is progressing through a playlist in some other order.
    pub async fn shuffle(&self) -> Result<Option<bool>> {
        if self.can_control().await? {
            handle_optional(self.proxy.shuffle().await)
        } else {
            Ok(None)
        }
    }

    /// Set whether the current playlist is shuffled or not.
    ///
    /// A value of false indicates that playback is progressing linearly through a playlist,
    /// while true means playback is progressing through a playlist in some other order.
    pub async fn set_shuffle(&self, value: bool) -> Result<()> {
        if self.proxy.can_control().await? {
            self.proxy.set_shuffle(value).await.map_err(Error::from)
        } else {
            Ok(())
        }
    }

    /// The current loop / repeat status.
    pub async fn loop_status(&self) -> Result<Option<LoopStatus>> {
        if self.proxy.can_control().await? {
            handle_optional(self.proxy.loop_status().await)
        } else {
            Ok(None)
        }
    }

    /// Set the current loop / repeat status.
    pub async fn set_loop_status(&self, value: LoopStatus) -> Result<()> {
        if self.proxy.can_control().await? {
            handle_optional(self.proxy.set_loop_status(value).await).map(|_| ())
        } else {
            Ok(())
        }
    }
}

impl Deref for Player {
    type Target = PlayerProxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.proxy
    }
}

impl From<PlayerProxy<'static>> for Player {
    fn from(proxy: PlayerProxy<'static>) -> Self {
        Self { proxy }
    }
}
