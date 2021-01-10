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
        // 2. Every URI scheme registered with the current Windows OS has subkey: "shell\open\command" which containing tuples: "(Default)":REG_SZ:"Path_To_Binary_Executable"
        // 3. In root subkey, there are tuples: "(Default)":REG_SZ:"URL:protocol-friendly-name" and "URL Protocol":REG_SZ:""
        if i.chars().all(char::is_alphanumeric) && i.chars().count()<MAX_LEN+1 {
            let mut subkey2check: String = i.clone();
            subkey2check.push_str("\\shell\\open\\command");
            let mut flag2 = true; // true = failed the checkpoint
            let mut flag3 = true; // true = failed the checkpoint

            let exe_subkey_check = classroot.open_subkey(subkey2check);

            match exe_subkey_check {
                Ok(subkey) => {
                    for (name2, value2) in subkey.enum_values().map(|x| x.unwrap()) {
                        if name2=="" {
                            flag2 = false; // If can't find the (Default) key, checkpoint 2 does not pass.
                            match value2.vtype {
                                REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                                    let value2str = String::from(format!("{}", value2));
                                    let rootkey_check = classroot.open_subkey(String::from(format!("{}",i)));
                                    match rootkey_check {
                                        Ok(rootkey) => {
                                            // Look for 
                                            // "(Default)":REG_SZ:"URL:protocol-friendly-name" 
                                            // "URL Protocol":REG_SZ:""
                                            let mut found_friendlyname = false;
                                            let mut found_protocol = false;                                            
                                            for (name3, value3) in rootkey.enum_values().map(|x| x.unwrap()) {
                                                if name3=="" {
                                                    match value3.vtype {
                                                        REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => found_friendlyname = true,
                                                        _ => found_friendlyname = false,
                                                    }
                                                } else if name3=="URL Protocol" {
                                                    match value3.vtype {
                                                        REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                                                            if String::from(format!("{}",value3))==String::from("") {
                                                                found_friendlyname = true;
                                                            }                                                            
                                                        },
                                                        _ => found_friendlyname = false,
                                                    }
                                                }
                                            }

                                            if found_friendlyname==false || found_protocol==false {
                                                flag3 = true;
                                            } else {
                                                // Passed all checkpoints! 
                                                // Save these entries from subkey(s):
                                                // i
                                                //      "(Default)":REG_SZ:"URL:protocol-friendly-name"
                                                // i\shell\open\command
                                                //      "(Default)":REG_SZ:"Path_To_Binary_Executable"
                                            }

                                        },
                                        Err(e) => panic!("{} - Is registry edited?", e), 
                                    }

                                    // println!("{}    {}", i, valuestr);
                                },
                                _ => flag2 = true,
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

    println!("===== Summary =====");
    println!("{}    Found", success_cntr);
    println!("{}    Failed due to non-alphanumeric or length", naming_fail_cntr);
    println!("{}    Missing subkey shell\\open\\command", subkey_fail_cntr);
    println!("{}    Missing or malformed entries in root", malform_cntr);
    

    // let system = RegKey::predef(HKEY_LOCAL_MACHINE)
    //     .open_subkey("HARDWARE\\DESCRIPTION\\System")?;
    // for (name, value) in system.enum_values().map(|x| x.unwrap()) {
    //     println!("{} = {:?}", name, value);
    // }

    Ok(())
}