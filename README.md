## Description
```
This program is designed to help you achieve user and root access on the HackTheBox machine, Absolute. It is structured to execute tasks in a sequential order, and unless specified otherwise, steps cannot be skipped.

As of now, the program will only help you obtain user access. Root access support will be added in the future.

Good Luck Have Fun!
```
## Installation

### On your linux attacker machine
1. Close the repository
```
https://github.com/SourM1lk/Absolute_Auto_Pwn.git
```
2. Install Rust, Cargo, and Rustc:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Build Release files
```
# Windows .exe
cargo build --release --target=x86_64-pc-windows-gnu --bin Absolute_Auto_Pwn_windows

# Linux Executable
cargo build --release --bin Absolute_Auto_Pwn
```

Alternatively, you can download the release files for both Linux and Windows.


4. Ensure the following tools are installed and available in your /usr/bin directory:
```
tee
wget
exiftool
kerbrute
impacket-GetNPUsers
john
timedatectl
ntpdate
impacket-getTGT
crackmapexec
impacket-smbclient (not required unless you perform step 5)
bloodhound-python (not required unless you perform step 6)
```
5. The following tools are required, but the program will ask for their location:
```
pywhisker
gettgtpkinit
```
6. Make sure you have these wordlists:
```
rockyou.txt
```

### On your windows attacker machine (yes its required)
```
1. Windows Server or Windows 10 Pro/Enterprise
2. Openvpn installed with your .ovpn file for HTB
3. Sync your Windows time to absolute.htb
4. On your OpenVPN LAN Adapter set the primary DNS as the absolute.htb IP address

Note: The Auto Pwn program will try to set these, but more then likley fail, until I spend more time on this
```




## Tools/Commands Required
```
# Warning: This tool will not work if these commands are not availiable to the user.
tee
wget
exiftool
kerbrute
impacket-GetNPUsers
john
timedatectl
ntpdate
impacket-getTGT
crackmapexec
impacket-smbclient
bloodhound-python
PowerView.ps1 (on Windows)
ActiveDirectory Module (windows)
pywhisker
gettgtpkinit
```

## Other Comments/Notes
```
I created this project as a way to learn Rust and have some fun in the process.

This program has been tested on Kali Linux and Windows 10 Enterprise.

If the program encounters any issues during a specific step, refer to the source code to understand what it's doing, and try to execute the step manually.

Keep in mind that this is an insane-level box, so challenges are to be expected....
```