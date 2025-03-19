use std::sync::mpsc;

use signal_hook::consts::SIGINT;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;
use tracing::debug;

use super::Event;
use super::Guard;

pub fn termsignal(tx: mpsc::Sender<Event>) -> anyhow::Result<Guard> {
    let mut signals = Signals::new([SIGINT, SIGTERM])?;

    let handle = std::thread::spawn(move || {
        for signum in &mut signals {
            debug!(%signum, "received signal");
            match signum {
                SIGINT | SIGTERM => {
                    if let Err(error) = tx.send(Event::Terminate) {
                        debug!(?error, %signum, "unable to send terminate event");
                    };
                    // TODO: For now, this only allows one term event to be sent. If we don't, then
                    // the program never exits.
                    break;
                }
                _other => {}
            }
        }
    });

    Ok(Guard::new(handle))
}
