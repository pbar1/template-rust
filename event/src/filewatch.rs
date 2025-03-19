use std::path::Path;
use std::sync::mpsc;

use anyhow::Context;
use notify::Config;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use tracing::debug;

use super::Event;
use super::EventType;
use super::Guard;

/// Publishes an [`Event`] if a killfile is created.
pub fn killfile(
    tx: mpsc::Sender<Event>,
    path: impl AsRef<Path> + Send,
    event_type: EventType,
) -> anyhow::Result<Guard> {
    let path = path.as_ref().to_owned();
    let parent = path
        .parent()
        .context("unable to get parent path")?
        .to_owned();

    let config = Config::default();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| match res {
            Ok(notify_event) if notify_event.kind.is_create() => {
                let target_path = path.clone();
                for p in notify_event.paths {
                    if p == target_path {
                        debug!(path = %p.to_string_lossy(), "file create event");
                        let event = match event_type {
                            EventType::Terminate => Event::Terminate,
                            EventType::FileUpdate => Event::FileUpdate(p.clone()),
                        };
                        if let Err(error) = tx.clone().send(event) {
                            debug!(?error, path = %p.to_string_lossy(), "unable to send event");
                        };
                        break;
                    }
                }
            }
            Ok(_) => {}
            Err(error) => debug!(?error, "notify error"),
        },
        config,
    )?;

    watcher.watch(&parent, RecursiveMode::NonRecursive)?;

    Ok(Guard::new(watcher))
}
