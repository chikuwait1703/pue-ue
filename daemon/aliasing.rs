use ::anyhow::Result;
use ::log::{info, warn};
use ::std::collections::HashMap;
use ::std::fs::File;
use ::std::io::prelude::*;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use ::pueue::linux::directories::get_config_directories;

#[cfg(target_os = "macos")]
use ::pueue::macos::directories::get_config_directories;

#[cfg(target_os = "windows")]
use ::pueue::windows::directories::get_config_directories;

/// Return the contents of the alias file, if it exists and can be parsed.
/// The file should be located in `pueue_directory` and named `pueue_aliases`.
pub fn get_aliases() -> Result<HashMap<String, String>> {
    // Go through all config directories and check for a alias file.
    let mut alias_file_path = None;
    for directory in get_config_directories()? {
        let path = directory.join("pueue_aliases.yml");
        if path.exists() {
            alias_file_path = Some(path);
        }
    }

    // Return early if we cannot find the file
    let alias_file_path = match alias_file_path {
        None => {
            info!("Didn't find pueue alias file.");
            return Ok(HashMap::new());
        }
        Some(alias_file_path) => alias_file_path,
    };

    // Read the file content
    let mut alias_file = File::open(alias_file_path)?;
    let mut content = String::new();
    alias_file.read_to_string(&mut content)?;

    Ok(serde_yaml::from_str(&content)?)
}

/// Check if there exists an alias for a given command.
/// Only the first word will be replaced. The separator is a space.
pub fn insert_alias(command: String) -> String {
    let first = match command.split_whitespace().next() {
        Some(first) => first,
        None => return command,
    };

    let aliases = match get_aliases() {
        Err(err) => {
            warn!("Failed to open aliases file: {}", err);
            return command;
        }
        Ok(aliases) => aliases,
    };

    for (original, alias) in aliases.iter() {
        if original == first {
            return command.replacen(original, alias, 1);
        }
    }

    command
}
