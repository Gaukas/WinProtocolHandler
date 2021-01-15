extern crate winreg;
use std::io;
use win_protocol_handler::*;

fn main() -> io::Result<()> {
    let mut list_entries = Vec::<(String, String, String)>::new();
    
    scan_protocol(&mut list_entries, true);

    //add_protocol(&mut list_entries, String::from("zgaukas"), String::from("URL:ZGAUKAS PROTOCOL"), String::from("\"D:\\ZGAUKAS\\z.exe\""));

    Ok(())
}
