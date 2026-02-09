use anyhow::Result;
use chrome_password_unlock::ChromePasswordUnlock;
use chrome_password_unlock::models::LoginInfoWithProfile;
use chrome_password_unlock::output::print_results;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};

/// Chrome password unlock tool (CPU - Chrome Password Unlock)
#[derive(Parser)]
#[command(name = "chrome-password-unlock")]
#[command(about = "Chrome password unlock tool (CPU)", long_about = None)]
#[command(version)]
struct Cli {
    /// List all Chrome profiles
    #[arg(long)]
    list: bool,

    /// Query all passwords (requires confirmation)
    #[arg(long)]
    all: bool,

    /// Profile name (e.g., Default, Profile 1)
    #[arg(short, long)]
    profile: Option<String>,

    /// Filter by URL keyword
    #[arg(short, long)]
    keyword: Option<String>,

    /// Output in JSON format
    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<()> {
    // Initialize logging system
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();

    if cli.list {
        list_profiles();
    } else if cli.keyword.is_none() && !cli.all {
        // No keyword and not --all, show statistics
        show_password_statistics();
    } else {
        query_passwords(cli.profile, cli.keyword, cli.all, cli.json)?;
    }

    Ok(())
}

/// List all available Chrome profiles
fn list_profiles() {
    tracing::info!("Listing all Chrome profiles");
    let profiles = ChromePasswordUnlock::list_profiles();

    if profiles.is_empty() {
        println!("❌ No Chrome profiles found");
        println!("💡 Please ensure Chrome is installed and has been used at least once");
        tracing::warn!("No Chrome profiles found");
        return;
    }

    tracing::info!("Found {} Chrome profile(s)", profiles.len());
    println!("🔍 Found {} Chrome profile(s):\n", profiles.len());

    for profile in profiles {
        println!("  📁 {}", profile.name);
        println!("     Path: {}", profile.path.display());
        println!("     Database: {}", profile.login_data_path.display());
        println!();
    }
}

/// Show password statistics
fn show_password_statistics() {
    let unlocker = match ChromePasswordUnlock::new() {
        Ok(u) => u,
        Err(e) => {
            eprintln!("❌ Failed to initialize: {}", e);
            return;
        }
    };

    let profiles = ChromePasswordUnlock::list_profiles();
    let mut total_count = 0;

    for profile in &profiles {
        if let Ok(results) = unlocker.decrypt_from_profile(profile, None) {
            total_count += results.len();
        }
    }

    println!(
        "🔐 Found {} password(s) in {} profile(s)",
        total_count,
        profiles.len()
    );
    println!();
    println!("💡 Use --keyword <term> to filter passwords");
    println!("💡 Use --all to show all passwords");
    println!("💡 Use --profile <name> to query specific profile");
}

/// Query passwords
fn query_passwords(
    profile: Option<String>,
    keyword: Option<String>,
    _all: bool,
    json: bool,
) -> Result<()> {
    tracing::info!("Starting password decryption");
    let query = ChromePasswordUnlock::new()?;

    if let Some(profile_name) = profile {
        // Query specific profile
        tracing::info!("Decrypting specific profile: {}", profile_name);
        let profiles = ChromePasswordUnlock::list_profiles();
        let profile = profiles
            .into_iter()
            .find(|p| p.name == profile_name)
            .ok_or_else(|| anyhow::anyhow!("Profile not found: {}", profile_name))?;

        let results = query.decrypt_from_profile(&profile, keyword.as_deref())?;
        tracing::info!(
            "Successfully decrypted {} password(s) from profile '{}'",
            results.len(),
            profile_name
        );

        if json {
            let json_results: Vec<LoginInfoWithProfile> = results
                .into_iter()
                .map(|info| LoginInfoWithProfile {
                    profile: profile.name.clone(),
                    info,
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_results)?);
        } else {
            print_results(&results, Some(&profile.name));
        }
    } else {
        // Query all profiles
        tracing::info!("Decrypting all Chrome profiles");
        let all_results = query.decrypt_from_all_profiles(keyword.as_deref());

        if json {
            let all: Vec<LoginInfoWithProfile> = all_results
                .into_iter()
                .filter_map(|(name, result)| result.ok().map(|r| (name, r)))
                .flat_map(|(name, results)| {
                    results.into_iter().map(move |info| LoginInfoWithProfile {
                        profile: name.clone(),
                        info,
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&all)?);
        } else {
            let mut total_count = 0;
            for (profile_name, result) in all_results {
                match result {
                    Ok(results) => {
                        if !results.is_empty() {
                            print_results(&results, Some(&profile_name));
                            total_count += results.len();
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to decrypt profile '{}': {}", profile_name, e);
                    }
                }
            }

            if total_count == 0 {
                println!("❌ No passwords found");
            }
        }
        tracing::info!("Password decryption completed successfully");
    }

    Ok(())
}
