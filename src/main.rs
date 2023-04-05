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

    if output.status.success() {
        println!("IP address already exists in /etc/hosts file");
        return;
    }

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

    println!("Added host entry: {}", host_entry);
}
