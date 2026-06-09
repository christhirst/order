use oracle::Connection;

/// Rows where usr.usr_division differs from ud_fb.ud_fb_division
/// (joined on login).
pub struct DivisionMismatch {
    pub login: String,
    pub usr_division: String,
    pub ud_fb_division: String,
}

/// Find all users whose division in `usr` doesn't match `ud_fb`.
pub fn find_division_mismatches(
    conn: &Connection,
) -> Result<Vec<DivisionMismatch>, Box<dyn std::error::Error>> {
    let rows = conn.query(
            "SELECT u.usr_login,
                    u.usr_division,
                    f.ud_fb_division
               FROM usr u
               JOIN ud_fb f ON u.usr_login = f.ud_fb_login
              WHERE u.usr_division <> f.ud_fb_division",
            &[],
        )?;

    let mut mismatches = Vec::new();
    for row_result in rows {
        let row = row_result?;
        let login: Option<String> = row.get(0)?;
        let usr_division: Option<String> = row.get(1)?;
        let ud_fb_division: Option<String> = row.get(2)?;

        mismatches.push(DivisionMismatch {
            login: login.unwrap_or_else(|| "NULL".to_string()),
            usr_division: usr_division.unwrap_or_else(|| "NULL".to_string()),
            ud_fb_division: ud_fb_division.unwrap_or_else(|| "NULL".to_string()),
        });
    }

    Ok(mismatches)
}

/// Set `usr_prov = 1` for all rows in the `usr` table.
/// Returns the number of rows updated.
pub fn set_usr_prov(
    conn: &Connection,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut stmt = conn.statement("UPDATE usr SET usr_prov = 1").build()?;
    stmt.execute(&[])?;
    conn.commit()?;

    // Note: rust-oracle Statement has a row_count method to get rows affected.
    Ok(stmt.row_count()? as u64)
}
