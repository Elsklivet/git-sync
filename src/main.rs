use std::process::exit;
use subprocess::{Popen, PopenConfig, Redirection};
use regex::Regex;

fn main() {
    // Check that git is installed
    let pr = Popen::create(
        &["git", "--version"],
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    );

    match pr {
        Ok(pop) => {
            let mut p = pop;
            let (out, _err) = p.communicate(None).unwrap();
            let version_regex = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
            for cap in version_regex.captures_iter(&out.unwrap()) {
                println!("Found git version {}", &cap[1]);
            }

            if let Some(_status) = p.poll() {
                // Program already exit correctly
            } else {
                // Terminate the program
                p.terminate().unwrap();
            }
        }
        Err(_e) => {
            println!("No git installation found.");
            exit(1);
        }
    }
}
