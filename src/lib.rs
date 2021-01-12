extern crate winreg;
use std::io;
use winreg::RegKey;
use winreg::enums::*;

const MAX_LEN: usize = 8;

pub fn scan_protocol(protocol_list: &mut Vec::<(String, String, String)>) -> bool {
    // TO-DO: Move main() to here
    unimplemented!();
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

    let debug_flag: u8 = 1;
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let (key, disp) = hkcu.create_subkey(protocol)?;

    match disp {
        REG_CREATED_NEW_KEY => debug_flag+=1,
        _ => return debug_flag,
    }

    key.set_value("",&friendly_name[..])?;
    key.set_value("URL Protocol",&"\"\"")?;
    
    let (comm_key, comm_disp) = key.create_subkey("shell\\open\\command")?;
    
    match comm_disp {
        REG_CREATED_NEW_KEY => debug_flag+=1,
        _ => return debug_flag,
    }

    comm_key.set_value("", &path_to_exe[..])?;
}