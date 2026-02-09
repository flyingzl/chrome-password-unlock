//! Chrome Password Unlock - CPU (Chrome Password Unlock)
//!
//! This is a tool for unlocking Chrome browser saved passwords on macOS.
//!
//! # Features
//! - Get Chrome master password from macOS keychain
//! - Support multiple Chrome profiles
//! - Decrypt and display saved login information
//! - Support filtering by keyword
//! - Support JSON and table format output

pub mod crypto;
pub mod database;
pub mod keychain;
pub mod models;
pub mod output;
pub mod profile;

use crate::crypto::derive_key;
use crate::database::query_login_data;
use crate::keychain::get_master_password_with_cache;
use crate::models::LoginInfo;
use crate::profile::{ChromeProfile, find_chrome_profiles};

/// Chrome Password Unlocker (CPU - Chrome Password Unlock)
pub struct ChromePasswordUnlock {
    derived_key: Vec<u8>,
}

impl ChromePasswordUnlock {
    /// Create a new unlocker instance
    pub fn new() -> Result<Self, crate::models::DecryptError> {
        tracing::debug!("Initializing ChromePasswordUnlock");
        let master_password = get_master_password_with_cache()?;
        let derived_key = derive_key(&master_password);
        tracing::debug!("Successfully derived encryption key");

        Ok(Self { derived_key })
    }

    /// Decrypt passwords from a specific Chrome profile
    pub fn decrypt_from_profile(
        &self,
        profile: &ChromeProfile,
        keyword: Option<&str>,
    ) -> Result<Vec<LoginInfo>, crate::models::DecryptError> {
        tracing::debug!("Decrypting profile: {}", profile.name);
        let params = crate::models::QueryParams {
            keyword: keyword.map(|s| s.to_string()),
            derived_key: &self.derived_key,
        };

        query_login_data(&profile.login_data_path, &params)
    }

    /// Decrypt passwords from all Chrome profiles
    pub fn decrypt_from_all_profiles(
        &self,
        keyword: Option<&str>,
    ) -> Vec<(String, Result<Vec<LoginInfo>, crate::models::DecryptError>)> {
        let profiles = find_chrome_profiles();

        profiles
            .into_iter()
            .map(|profile| {
                let result = self.decrypt_from_profile(&profile, keyword);
                (profile.name, result)
            })
            .collect()
    }

    /// Get all available Chrome profiles
    pub fn list_profiles() -> Vec<ChromeProfile> {
        find_chrome_profiles()
    }
}

impl Default for ChromePasswordUnlock {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ChromePasswordUnlock")
    }
}
