use rusqlite::{Connection, Result};

pub fn verbinding() -> Result<Connection> {
    Connection::open("kolkie.db")
}

pub fn init() -> Result<()> {
    let conn = verbinding()?;
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS gebruikers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            gebruikersnaam TEXT NOT NULL UNIQUE,
            wachtwoord_hash TEXT NOT NULL,
            rol TEXT NOT NULL CHECK(rol IN ('werknemer', 'manager')),
            medewerker_id INTEGER,
            FOREIGN KEY (medewerker_id) REFERENCES medewerkers(id)
        );

        CREATE TABLE IF NOT EXISTS sessies (
            token TEXT PRIMARY KEY,
            gebruiker_id INTEGER NOT NULL,
            aangemaakt_op TEXT NOT NULL,
            FOREIGN KEY (gebruiker_id) REFERENCES gebruikers(id)
        );

        CREATE TABLE IF NOT EXISTS medewerkers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            naam TEXT NOT NULL,
            rol TEXT NOT NULL DEFAULT 'werknemer' CHECK(rol IN ('werknemer', 'manager'))
        );

        CREATE TABLE IF NOT EXISTS klokslagen (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            medewerker_id INTEGER NOT NULL,
            ingeklokt_op TEXT NOT NULL,
            uitgeklokt_op TEXT,
            FOREIGN KEY (medewerker_id) REFERENCES medewerkers(id)
        );

        CREATE TABLE IF NOT EXISTS rooster (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            medewerker_id INTEGER NOT NULL,
            datum TEXT NOT NULL,
            start_tijd TEXT NOT NULL,
            eind_tijd TEXT,
            FOREIGN KEY (medewerker_id) REFERENCES medewerkers(id)
        );
    ")?;
    Ok(())
}
