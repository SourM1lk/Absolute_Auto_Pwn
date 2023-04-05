use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};

fn main() {
    // Get the IP address from the user
    let mut ip = String::new();
    print!("Enter IP address: ");
    let _ = stdout().flush();
    stdin().read_line(&mut ip).expect("Failed to read input");

    // Trim whitespace from IP address
    ip = ip.trim().to_string();

    // Construct the new host entry
    let host_entry = format!("{}\tabsolute.htb", ip);

    // Check if the IP address already exists in the /etc/hosts file
    let output = Command::new("grep")
        .arg(&ip)
        .arg("/etc/hosts")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        // Use the `echo` and `sudo tee -a` commands to append the host entry to /etc/hosts
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

    // Menu to Select step to pwn machine
    loop {
        println!("\nSelect an option:");
        println!("1. Option 1");
        println!("2. Option 2");
        println!("3. Option 3");
        println!("4. Quit");

        print!("Enter your choice: ");
        let _ = stdout().flush();

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("You selected Option 1");
                // Perform the desired action for option 1
            }
            "2" => {
                println!("You selected Option 2");
                // Perform the desired action for option 2
            }
            "3" => {
                println!("You selected Option 3");
                // Perform the desired action for option 3
            }
            "4" => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
