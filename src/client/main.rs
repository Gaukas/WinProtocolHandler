extern crate winreg;
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;  // Optional. Only if the derive macro is used.
use std::io;
use win_protocol_handler::*;

fn main() -> io::Result<()> {
    

    let mut list_entries = Vec::<(String, String, String)>::new();
    
    scan_protocol(&mut list_entries, true);

    //add_protocol(&mut list_entries, String::from("zgaukas"), String::from("URL:ZGAUKAS PROTOCOL"), String::from("\"D:\\ZGAUKAS\\z.exe\""));

    Ok(())
}
