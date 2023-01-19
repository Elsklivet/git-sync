use std::process::exit;
use requestty::{Question, question::Choice::Choice, Answers, Answer, prompt};
use subprocess::{Popen, PopenConfig, Redirection};
use regex::Regex;

fn main() {
    // Check git is installed
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

    // TODO
    // First thing's first, need to check if there is a save file with the list of repositories.
    // If there is not, create one. If there is, load them into a vector. This vector will be saved to the file anytime
    // a repository is added to or removed from the list, or on quit.

    // Can now proceed to show menu
    // Show the menu indefinitely, let user exit when they want to exit
    println!("Welcome to git-sync!");
    loop { 
        let main_menu = Question::select("menu")
            .message("Select an action:")
            .choices(vec![
                "Add a repository".to_string(),
                "Remove a repository".to_string(),
                "Sync all repositories".to_string(),
                "Help".to_string(),
                "Quit".to_string()
            ])
            .default(0)
            .build();
        
        let selection = prompt(vec![main_menu]);
        if let Ok(sel) = selection {
            for ans in sel.values() {
                if ans.is_list_item() {
                    match ans.as_list_item().expect("nothing selected").index {
                        // Add a repository
                        0 => {},
                        // Remove a repository
                        1 => {},
                        // Sync all repositories
                        2 => {},
                        // Print help
                        3 => {},
                        // Exit program
                        4 => {
                            exit(0)
                        },
                        // This should never happen
                        _ => { 
                            println!("I didn't understand that...");
                        }
                    }
                }
            }
        }
    }
}
