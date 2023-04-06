use std::io::{stdin, stdout, Write};
mod step_8;

pub fn main() {
    loop {
        println!("\nSelect an option:");
        println!("Step 8. MUST BE RAN ON WINDOWS!");
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
                println!("Syncing time with absolute.htb");
                step_8::sync_windows_time();
                //println!("Setting absolute.htb as Primary DNS...");
                //step_8::set_primary_dns();
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
