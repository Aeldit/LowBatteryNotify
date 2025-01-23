extern crate battery;
use std::process::Command;

use std::io::{Error, ErrorKind};
use std::thread::sleep;
use std::time::Duration;

use battery::State;

fn notify(percentage: u8, cmd: &mut Command) {
    if let Err(e) = cmd
        .arg(format!(
            "notify-send -a \"LowBatteryNotify\" -u CRITICAL -t 5000 -p \"Battery Low ({}%)\"",
            percentage
        ))
        .output()
    {
        eprintln!("Failed to run the notify-send command ({e:?})");
    };
}

fn main() -> battery::Result<()> {
    if !is_laptop::check() {
        eprintln!("Terminating the program, this computer is not a laptop");
        return Err(Error::from(ErrorKind::NotFound).into());
    }

    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(Error::from(ErrorKind::NotFound).into());
        }
    };

    let mut is_discharging;
    let mut has_notified_20 = false;
    let mut has_notified_10 = false;
    let mut has_notified_5 = false;

    let mut cmd = Command::new("sh");
    cmd.arg("-c");

    loop {
        let percentage = (battery.state_of_charge().value * 100.0) as u8;
        is_discharging = battery.state() == State::Discharging;
        if !is_discharging {
            has_notified_20 = false;
            has_notified_10 = false;
            has_notified_5 = false;
        }

        if is_discharging {
            if percentage <= 5 && !has_notified_5 {
                has_notified_5 = true;
                notify(percentage, &mut cmd);
            } else if percentage <= 10 && !has_notified_10 {
                has_notified_10 = true;
                notify(percentage, &mut cmd);
            } else if percentage <= 20 && !has_notified_20 {
                has_notified_20 = true;
                notify(percentage, &mut cmd);
            }
        }

        sleep(Duration::from_secs(20));
        manager.refresh(&mut battery)?;
    }
}
