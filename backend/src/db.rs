use rusqlite::{Connection, Result, params};
use bcrypt::{hash, DEFAULT_COST};

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
            pauze_start TEXT,
            pauze_totaal_ms INTEGER NOT NULL DEFAULT 0,
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

    // Migratie: pauze kolommen toevoegen aan bestaande databases
    let _ = conn.execute("ALTER TABLE klokslagen ADD COLUMN pauze_start TEXT", []);
    let _ = conn.execute("ALTER TABLE klokslagen ADD COLUMN pauze_totaal_ms INTEGER NOT NULL DEFAULT 0", []);

    // Seed standaard admin als er nog geen manager-account bestaat
    let manager_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM gebruikers WHERE rol = 'manager'",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    if manager_count == 0 {
        let wachtwoord_hash = hash("admin123", DEFAULT_COST)
            .expect("Wachtwoord hashen mislukt");
        conn.execute(
            "INSERT INTO medewerkers (naam, rol) VALUES ('Admin', 'manager')",
            [],
        )?;
        let medewerker_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO gebruikers (gebruikersnaam, wachtwoord_hash, rol, medewerker_id) VALUES ('admin', ?1, 'manager', ?2)",
            params![wachtwoord_hash, medewerker_id],
        )?;
    }

    Ok(())
}
