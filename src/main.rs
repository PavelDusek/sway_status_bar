use std::process::Command;
// use std::{thread, time};
use chrono::prelude::*;

fn main() {
    let dt: DateTime<Local> = Local::now();
    let time: String = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    let acpi_command = Command::new("/usr/bin/acpi")
        .output()
        .expect("Could not run acpi");
    
    let acpi = String::from_utf8(acpi_command.stdout).unwrap();

    //the pattern is as follows:
    //Battery 0: Full, 100%
    //Battery 0: Discharging, 67%, 03:17:42 remaining
    
    let percent: usize;
    let remaining: &str;
    let battery: &str;
    match acpi.find("Full") {
        Some(_) => {
            battery = "100";
            remaining = "charging";
        },
        None => {
            percent = acpi.find('%').unwrap();
            battery = &acpi[percent-2..percent];
            remaining = &acpi[percent+3..percent+11];
        }
    }

    let result = format!("{} % ({}) | {}", battery, remaining, time);
    println!("{}", result);
}
