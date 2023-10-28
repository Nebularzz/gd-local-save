use std::fs;

#[derive(Debug)]
pub enum DirError { // I know these are overly descriptive, but I like readable code.
    SavesDirExistsSavesFileExists
}

pub fn saves_dir_exists(current_directory: &String) -> Result<bool, DirError> {
    match fs::metadata(current_directory.to_owned() + "\\saves") {
        // Checks if a file or folder with the name "saves" already exists.
        // Ok(data) means a file or folder with the name already exists.
        Ok(data) => {
            if data.is_dir() {
                // Folder exists
                Ok(true)
            } else {
                // File with the same name exists
                Err(DirError::SavesDirExistsSavesFileExists)
            }
        }

        // No folder exists!
        Err(_) => {
            Ok(false)
        }
    }
}
