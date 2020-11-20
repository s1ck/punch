mod args;
mod data;

extern crate clap;

use crate::data::{load, store};
use args::args;
use std::io;

fn main() -> Result<(), io::Error> {
    let data = load()?;
    let action = args();
    println!("{:?}", action);
    store(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_start() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("in")
            .arg("t1")
            .arg("t2")
            .assert()
            .stdout("START([\"t1\", \"t2\"])\n");
    }

    #[test]
    fn test_stop() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("out")
            .arg("t1")
            .arg("t2")
            .assert()
            .stdout("STOP([\"t1\", \"t2\"])\n");
    }

    #[test]
    fn test_list() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("list")
            .assert()
            .stdout("LIST\n");
    }
}
