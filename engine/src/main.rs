use std::io::Write;
use std::os::unix::net::UnixListener;

fn main() -> std::io::Result<()> {
    let listener_path = "/tmp/bulwark.sock";
    let listener = UnixListener::bind(listener_path)?;

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
