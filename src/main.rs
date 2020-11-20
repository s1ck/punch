mod args;

extern crate clap;

use clap::{App, Arg, SubCommand, ArgMatches};
use args::args;
use crate::args::Action;

fn main() {
    let action = args();
    println!("{:?}", action);
}

#[cfg(test)]
mod tests {
    use super::*;
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
