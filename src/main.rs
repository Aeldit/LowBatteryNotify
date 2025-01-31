use std::fs;
use std::process::Command;

use std::thread::sleep;
use std::time::Duration;

/**
** \returns 1 if the current computer is a laptop or a notebook, 0 otherwise
*/
fn is_laptop() -> bool {
    match fs::read_to_string("/sys/class/dmi/id/chassis_type") {
        Ok(contents) => contents.starts_with("9") || contents.starts_with("10"),
        Err(_) => false,
    }
}

fn is_discharging() -> bool {
    match fs::read_to_string("/sys/class/power_supply/BAT0/status") {
        Ok(contents) => contents.starts_with("Discharging"),
        Err(_) => false,
    }
}

fn get_percentage() -> Option<u8> {
    match fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
        Ok(contents) => match contents.parse::<u8>() {
            Ok(percentage) => Some(percentage),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn notify(percentage: u8) {
    if Command::new("sh")
        .arg("-c")
        .arg(format!(
            "notify-send -a \"LowBatteryNotify\" -u CRITICAL -t 5000 -p \"Battery Low ({}%)\"",
            percentage
        ))
        .output()
        .is_err()
    {};
}

fn main() {
    if !is_laptop() {
        return;
    }

    let mut has_notified_20 = false;
    let mut has_notified_10 = false;
    let mut has_notified_5 = false;

    loop {
        if !is_discharging() {
            has_notified_20 = false;
            has_notified_10 = false;
            has_notified_5 = false;
        } else if let Some(percentage) = get_percentage() {
            if percentage <= 5 && !has_notified_5 {
                has_notified_5 = true;
                notify(percentage);
            } else if percentage <= 10 && !has_notified_10 {
                has_notified_10 = true;
                notify(percentage);
            } else if percentage <= 20 && !has_notified_20 {
                has_notified_20 = true;
                notify(percentage);
            }
        }

        sleep(Duration::from_secs(60));
    }
}
