// SPDX-License-Identifier: MPL-2.0
use std::collections::HashMap;

use doobs_mpris::binding::{PlayerProvider, PlayerProxy};
use doobs_mpris::types::MprisDuration;
use jiff::SignedDuration;
use miette::{IntoDiagnostic, Result, WrapErr};
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    // the name of our dbus service
    // it should start with `org.mpris.MediaPlayer2.` in order to be discoverable
    // by the enumerator and tools like playerctl
    let service_name = "org.mpris.MediaPlayer2.foo";

    // start our Foo player

    let player = doobs_mpris::binding::Player(FooPlayerProvider);
    let _player_iface = zbus::connection::Builder::session()
        .into_diagnostic()
        .wrap_err("Failed to create session D-Bus builder")?
        .name(service_name)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to register D-Bus service: {service_name}"))?
        .serve_at("/org/mpris/MediaPlayer2", player)
        .into_diagnostic()
        .wrap_err("Failed to serve Foo player on MPRIS path")?
        .build()
        .await
        .into_diagnostic()
        .wrap_err("Failed to build session D-Bus connection for Foo player")?;
    // the player is now listening to method calls until the iface is dropped

    // connect to the Foo player and fetch the metadata for the current track

    let client_conn = Connection::session()
        .await
        .into_diagnostic()
        .wrap_err("Failed to establish session D-Bus connection for client")?;
    let client = PlayerProxy::new(&client_conn, service_name)
        .await
        .into_diagnostic()
        .wrap_err("Failed to create proxy for Foo player")?;

    let metadata = client
        .metadata()
        .await
        .into_diagnostic()
        .wrap_err("Failed to get metadata from Foo player")?;

    println!("Current track on Foo player has the following metadata: {metadata:?}");

    Ok(())
}

/// Example for a minimal read only player.
/// Additional functions for controlling the player are available in the trait.
struct FooPlayerProvider;

impl PlayerProvider for FooPlayerProvider {
    async fn metadata(&self) -> zbus::fdo::Result<HashMap<String, zvariant::OwnedValue>> {
        let track_id: zvariant::OwnedValue =
            zvariant::Value::from("/noplaylist").try_into().unwrap();
        let title: zvariant::OwnedValue = zvariant::Value::from("A Really Cool Song")
            .try_into()
            .unwrap();

        let mut metadata = HashMap::new();
        metadata.insert("mpris:trackid".to_string(), track_id); // trackid is required
        metadata.insert("xesam:title".to_string(), title);
        Ok(metadata)
    }

    async fn playback_status(&self) -> zbus::fdo::Result<String> {
        Ok("Playing".to_string())
    }

    async fn position(&self) -> zbus::fdo::Result<MprisDuration> {
        Ok(SignedDuration::from_secs(42).into())
    }
}
