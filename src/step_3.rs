use std::process::Command;
use colored::*;

pub fn run() {
    println!("{}", "Warning: This command will sync your time to absolute.htb...".blue());
    run_ntpdate();
    println!("{}", "Time Changed, you should double check it really did....".green());
}

fn run_ntpdate() {
    // Stop the systemd-timesyncd.service
    let stop_output = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("systemd-timesyncd.service")
        .output()
        .expect("failed to execute sudo systemctl stop");

    println!("{}", String::from_utf8_lossy(&stop_output.stdout));
    println!("{}", String::from_utf8_lossy(&stop_output.stderr));
    
    // timedatectl command
    let timedate_ctl = Command::new("sudo")
        .arg("timedatectl")
        .arg("set-ntp")
        .arg("false")
        .output()
        .expect("failed to execute sudo timedatectl");

    println!("{}", String::from_utf8_lossy(&timedate_ctl.stdout));
    println!("{}", String::from_utf8_lossy(&timedate_ctl.stderr));

    // ntpdate command
    let ntpdate_output = Command::new("sudo")
        .arg("ntpdate")
        .arg("-s")
        .arg("absolute.htb")
        .output()
        .expect("failed to execute sudo ntpdate");

    // Print the command output to the console
    println!("{}", String::from_utf8_lossy(&ntpdate_output.stdout));
    println!("{}", String::from_utf8_lossy(&ntpdate_output.stderr));
}