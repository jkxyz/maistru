use std::{process::{Command, Child}, fs};

use regex::Regex;

fn main() {
    let procfile = fs::read_to_string("Procfile")
        .expect("Could not open Procfile");

    let re = Regex::new(r"\s*([^:]+):\s*(.+)").unwrap();

    let processes: Vec<_> = re
        .captures_iter(&procfile)
        .map(|c| c.extract())
        .map(|(_, [name, command])| (name, command))
        .collect();

    let children: Vec<Child> = processes
        .into_iter()
        .map(|(_name, command)| {
            Command::new("bash")
                .args(["-c", command])
                .spawn()
                .expect("Failed to spawn child")
        })
        .collect();

    for mut child in children {
        child.wait().unwrap();
    }
}
