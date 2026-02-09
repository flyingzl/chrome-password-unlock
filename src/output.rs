use crate::models::LoginInfo;
use comfy_table::{Cell, Color, Table, presets::UTF8_FULL};
use tracing::{self, warn};

/// Format query results as table output
pub fn format_results_table(results: &[LoginInfo], profile_name: Option<&str>) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["URL", "Username", "Password"]);

    for item in results {
        table.add_row(vec![
            Cell::new(&item.url).fg(Color::Cyan),
            Cell::new(&item.username).fg(Color::Green),
            Cell::new(&item.password).fg(Color::Yellow),
        ]);
    }

    let mut output = String::new();

    if let Some(profile) = profile_name {
        output.push_str(&format!("\n🔐 Chrome Profile: {}\n", profile));
    }

    output.push_str(&table.to_string());

    // Add statistics
    output.push_str(&format!("\n\n📊 Total: {} record(s)\n", results.len()));

    output
}

/// Format query results as JSON output
pub fn format_results_json(results: &[LoginInfo]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".to_string())
}

/// Print query results
pub fn print_results(results: &[LoginInfo], profile_name: Option<&str>) {
    if results.is_empty() {
        warn!("No passwords found for profile: {:?}", profile_name);
        println!("❌ No passwords found");
        return;
    }

    println!("{}", format_results_table(results, profile_name));
}
