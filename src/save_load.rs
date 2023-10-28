use std::{fs::copy, env, process::exit};

const FILE_NAMES: [&str; 4] = ["CCGameManager.dat", "CCGameManager2.dat", "CCLocalLevels.dat", "CCLocalLevels2.dat"];

pub enum Mode {
    Save,
    Load
}

pub fn save() {
    let saves_dir: &str = "saves\\";
    let gd_path =  if let Some(local_appdata) = env::var("LOCALAPPDATA").ok() {
        format!("{}\\GeometryDash\\", local_appdata)
    } else {
        eprintln!("No LOCALAPPDATA environment variable!");
        exit(1);
    };

    // assuming all of the needed files and folders exist
    for i in 0..4 {
        copy(format!("{}{}", gd_path, FILE_NAMES[i]), saves_dir.to_owned() + FILE_NAMES[i]).unwrap();
    }
}

pub fn load() {
    let saves_dir: &str = "saves\\";
    let gd_path =  if let Some(local_appdata) = env::var("LOCALAPPDATA").ok() {
        format!("{}\\GeometryDash\\", local_appdata)
    } else {
        eprintln!("No LOCALAPPDATA environment variable!");
        exit(1);
    };

    // assuming all of the needed files and folders exist
    for i in 0..4 {
        copy(saves_dir.to_owned() + FILE_NAMES[i], format!("{}{}", gd_path, FILE_NAMES[i])).unwrap();
    }

}
