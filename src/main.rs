use std::process::exit;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error};
use requestty::{Question, question::Choice::Choice, Answers, Answer, prompt};
use subprocess::{Popen, PopenConfig, Redirection};
use regex::Regex;

fn main() {
    // Check git is installed
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
    let repository_list_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("repos.txt") {
            Ok(file) => file,
            Err(_) => { println!("fatal: Could not open or create repository list!"); exit(1); }
        };

    // Read all lines from it and add to list of repositories
    let mut repo_list: Vec<String> = Vec::new();
    let bufreader = BufReader::new(repository_list_file);
    for line in bufreader.lines() {
        match line {
            Ok(text) => {
                repo_list.push(text);
            },
            Err(_) => {
                println!("fatal: Error reading data from repos.txt file!");
                exit(1);
            }
        }
    }

    println!("Read {} repositories into system...", repo_list.len());

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
                        0 => {
                            // Get a directory from the prompt
                            let directory_question = Question::input("add-repo")
                                .message("Enter path to repository root directory: ")
                                .validate(|repo, previous_answers| 
                                    if Path::new(repo).is_dir() && Path::new(&format!("{}\\.git", repo)).is_dir() { 
                                        Ok(())
                                    } else {
                                        Err("Please enter a valid directory containing a git repository.".to_string())
                                    })
                                .build();
                            let repo_dir_res = prompt(vec![directory_question]);
                            match repo_dir_res {
                                Ok(answers) => {
                                    // Add it to the memory-resident list and 
                                    // serialize it to the file
                                    for ans in answers {
                                        let repo = ans.1.as_string().unwrap().to_string();
                                        if !repo_list.contains(&repo) {
                                            repo_list.push(repo.clone());
                                            println!("Successfully added '{}' to sync list.", repo);
                                            // Add to file here.
                                        }
                                    }
                                    
                                },
                                Err(_) => { 
                                    println!("There was an error processing your input...");
                                }
                            }
                        },
                        // Remove a repository
                        1 => {

                        },
                        // Sync all repositories
                        2 => {
                            
                        },
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
