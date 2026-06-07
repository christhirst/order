use oracle_rs::Connection;

/// Rows where usr.usr_division differs from ud_fb.ud_fb_division
/// (joined on login).
pub struct DivisionMismatch {
    pub login: String,
    pub usr_division: String,
    pub ud_fb_division: String,
}

/// Find all users whose division in `usr` doesn't match `ud_fb`.
pub async fn find_division_mismatches(
    conn: &Connection,
) -> Result<Vec<DivisionMismatch>, Box<dyn std::error::Error>> {
    let result = conn
        .query(
            "SELECT u.usr_login,
                    u.usr_division,
                    f.ud_fb_division
               FROM usr u
               JOIN ud_fb f ON u.usr_login = f.ud_fb_login
              WHERE u.usr_division <> f.ud_fb_division",
            &[],
        )
        .await?;

    let mismatches = result
        .rows
        .iter()
        .map(|row| DivisionMismatch {
            login: row.get_string(0).unwrap_or("NULL").to_string(),
            usr_division: row.get_string(1).unwrap_or("NULL").to_string(),
            ud_fb_division: row.get_string(2).unwrap_or("NULL").to_string(),
        })
        .collect();

    Ok(mismatches)
}

/// Set `usr_prov = 1` for all rows in the `usr` table.
/// Returns the number of rows updated.
pub async fn set_usr_prov(
    conn: &Connection,
) -> Result<u64, Box<dyn std::error::Error>> {
    let result = conn
        .execute("UPDATE usr SET usr_prov = 1", &[])
        .await?;

    conn.commit().await?;

    Ok(result.rows_affected)
}
