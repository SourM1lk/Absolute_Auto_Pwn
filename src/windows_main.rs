use std::io::{stdin, stdout, Write};
mod step_8;

pub fn main() {
    display_intro();
    
    loop {
        println!("\nSelect an option:");
        println!("Step 8.  Update Windows hosts File");
        println!("Step 8a. Sync Time with absolute.htb");
        println!("Step 8b. Set absolute.htb as Primary DNS");
        println!("Step 8c. Add m.lovegod as NetWork Audit Member");
        println!("9. Quit");

        print!("Enter your choice: ");
        let _ = stdout().flush();

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "8" => {
                println!("Updating Host File...");
                step_8::update_hosts_file();
            }
            "8a" => {
                println!("Syncing time with absolute.htb");
                step_8::sync_windows_time();
            }
            "8b" => {
                println!("Setting absolute.htb as Primary DNS...");
                step_8::set_primary_dns();
            }
            "8c" => {
                println!("Adding m.lovegod as Network Audit Member");
                step_8::run_powershell_commands();
                println!("Assuming you did not get any errors. Log back on your Linux and continue the steps...")
            }
            "9" => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

fn display_intro() {
    println!("Please ensure the following prerequisites are met on your Windows machine:");
    println!("1. Windows Server or Windows 10 Pro/Enterprise");
    println!("2. OpenVPN installed with your .ovpn file for HTB and is running");
    println!("3. PowerView.ps1 in the same directory as the program");
    println!("4. ActiveDirectory Module installed on the Windows system");

    println!("\nNote: If the Auto Pwn program steps 8a and 8b are selected, it will attempt to sync your Windows internet time with absolute.htb and set the absolute.htb IP as the primary DNS on your OpenVPN LAN Adapter. However, this is likely to fail, and you may need to perform these steps manually.");
}
