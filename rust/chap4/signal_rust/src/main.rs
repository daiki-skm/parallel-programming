use signal_hook::{iterator::Signals, SIGUSR1};
use std::{error::Error, process, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    println!("pid: {}", process::id());

    let signals = Signals::new(&[SIGUSR1])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("received signal: {:?}", sig);
        }
    });

    thread::sleep(Duration::from_secs(10));
    Ok(())
}