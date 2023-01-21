use regex::Regex;
use requestty::{prompt, Question};
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::exit;
use subprocess::{Popen, PopenConfig, Redirection};

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

    // First thing's first, need to check if there is a save file with the list of repositories.
    // If there is not, create one. If there is, load them into a vector. This vector will be saved to the file anytime
    // a repository is added to or removed from the list, or on quit.
    let repository_list_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("repos.txt")
    {
        Ok(file) => file,
        Err(_) => {
            println!("fatal: Could not open or create repository list at start!");
            exit(1);
        }
    };

    // Read all lines from it and add to list of repositories
    let mut repo_list: Vec<String> = Vec::new();
    let bufreader = BufReader::new(repository_list_file.try_clone().unwrap());
    for line in bufreader.lines() {
        match line {
            Ok(text) => {
                repo_list.push(text);
            }
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
                "Quit".to_string(),
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
                                .validate(|repo, _previous_answers| 
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
                                            let mut repository_list_file = match OpenOptions::new()
                                                .append(true)
                                                .open("repos.txt")
                                            {
                                                Ok(file) => file,
                                                Err(_) => {
                                                    println!("fatal: Could not open or create repository list!");
                                                    exit(1);
                                                }
                                            };
                                            writeln!(repository_list_file, "{}", repo)
                                                .expect("fatal: Error writing to file.");
                                        }
                                    }
                                }
                                Err(_) => {
                                    println!("There was an error processing your input...");
                                }
                            }
                        }
                        // Remove a repository
                        1 => {
                            // Get a directory from the prompt
                            let directory_question = Question::input("add-repo")
                                .message("Enter path to repository to remove from sync system: ")
                                .validate(|repo, _previous_answers| {
                                    if Path::new(repo).is_dir()
                                        && Path::new(&format!("{}\\.git", repo)).is_dir()
                                    {
                                        Ok(())
                                    } else {
                                        Err("Please enter a valid git repository.".to_string())
                                    }
                                })
                                .build();
                            let repo_dir_res = prompt(vec![directory_question]);
                            match repo_dir_res {
                                Ok(answers) => {
                                    // Add it to the memory-resident list and
                                    // serialize it to the file
                                    for ans in answers {
                                        let repo = ans.1.as_string().unwrap().to_string();
                                        if repo_list.contains(&repo) {
                                            repo_list.remove(
                                                repo_list
                                                    .iter()
                                                    .position(|r| *r == repo.clone())
                                                    .unwrap(),
                                            );
                                            println!("Successfully removed '{}' from sync list. This did NOT remove it from your disk.", repo);
                                            println!("This change will be serialized when you soft quit the program.")
                                        } else {
                                            println!("No such repository found in sync list...");
                                        }
                                    }
                                }
                                Err(_) => {
                                    println!("There was an error processing your input...");
                                }
                            }
                        }
                        // Sync all repositories
                        2 => {
                            for repo in repo_list.iter() {
                                // Check git is installed
                                let pr = Popen::create(
                                    &["cmd.exe", "/c", format!("cd {} && git pull", repo).as_str()],
                                    PopenConfig {
                                        stdout: Redirection::Pipe,
                                        ..Default::default()
                                    },
                                );

                                match pr {
                                    Ok(mut pop) => {
                                        println!("Ran git pull on {}", repo);
                                        let (out, _err) = pop.communicate(None).unwrap();
                                        println!(
                                            "Output: {}",
                                            out.unwrap_or("No output".to_string())
                                        );
                                        if let Some(_status) = pop.poll() {
                                            // Program already exit correctly
                                        } else {
                                            // Terminate the program
                                            pop.terminate().unwrap();
                                        }
                                    }
                                    Err(_e) => {
                                        println!("Could not open cmd.exe.");
                                        exit(1);
                                    }
                                }
                            }
                        }
                        // Exit program
                        3 => {
                            let mut conglomerate_string = String::new();
                            for repo in repo_list {
                                conglomerate_string.push_str(repo.as_str());
                                conglomerate_string.push('\n');
                            }

                            let mut repository_list_file = match OpenOptions::new()
                                .write(true)
                                .truncate(true)
                                .create(true)
                                .open("repos.txt")
                            {
                                Ok(file) => file,
                                Err(_) => {
                                    println!("fatal: Could not open or create repository list!");
                                    exit(1);
                                }
                            };
                            write!(repository_list_file, "{}", conglomerate_string)
                                .expect("fatal: Could not save to file");
                            exit(0);
                        }
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
