/// Login information model
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoginInfo {
    pub url: String,
    pub username: String,
    pub password: String,
}

/// Login information with profile name (for JSON output)
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoginInfoWithProfile {
    pub profile: String,
    #[serde(flatten)]
    pub info: LoginInfo,
}

/// Query parameters
#[derive(Debug)]
pub struct QueryParams<'a> {
    pub keyword: Option<String>,
    pub derived_key: &'a [u8],
}

/// Chrome profile information
#[derive(Debug, Clone)]
pub struct ChromeProfile {
    pub name: String,
    pub path: std::path::PathBuf,
    pub login_data_path: std::path::PathBuf,
}

/// Decryption error type
#[derive(Debug, thiserror::Error)]
pub enum DecryptError {
    #[error("Keychain access error: {0}")]
    KeychainError(String),

    #[error("Cryptography error: {0}")]
    CryptoError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Login data file not found")]
    LoginDataNotFound,
}

impl From<rusqlite::Error> for DecryptError {
    fn from(err: rusqlite::Error) -> Self {
        DecryptError::DatabaseError(err.to_string())
    }
}

impl From<std::io::Error> for DecryptError {
    fn from(err: std::io::Error) -> Self {
        DecryptError::IoError(err.to_string())
    }
}
