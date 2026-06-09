use oracle::Connection;

use crate::settings::DatabaseSettings;

/// Builds an Oracle connection from the given database settings.
pub fn connect(
    db: &DatabaseSettings,
) -> Result<Connection, Box<dyn std::error::Error>> {
    let connect_string = format!("//{}:{}/{}", db.host, db.port, db.service);
    let conn = Connection::connect(&db.user, &db.password, &connect_string)?;

    Ok(conn)
}
