mod db;
mod queries;
mod settings;

use settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;

    if let Some(dir) = &settings.database.client_dir {
        let mut params = oracle::InitParams::new();
        let _ = params.oracle_client_lib_dir(dir);
        params.init()?;
    }

    println!(
        "Connecting to Oracle at {}:{}...",
        settings.database.host, settings.database.port
    );

    let conn = db::connect(&settings.database)?;

    // --- Division mismatch report ---
    let mismatches = queries::find_division_mismatches(&conn)?;

    if mismatches.is_empty() {
        println!("No division mismatches found.");
    } else {
        println!(
            "{:<20} {:<20} {:<20}",
            "LOGIN", "USR_DIVISION", "UD_FB_DIVISION"
        );
        println!("{}", "-".repeat(60));

        for m in &mismatches {
            println!("{:<20} {:<20} {:<20}", m.login, m.usr_division, m.ud_fb_division);
        }

        println!("\nTotal mismatches: {}", mismatches.len());
    }

    // --- Set usr_prov = 1 ---
    let updated = queries::set_usr_prov(&conn)?;
    println!("Updated usr_prov to 1 for {} rows.", updated);

    Ok(())
}
