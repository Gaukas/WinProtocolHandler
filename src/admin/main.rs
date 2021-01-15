extern crate winreg;
extern crate winres;

use std::io;
// use win_protocol_handler::*;

fn main() -> io::Result<()> {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#);
    
    print!("Hello World!");

    //add_protocol(&mut list_entries, String::from("zgaukas"), String::from("URL:ZGAUKAS PROTOCOL"), String::from("\"D:\\ZGAUKAS\\z.exe\""));

    Ok(())
}
