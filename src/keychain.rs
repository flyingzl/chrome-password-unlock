use crate::models::DecryptError;
use security_framework::passwords::get_generic_password;

/// Chrome service and account names in keychain
const CHROME_SERVICE: &str = "Chrome Safe Storage";
const CHROME_ACCOUNT: &str = "Chrome";

/// Get Chrome master password from macOS keychain
///
/// Chrome stores the master password in the system keychain
/// Service name: "Chrome Safe Storage"
/// Account name: "Chrome"
pub fn get_chrome_master_password() -> Result<String, DecryptError> {
    tracing::debug!("Attempting to retrieve Chrome master password from keychain");
    let password_bytes = get_generic_password(CHROME_SERVICE, CHROME_ACCOUNT)
        .map_err(|e| DecryptError::KeychainError(format!("Failed to get password: {}", e)))?;

    tracing::debug!("Successfully retrieved master password from keychain");
    String::from_utf8(password_bytes)
        .map_err(|e| DecryptError::KeychainError(format!("Invalid UTF-8 in password: {}", e)))
}

/// Cache master password to local file (optional)
pub fn cache_master_password(password: &str) -> Result<(), DecryptError> {
    let cache_dir = dirs::home_dir()
        .ok_or_else(|| DecryptError::IoError("Cannot find home directory".to_string()))?
        .join(".chrome-password-unlock");

    std::fs::create_dir_all(&cache_dir)?;

    let cache_file = cache_dir.join("master_password");
    std::fs::write(cache_file, password)?;
    tracing::debug!("Cached master password to local file");

    Ok(())
}

/// Load master password from cache file
pub fn load_cached_master_password() -> Result<String, DecryptError> {
    let cache_dir = dirs::home_dir()
        .ok_or_else(|| DecryptError::IoError("Cannot find home directory".to_string()))?
        .join(".chrome-password-unlock");

    let cache_file = cache_dir.join("master_password");

    if cache_file.exists() {
        tracing::debug!("Loading master password from cache");
        std::fs::read_to_string(cache_file)
            .map_err(|e| DecryptError::IoError(format!("Failed to read cache: {}", e)))
    } else {
        tracing::debug!("No cached master password found");
        Err(DecryptError::IoError("Cache file not found".to_string()))
    }
}

/// Get master password (prefer to read from cache first)
pub fn get_master_password_with_cache() -> Result<String, DecryptError> {
    // Try to read from cache
    if let Ok(password) = load_cached_master_password() {
        tracing::info!("Using cached master password");
        return Ok(password);
    }

    // Get from keychain
    tracing::info!("Retrieving master password from keychain");
    let password = get_chrome_master_password()?;

    // Cache to local
    let _ = cache_master_password(&password);

    Ok(password)
}
