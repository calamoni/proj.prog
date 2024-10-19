use std::path::path; 
use std::env;

fn main() {
    let os = env::consts::OS;


    match os {
        "windows" => {
            let path = Path::new("C:\\Windows\\System32\\snmp.exe");

            if path.exists() {
                println!("The file exists... Parsing");
            } else {
                println!("The file does not exist!");
            }
        } 
        "linux" => {
            let path1 = Path::new("/etc/snmp/snmpd.conf");
            let path2 = Path::new("/etc/snmp/snmp.conf");

            if path1.exists() {
                println!("The smpd.conf file exists... parsing");
            } else if path2.exists() {
                println!("The snmp.conf file exists... parsing");
            } else {
                println!("Neither file exists!");
            }
        }
        _ => {
            println!("Unsupported operating system");
        }
    }
}