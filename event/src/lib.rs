#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use std::any::Any;
use std::path::PathBuf;
use std::sync::mpsc;

use bon::builder;
use bon::Builder;
use camino::Utf8PathBuf;

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

    #[cfg(feature = "signal")]
    /// Event that should be published upon receiving an interrupt signal.
    #[builder(default = EventType::Terminate)]
    interrupt_event: EventType,

    #[cfg(feature = "signal")]
    /// Event that should be published upon receiving a termination signal.
    #[builder(default = EventType::Terminate)]
    terminate_event: EventType,
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
    pub fn listen(&self) -> anyhow::Result<(mpsc::Receiver<Event>, Vec<Guard>)> {
        let (tx, rx) = mpsc::channel();
        let mut guards = Vec::new();

        #[cfg(feature = "filewatch")]
        {
            if let Some(killfile) = &self.killfile_path {
                let killfile_guard =
                    filewatch::killfile(tx.clone(), &killfile, self.killfile_event)?;
                guards.push(killfile_guard);
            }
        }

        #[cfg(feature = "signal")]
        {}

        Ok((rx, guards))
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
