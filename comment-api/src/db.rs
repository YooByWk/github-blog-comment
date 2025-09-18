use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_db(pool: &DbPool) -> anyhow::Result<()> {
    let conn = pool.get()?;
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS comments (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            post        TEXT NOT NULL,
            parent      INTEGER,
            content     TEXT NOT NULL,
            writer      TEXT NOT NULL,
            password    TEXT NOT NULL,
            user_uuid   TEXT NOT NULL,
            ip          TEXT,
            created_at  TEXT NOT NULL,
            deleted     BOOLEAN DEFAULT 0
        )
        "#,
        [],
    )?;
    Ok(())
}
