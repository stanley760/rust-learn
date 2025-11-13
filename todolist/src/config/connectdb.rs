use rusqlite::Connection;
use rusqlite::Result;
pub fn connect_db() -> Result<Connection> {
    let conn = Connection::open("./database.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
        id BIGINT PRIMARY KEY,
        description TEXT NOT NULL,
        completed BOOLEAN NOT NULL
        )",
        (),
    )?;
    Ok(conn)
}
