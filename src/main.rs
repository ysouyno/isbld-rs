use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    // see: https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#continuously-process-child-process-outputs
    let stdout = Command::new("CMD")
        // first of all you should exec `chcp 65001`
        .args(&["/C", "ping 127.0.0.1"])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())
}
