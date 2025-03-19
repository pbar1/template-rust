#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::any::Any;
use std::path::PathBuf;

use bon::builder;
use bon::Builder;
use camino::Utf8PathBuf;
use tracing::debug;

#[cfg(feature = "filewatch")]
pub mod filewatch;
#[cfg(feature = "signal")]
pub mod signal;

/// Variants of [`Event`] with no payload.
///
/// This is needed for configuration purposes, as the event may contain a
/// payload that varies at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// See [`Event::Terminate`]
    Terminate,
    /// See [`Event::FileUpdate`]
    FileUpdate,
}

/// Events that can be published.
#[derive(Debug, Clone)]
pub enum Event {
    /// Graceful termination has been requested.
    Terminate,
    /// File has been updated.
    FileUpdate(PathBuf),
}

/// All-in-one config for handling events such as signals and killswitches.
#[derive(Debug, Clone, Builder)]
pub struct EventConfig {
    #[cfg(feature = "filewatch")]
    /// Watch for the creation of this file to trigger the killfile event.
    killfile_path: Option<Utf8PathBuf>,

    #[cfg(feature = "filewatch")]
    /// Event that should be broadcast upon creation of the killfile.
    #[builder(default = EventType::Terminate)]
    killfile_event: EventType,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl EventConfig {
    /// Begin listening for events, which will be relayed on the returned
    /// channel. Each listener will be spawned on its own thread, so this
    /// function does not need to be.
    ///
    /// If the guards that are returned by this function are dropped, no more
    /// values will be produced.
    pub fn listen(&self) -> anyhow::Result<(std::sync::mpsc::Receiver<Event>, Vec<Guard>)> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut guards = Vec::new();

        #[cfg(feature = "filewatch")]
        {
            if let Some(killfile) = &self.killfile_path {
                let guard = filewatch::killfile(tx.clone(), &killfile, self.killfile_event)?;
                guards.push(guard);
            }
        }

        #[cfg(feature = "signal")]
        {
            let guard = signal::termsignal(tx.clone())?;
            guards.push(guard);
        }

        Ok((rx, guards))
    }

    #[cfg(feature = "tokio")]
    /// Async version of [`EventConfig::listen`] compatible with Tokio.
    ///
    /// Spawns a thread to bridge the std and Tokio channels. If either receive
    /// or send fail, the loop is broken and the thread ends.
    pub fn listen_tokio(
        &self,
    ) -> anyhow::Result<(tokio::sync::mpsc::UnboundedReceiver<Event>, Vec<Guard>)> {
        let (std_rx, guards) = self.listen()?;
        let (tok_tx, tok_rx) = tokio::sync::mpsc::unbounded_channel();

        // TODO: Should we put the JoinHandle into guards?
        tokio::task::spawn_blocking(move || {
            while let Ok(event) = std_rx.recv() {
                if let Err(error) = tok_tx.send(event) {
                    debug!(?error, "tokio mpsc sender was closed, exiting bridge loop");
                    break;
                }
            }
        });

        Ok((tok_rx, guards))
    }
}

/// Guard to keep things like watchers alive. When this is dropped, the inner
/// value will also be dropped.
pub struct Guard {
    _inner: Box<dyn Any>,
}

impl Guard {
    pub fn new(inner: impl Any) -> Self {
        Self {
            _inner: Box::new(inner),
        }
    }
}
