use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use anyhow::Context;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::Error as NotifyError;
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::DebouncedEvent;
use tracing::debug;

use super::Event;
use super::EventType;
use super::Guard;

/// Publishes an [`Event`] if a killfile is created.
///
/// Uses a debounce duration of 1 second.
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

    let mut watcher = new_debouncer(
        Duration::from_secs(1),
        None,
        move |res: Result<Vec<DebouncedEvent>, Vec<NotifyError>>| match res {
            Ok(notify_events) => {
                for notify_event in notify_events.iter() {
                    if notify_event.kind.is_create() {
                        let target_path = path.clone();
                        for p in &notify_event.paths {
                            if *p == target_path {
                                debug!(path = %p.to_string_lossy(), "file create event");
                                let event = match event_type {
                                    EventType::Terminate => Event::Terminate,
                                    EventType::FileUpdate => Event::FileUpdate(p.clone()),
                                };
                                if let Err(error) = tx.send(event) {
                                    debug!(?error, path = %p.to_string_lossy(), "unable to send event");
                                };
                                break;
                            }
                        }
                    }
                }
            }
            Err(error) => debug!(?error, "notify error"),
        },
    )?;

    watcher.watch(&parent, RecursiveMode::NonRecursive)?;

    Ok(Guard::new(watcher))
}
