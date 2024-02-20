use rusqlite::{Connection, Result};

pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub role: String,
    pub date_created: String,
    pub last_connection: String,
}

pub fn add_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, hashed_password, role, date_created, last_connection) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&user.username, &user.hashed_password, &user.role, &user.date_created, &user.last_connection]
    )?;
    Ok(())
}
