use std::process::Command;

pub fn run() {
    println!("Downloading test.exe from share...");
    download_test_exe();
}

fn download_test_exe() {
    // Prepare the impacket-smbclient command with the specified arguments
    let output = Command::new("impacket-smbclient")
        .arg("\\\\dc.absolute.htb\\Shared")
        .arg("-k")
        .arg("-no-pass")
        .arg("-U")
        .arg("svc_smb@dc.absolute.htb")
        .arg("--command")
        .arg("get test.exe")
        .output()
        .expect("failed to execute impacket-smbclient");

    // Print the command output to the console
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
