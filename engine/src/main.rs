mod state;
mod models;
mod network;

use std::fs;
use std::io::Write;
use std::os::unix::net::UnixListener;

use crate::network::arp;
use crate::state::machine;

fn main() -> std::io::Result<()> {
    // let path = "/tmp/bulwark.sock";
    //
    // let _ = fs::remove_file(path);
    //
    // let listener = UnixListener::bind(path)?;
    //
    // let json = r#"
    // {
    //     "code": 123,
    //     "cool-word": "chicken"
    // }
    // "#;
    //
    // println!("[*] Waiting for connection");
    //
    // let (mut stream, _) = listener.accept()?;
    //
    // println!("[+] Server connected");
    //
    // loop {
    //     stream.write_all(json.as_bytes())?;
    //     stream.write_all(b"\n")?;
    //
    //     println!("[+] JSON sended");
    //
    //     std::thread::sleep(
    //         std::time::Duration::from_secs(2)
    //     );
    // }
    
    let mut state = machine::NetworkState::new();
    let arp_result = arp::scan();

    for result in arp_result {
        state.add_device(result);
    }

    for device in &state.devices {
        println!("IP: {}\nMAC: {}", device.ip, device.mac);
    }


    Ok(())
}
