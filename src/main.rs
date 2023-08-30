use std::{process::{Command, Child, Stdio}, fs, thread, io::{BufReader, BufRead}, str::FromStr};

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

    let mut children: Vec<_> = processes
        .into_iter()
        .map(|(name, command)| {
            (String::from_str(name).unwrap(),
             Command::new("bash")
             .args(["-c", command])
             .stdout(Stdio::piped())
             .stderr(Stdio::piped())
             .spawn()
             .unwrap())
        })
        .collect();

    for (name, child) in children.as_mut_slice() {
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let name = name.clone();
        thread::spawn(move || {
            loop {
                let mut line = String::new();
                let len = stdout.read_line(&mut line).unwrap();
                if len == 0 {
                    return
                }
                println!("{name} | {}", line.trim_end());
            }
        });
    }

    for (_, mut child) in children {
        child.wait().unwrap();
    }
}
