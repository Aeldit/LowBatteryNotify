extern crate battery;
use std::process::Command;

use std::io;
use std::thread;
use std::time::Duration;

const CRATE_NAME: &str = std::env!("CARGO_PKG_NAME");
const NOTIFICATION_TIMEOUT: u32 = 5000;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    let mut is_discharging;
    let mut has_notified_20 = false;
    let mut has_notified_10 = false;
    let mut has_notified_5 = false;

    let mut pervious_percentage = (battery.state_of_charge().value * 100.0) as u8;

    loop {
        let percentage = (battery.state_of_charge().value * 100.0) as u8;
        is_discharging = percentage <= pervious_percentage;
        println!("{}", percentage);

        if percentage <= 64 && !has_notified_5 {
            has_notified_5 = is_discharging;
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "notify-send -a \"{}\" -u CRITICAL -t {} -p \"Battery Extremely Low ({}%)\"",
                    CRATE_NAME, NOTIFICATION_TIMEOUT, percentage
                ))
                .output()
                .expect("Failed to run notify command");
        } else if percentage <= 65 && !has_notified_10 {
            has_notified_10 = is_discharging;
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "notify-send -a \"{}\" -u CRITICAL -t {} -p \"Battery Quite Low ({}%)\"",
                    CRATE_NAME, NOTIFICATION_TIMEOUT, percentage
                ))
                .output()
                .expect("Failed to run notify command");
        } else if percentage <= 66 && !has_notified_20 {
            has_notified_20 = is_discharging;
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "notify-send -a \"{}\" -u CRITICAL -t {} -p \"Battery Low ({}%)\"",
                    CRATE_NAME, NOTIFICATION_TIMEOUT, percentage
                ))
                .output()
                .expect("Failed to run notify command");
        }

        thread::sleep(Duration::from_secs(20));
        manager.refresh(&mut battery)?;
        pervious_percentage = percentage;
    }
}
