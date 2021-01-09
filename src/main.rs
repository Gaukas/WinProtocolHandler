extern crate winreg;
use std::io;
use winreg::RegKey;
use winreg::enums::*;

const MAX_LEN: usize = 8;

fn main() -> io::Result<()> {
    let classroot = RegKey::predef(HKEY_CLASSES_ROOT);
    let mut naming_fail_cntr = 0;
    let mut subkey_fail_cntr = 0;
    let mut malform_cntr = 0;
    let mut success_cntr = 0;
    println!("Protocols registered:");
    for i in RegKey::predef(HKEY_CLASSES_ROOT).enum_keys().map(|x| x.unwrap())
    {
        // A few assumptions:
        // 1. URI scheme is always alphanumeric and not "too long"
        // 2. Every URI scheme registered with the current Windows OS has subkey: "shell\open\command"
        // 3. In root subkey, there are tuples: "(Default)":REG_SZ:"URL:protocol-friendly-name" and "URL Protocol":REG_SZ:""
        // println!("{}",i);
        if i.chars().all(char::is_alphanumeric) && i.chars().count()<MAX_LEN+1 {
            // print!("{}: ", i);
            let mut subkey2check: String = i.clone();
            subkey2check.push_str("\\shell\\open\\command");
            let mut flag2 = false;
            let mut flag3 = false;

            let exe_subkey_check = classroot.open_subkey(subkey2check);

            match exe_subkey_check {
                Ok(subkey) => {
                    for (name, value) in subkey.enum_values().map(|x| x.unwrap()) {
                        if name=="" {
                            match value.vtype {
                                REG_SZ => {
                                    let valuestr = String::from(format!("{}", value));
                                    println!("{}    {}", i, valuestr);
                                },
                                _ => flag3 = true,
                            }
                        }
                    }
                },
                Err(_e) => flag2 = true,
            }

            if flag2 == true {
                subkey_fail_cntr+=1;
                continue;
            }

            if flag3 == true {
                malform_cntr+=1;
                continue;
            }

            success_cntr+=1;
        } else {
            naming_fail_cntr+=1;
        }
    }

    println!("=== Summary ===");
    println!("{}    Found", success_cntr);
    println!("{}    Failed due to non-alphanumeric or length", naming_fail_cntr);
    println!("{}    Missing subkey", subkey_fail_cntr);
    println!("{}    Malformed key values types", malform_cntr);
    

    // let system = RegKey::predef(HKEY_LOCAL_MACHINE)
    //     .open_subkey("HARDWARE\\DESCRIPTION\\System")?;
    // for (name, value) in system.enum_values().map(|x| x.unwrap()) {
    //     println!("{} = {:?}", name, value);
    // }

    Ok(())
}