use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};
use colored::*;

mod step_1;
mod step_2;
mod step_3;
mod step_4;
mod step_5;
mod step_6;
mod step_8;
mod step_9;

fn main() {
    let ip = get_ip_address();
    add_host_entry(&ip);

    loop {
        display_menu();
        
        let choice = get_user_choice();

        match choice.as_str() {
            "1" => step_1::run(),
            "2" => step_2::run(),
            "3" => step_3::run(),
            "4" => step_4::run(),
            "5" => step_5::run(),
            "6" => step_6::run(),
            "7" => display_next_steps(),
            "8" => display_windows_warning(),
            "9" => step_9::run(),
            "00" => run_steps_1_to_4(),
            "10" => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

fn get_ip_address() -> String {
    let mut ip = String::new();
    print!("Enter IP address: ");
    let _ = stdout().flush();
    stdin().read_line(&mut ip).expect("Failed to read input");
    ip.trim().to_string()
}

fn add_host_entry(ip: &str) {
    let host_entry = format!("\n{}\tabsolute.htb dc.absolute.htb", ip);

    let output = Command::new("grep")
        .arg(ip)
        .arg("/etc/hosts")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let mut child = Command::new("sudo")
            .arg("tee")
            .arg("-a")
            .arg("/etc/hosts")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(host_entry.as_bytes()).expect("Failed to write to stdin");
        }

        child.wait_with_output().expect("Failed to read command output");

        println!("\nAdded host entry: {}", host_entry);
    } else {
        println!("IP address already exists in /etc/hosts file");
    }
}

fn display_menu() {
    println!("{}", "\nSelect an option:".bold());
    println!("{}", "Step 1. Generate Username List".green());
    println!("{}", "Step 2. Validate + ASREPRoast".green());
    println!("{}", "Step 3. Sync time to Absolute.htb".green());
    println!("{}", "Step 4. Generate TGTs + Dump Users".green());
    println!("{}", "Step 5. Grab SMB File (you can skip this...)".green());
    println!("{}", "Step 6. BloodHound Scan (you can skip this...)".green());
    println!("{}", "Step 7. Next Steps Information (please read)".green());
    println!("{}", "------------------------------------------------------".bold().red());
    println!("{}", "Step 8. MUST BE RAN ON WINDOWS!".bold().red());
    println!("{}", "------------------------------------------------------".bold().red());
    println!("{}", "Step 9. Own WinRM_User!".green());
    println!("{}", "10. Quit".green());
}

fn get_user_choice() -> String {
    print!("Enter your choice: ");
    let _ = stdout().flush();

    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Failed to read input");
    choice.trim().to_string()
}

fn display_next_steps() {
    println!("Please follow these instructions for the next step:");
    println!("1. Run the provided Windows release of this program on a Windows Server or Windows 10 Pro/Enterprise machine (Home edition will NOT work).");
    println!("2. Ensure the Windows machine is connected to the HTB VPN.");
    println!("3. Place PowerView in the same directory as this AutoPwn script.");
    println!("4. Run the Windows release of this program as an administrator.");
    println!("5. After completing Step 8 on the Windows machine, quickly reconnect the VPN on your Linux machine and continue with Step 9.");
}

fn display_windows_warning() {
    println!("Please Run this Step on Windows......");
}

fn run_steps_1_to_4() {
    step_1::run();
    step_2::run();
    step_3::run();
    step_4::run();
}