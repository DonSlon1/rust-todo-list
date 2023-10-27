use rusqlite::Connection;

pub fn connect() -> Connection {
    let conn = Connection::open("main.sqlite").unwrap();
    create_db(&conn);
    return conn
}

fn create_db(conn: &Connection) {
    conn.execute("
    CREATE TABLE IF NOT EXISTS todo
    (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT ,
        name VARCHAR NOT NULL,
        description TEXT,
        status TEXT DEFAULT 'NOT_STARTED' NOT NULL
    );", []).unwrap();
}
