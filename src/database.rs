use crate::crypto::decrypt_password;
use crate::models::{DecryptError, LoginInfo, QueryParams};
use rusqlite::Connection;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Create a temporary copy of the database file
///
/// Chrome locks the Login Data file while running, so we need to copy it first
pub fn create_temp_db_copy(db_path: &Path) -> Result<std::path::PathBuf, DecryptError> {
    tracing::debug!("Creating temporary copy of database: {}", db_path.display());
    // Create temporary file
    let mut temp_path = std::env::temp_dir();
    temp_path.push(format!("chrome_login_data_{}.db", std::process::id()));

    // Copy database file
    std::fs::copy(db_path, &temp_path)?;
    tracing::debug!("Temporary database created at: {}", temp_path.display());

    // Set strict file permissions (user read/write only)
    let perms = Permissions::from_mode(0o600);
    std::fs::set_permissions(&temp_path, perms)
        .map_err(|e| DecryptError::IoError(format!("Failed to set permissions: {}", e)))?;

    Ok(temp_path)
}

/// Query login information from database
pub fn query_login_data(
    db_path: &Path,
    params: &QueryParams<'_>,
) -> Result<Vec<LoginInfo>, DecryptError> {
    tracing::debug!("Querying login data from: {}", db_path.display());
    let temp_db = create_temp_db_copy(db_path)?;

    // Ensure temp file is deleted when function ends
    let _guard = TempFileGuard(temp_db.clone());

    let conn = Connection::open(&temp_db)?;

    let mut result = Vec::new();

    // Use parameterized query to prevent SQL injection
    if let Some(keyword) = &params.keyword {
        tracing::debug!("Querying with keyword filter: '{}'", keyword);
        // Use parameterized query
        let search_pattern = format!("%{}%", keyword);
        let mut stmt = conn.prepare(
            "SELECT action_url, username_value, password_value FROM logins WHERE action_url LIKE ?",
        )?;

        let rows = stmt.query_map([&search_pattern], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?;

        for row in rows {
            let (url, username, encrypted_pwd) = row?;

            // Skip empty passwords
            if encrypted_pwd.is_empty() {
                continue;
            }

            // Decrypt password
            let password = match decrypt_password(params.derived_key, &encrypted_pwd) {
                Ok(pwd) => pwd,
                Err(e) => {
                    tracing::debug!("Failed to decrypt password for {}: {}", url, e);
                    continue; // Skip entries that failed to decrypt
                }
            };

            // Skip empty URLs
            if url.is_empty() {
                continue;
            }

            result.push(LoginInfo {
                url,
                username,
                password,
            });
        }
    } else {
        // No keyword filter, query all records
        let mut stmt =
            conn.prepare("SELECT action_url, username_value, password_value FROM logins")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?;

        for row in rows {
            let (url, username, encrypted_pwd) = row?;

            // Skip empty passwords
            if encrypted_pwd.is_empty() {
                continue;
            }

            // Decrypt password
            let password = match decrypt_password(params.derived_key, &encrypted_pwd) {
                Ok(pwd) => pwd,
                Err(e) => {
                    tracing::debug!("Failed to decrypt password for {}: {}", url, e);
                    continue; // Skip entries that failed to decrypt
                }
            };

            // Skip empty URLs
            if url.is_empty() {
                continue;
            }

            result.push(LoginInfo {
                url,
                username,
                password,
            });
        }
    }

    tracing::debug!("Query completed, found {} login(s)", result.len());
    Ok(result)
}

/// Temporary file guard, ensures temp file is deleted on Drop
struct TempFileGuard(std::path::PathBuf);

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        tracing::debug!("Cleaning up temporary file: {}", self.0.display());
        let _ = std::fs::remove_file(&self.0);
    }
}
