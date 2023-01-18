use std::process::exit;
use requestty::{Question, question::Choice::Choice, Answers, Answer, prompt};
use subprocess::{Popen, PopenConfig, Redirection};
use regex::Regex;

/// Ensure a git executable is installed and on path.
/// 
/// WARNING: If git is not installed, this function DOES NOT RETURN.
fn check_git_installed() {
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

/// Show main menu 
fn show_menu() { 
    // Show the menu indefinitely, let user exit when they want to exit
    loop { 
        let main_menu = Question::select("menu")
            .message("Welcome to Git-Sync: ")
            .choices(vec![
                "Add a repository".to_string(),
                "Sync all repositories".to_string(),
                "Quit".to_string()
            ])
            .default(2)
            .build();
        
        let selection = prompt(vec![main_menu]);
        if let Ok(sel) = selection {
            for ans in sel.values() {
                if ans.is_list_item() {
                    if ans.as_list_item().expect("nothing selected").text == "Quit" {
                        exit(0);
                    }
                }
            }
        }
    }
}

fn main() {
    // Check git is installed
    check_git_installed();

    // Can now proceed to show menu
    show_menu();
}
