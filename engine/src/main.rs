use std::fs;
use std::io::Write;
use std::os::unix::net::UnixListener;

fn main() -> std::io::Result<()> {
    let path = "/tmp/bulwark.sock";

    let _ = fs::remove_file(path);

    let listener = UnixListener::bind(path)?;

    println!("waiting for connection...");

    let (mut stream, _) = listener.accept()?;

    println!("go connected!");

    let json = r#"
{
    "code": "123",
    "cool-word": "chicken"
}
        "#;

    stream.write_all(json.as_bytes())?;

    Ok(())
}
