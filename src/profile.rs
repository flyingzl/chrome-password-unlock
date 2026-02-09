pub use crate::models::ChromeProfile;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

/// Get Chrome data directory
fn get_chrome_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir().map(|home| home.join("Library/Application Support/Google/Chrome"))
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Linux: ~/.config/google-chrome
        // Windows: %LOCALAPPDATA%\Google\Chrome\User Data
        dirs::config_dir().map(|dir| dir.join("google-chrome"))
    }
}

/// Find all Chrome profiles
///
/// Chrome profile directory structure:
/// - Default (default profile)
/// - Profile 1, Profile 2, ... (other profiles)
pub fn find_chrome_profiles() -> Vec<ChromeProfile> {
    info!("Searching for Chrome profiles");
    let mut profiles = Vec::new();

    let chrome_dir = match get_chrome_data_dir() {
        Some(dir) => dir,
        None => {
            warn!("Could not determine Chrome data directory");
            return profiles;
        }
    };

    if !chrome_dir.exists() {
        warn!(
            "Chrome data directory does not exist: {}",
            chrome_dir.display()
        );
        return profiles;
    }

    // Iterate through subdirectories under Chrome directory
    let entries = match fs::read_dir(&chrome_dir) {
        Ok(entries) => entries,
        Err(_) => return profiles,
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        // Skip files and hidden directories
        if !path.is_dir() {
            continue;
        }

        let profile_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        // Skip system directories
        if profile_name.starts_with('.') {
            continue;
        }

        // Check if Login Data file exists
        let login_data = path.join("Login Data");
        if !login_data.exists() {
            continue;
        }

        profiles.push(ChromeProfile {
            name: profile_name.to_string(),
            path,
            login_data_path: login_data,
        });
    }

    info!("Found {} Chrome profile(s)", profiles.len());
    // Sort by profile name (Default first)
    profiles.sort_by(|a, b| {
        if a.name == "Default" {
            std::cmp::Ordering::Less
        } else if b.name == "Default" {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    profiles
}

/// Find a specific Chrome profile by name
pub fn find_profile_by_name(name: &str) -> Option<ChromeProfile> {
    find_chrome_profiles().into_iter().find(|p| p.name == name)
}

/// Get the default Chrome profile
pub fn get_default_profile() -> Option<ChromeProfile> {
    find_profile_by_name("Default")
}
