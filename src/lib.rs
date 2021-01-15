extern crate winreg;
use winreg::RegKey;
use winreg::enums::*;
use std::path::Path;
use std::io; 

const MAX_LEN: usize = 12;

pub fn scan_protocol(protocol_list: &mut Vec::<(String, String, String)>, verbose: bool) -> bool {
    // Purge the vector
    protocol_list.clear();
    
    let classroot = RegKey::predef(HKEY_CLASSES_ROOT);
    let mut naming_fail_cntr = 0;
    let mut subkey_fail_cntr = 0;
    let mut malform_cntr = 0;
    let mut success_cntr = 0;

    if verbose {
        println!("Protocols registered:");
    }
    
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
                                    //let value2str = String::from(format!("{}", value2));
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
                                                            // if String::from(format!("{}",value3))==String::from("\"\"") {
                                                                found_protocol = true;
                                                            // }                                                           
                                                        },
                                                        _ => found_protocol = true,
                                                    }
                                                }
                                                //println!("{} has {}", String::from(format!("{}",i)), name3);
                                            }

                                            if found_friendlyname==true && found_protocol==true {
                                                flag3 = false;
                                            }

                                            if flag3 == false
                                            {
                                                // Passed all checkpoints! 
                                                // Save these entries from subkey(s):
                                                // i
                                                //      "(Default)":REG_SZ:"URL:protocol-friendly-name"
                                                // i\shell\open\command
                                                //      "(Default)":REG_SZ:"Path_To_Binary_Executable"
                                                let path_to_exe: String = subkey.get_value("").unwrap();
                                                let friendly_name: String = rootkey.get_value("").unwrap();
                                                protocol_list.push((String::from(format!("{}",i)), friendly_name, path_to_exe));
                                                //println!("Pushed {}", String::from(format!("{}",i)));
                                            }
                                        },
                                        Err(_e) => return false, 
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

    if verbose {
        for (i, x) in protocol_list.iter().enumerate() {
            println!("{}  {}    {}    {}", i, x.0, x.1, x.2);
        }

        println!("===== Summary =====");
        println!("{}    Found", success_cntr);
        println!("{}    Failed due to non-alphanumeric or length", naming_fail_cntr);
        println!("{}    Missing subkey \"shell\\open\\command\"", subkey_fail_cntr);
        println!("{}    Missing or malformed entries in root", malform_cntr);
    }

    return true;
}

// Return 0 when success. Or return debug flag.
pub fn add_protocol(protocol_list: &mut Vec::<(String, String, String)>, protocol: String, friendly_name: String, path_to_exe: String) -> u8 {
    /*************************************************
     * Add a protocol:
     * 
     *  HKEY_CLASSES_ROOT/
     *      your-protocol-name/
     *          (Default)    "URL:your-protocol-name Protocol"
     *          URL Protocol ""
     *          shell/
     *              open/
     *                  command/
     *                      (Default) PathToExecutable
     * 
     *************************************************/

    let mut debug_flag: u8 = 1;
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let path = Path::new(&protocol[..]).join("");
    let (key, disp) = hkcr.create_subkey(&path).unwrap();

    match disp {
        REG_CREATED_NEW_KEY => debug_flag+=1,
        _ => return debug_flag,
    }

    key.set_value("",&friendly_name).unwrap();
    key.set_value("URL Protocol",&"").unwrap();
    
    let (comm_key, comm_disp) = key.create_subkey("shell\\open\\command").unwrap();
    
    match comm_disp {
        REG_CREATED_NEW_KEY => debug_flag+=1,
        _ => return debug_flag,
    }

    comm_key.set_value("", &path_to_exe).unwrap();

    scan_protocol(protocol_list, false);
    
    return debug_flag;
}

pub fn del_protocol(protocol: String, friendly_name: String) -> u8 {
    /*************************************************
     * Add a protocol:
     * 
     *  HKEY_CLASSES_ROOT/
     *      your-protocol-name/
     *          (Default)    "URL:your-protocol-name Protocol"
     *          URL Protocol ""
     *          shell/
     *              open/
     *                  command/
     *                      (Default) PathToExecutable
     * 
     *************************************************/

    let mut protocol_list = Vec::<(String, String, String)>::new();

    for p in protocol_list.iter() {
        if p.0 == protocol {
            if p.1 == friendly_name {
                let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
                let path = Path::new(&protocol[..]).join("");
                hkcr.delete_subkey_all(&path).unwrap();
                match hkcr.open_subkey(&path) {
                    Ok(_badkey) => {
                        scan_protocol(&mut protocol_list, false);
                        return 1;
                    },
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::NotFound => return 0,
                            _ => return 2,
                        }
                    },
                }
            } else {
                return 3;
            }
        }
    }
    return 3;
}