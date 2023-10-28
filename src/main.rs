use std::{fs, env::{args, current_dir}, process::exit};

use save_load::Mode;

mod dir;
mod save_load;

fn main() {
    let arguments: Vec<String> = args().collect();

    if arguments.len() >! 1 {
        eprintln!("Please enter one of the following modes (not case sensitive):\nsave\nload");
        exit(1)
    }

    println!("Running");

    let mode: Mode = if arguments[1].to_ascii_lowercase() == "save" {
        println!("save");
        Mode::Save
    } else if arguments[1].to_ascii_lowercase() == "load" {
        println!("load");
        Mode::Load
    } else {
        eprintln!("This is not a valid mode.");
        exit(1)
    };

    let current_directory: String = if let Err(err) = current_dir() {
        eprintln!("Directory error!\nInfo: {:?}\nExiting", err);
        return;
    } else if let Ok(path) = current_dir() {
        path.to_str().unwrap_or("").to_owned()
    } else {
        // I don't know why I need an else expression here, since current_dir() returns a Result<T, E> type.
        todo!();
    };

    match dir::saves_dir_exists(&current_directory) {
        // I know all of this can be written in a better way.
        // I just want to focus on working code, rather than clean code.
        Ok(exists) => {
            if exists {
                match mode {
                    Mode::Save => {
                        save_load::save()
                    }

                    Mode::Load => {
                        save_load::load()
                    }
                }
            } else {
                match mode {
                    Mode::Save => {
                        fs::create_dir(format!("{}\\saves", &current_directory)).unwrap();
                        save_load::save()
                    }

                    Mode::Load => {
                        eprintln!("You can't load your data without having saved data first!");
                        exit(1)
                    }
                }
            }
        }

        Err(e) => {
            eprintln!("Directory error!\nInfo: {:?}\nExiting", e)
        }
    }
}
