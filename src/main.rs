use std::io::{BufReader, BufRead};
use std::fs;
use std::thread;
use std::process::{Command, Stdio};
use std::error::Error;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let procfile = fs::read_to_string("Procfile")
        .expect("Could not open Procfile");

    let re = Regex::new(r"\s*([^:]+):\s*(.+)").unwrap();

    let processes: Vec<_> = re
        .captures_iter(&procfile)
        .map(|c| c.extract())
        .map(|(_, [name, command])| (name, command))
        .collect();

    let name_max_length = processes.iter().fold(0, |acc, (name, _)| std::cmp::max(name.len(), acc));

    let exit_threads: Vec<_> = processes
        .iter()
        .map(|(name, command)| {
            let mut child = Command::new("bash")
                .args(["-c", command])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Could not run bash");

            let mut stdout = BufReader::new(child.stdout.take().unwrap());
            let mut stderr = BufReader::new(child.stderr.take().unwrap());

            let name_padded = format!("{:width$}", name, width=name_max_length);
            let name_padded2 = name_padded.clone();
            let name_padded3 = name_padded.clone();

            thread::spawn(move || {
                let mut line = String::new();

                loop {
                    line.clear();
                    let len = stdout.read_line(&mut line).unwrap();
                    if len == 0 {
                        break
                    }
                    println!("{name_padded} | {}", line.trim_end());
                }
            });

            thread::spawn(move || {
                let mut line = String::new();

                loop {
                    line.clear();
                    let len = stderr.read_line(&mut line).unwrap();
                    if len == 0 {
                        break
                    }
                    println!("{name_padded2} | {}", line.trim_end());
                }
            });

            return thread::spawn(move || {
                match child.wait() {
                    Ok(status) => println!("{name_padded3} | exited with {status}"),
                    Err(_) => todo!(),
                }
            });
        })
        .collect();

    for t in exit_threads {
        let _ = t.join();
    }

    Ok(())
}
