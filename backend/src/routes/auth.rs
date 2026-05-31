use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::{RegistrerenBody, InloggenBody, Gebruiker};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::Local;

fn haal_gebruiker_op_token(token: &str) -> Option<Gebruiker> {
    let conn = verbinding().ok()?;
    conn.query_row(
        "SELECT g.id, g.gebruikersnaam, g.rol, g.medewerker_id
         FROM sessies s
         JOIN gebruikers g ON g.id = s.gebruiker_id
         WHERE s.token = ?1",
        [token],
        |row| Ok(Gebruiker {
            id: row.get(0)?,
            gebruikersnaam: row.get(1)?,
            rol: row.get(2)?,
            medewerker_id: row.get(3)?,
        }),
    ).ok()
}

pub fn check_auth(token: Option<&str>, vereiste_rol: Option<&str>) -> Result<Gebruiker, Status> {
    let token = token.ok_or(Status::Unauthorized)?;
    let gebruiker = haal_gebruiker_op_token(token).ok_or(Status::Unauthorized)?;
    if let Some(rol) = vereiste_rol {
        if gebruiker.rol != rol {
            return Err(Status::Forbidden);
        }
    }
    Ok(gebruiker)
}

#[post("/registreren", data = "<body>")]
pub fn registreren(body: Json<RegistrerenBody>) -> (Status, Json<serde_json::Value>) {
    // Valideer rol
    if body.rol != "werknemer" && body.rol != "manager" {
        return (Status::BadRequest, Json(serde_json::json!({
            "status": "fout", "bericht": "Rol moet 'werknemer' of 'manager' zijn"
        })));
    }

    let conn = match verbinding() {
        Ok(c) => c,
        Err(_) => return (Status::InternalServerError, Json(serde_json::json!({ "status": "fout" }))),
    };

    // Check of gebruikersnaam al bestaat
    let bestaat: bool = conn.query_row(
        "SELECT COUNT(*) FROM gebruikers WHERE gebruikersnaam = ?1",
        [&body.gebruikersnaam],
        |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0;

    if bestaat {
        return (Status::Conflict, Json(serde_json::json!({
            "status": "fout", "bericht": "Gebruikersnaam al in gebruik"
        })));
    }

    // Hash wachtwoord
    let hash = match hash(&body.wachtwoord, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (Status::InternalServerError, Json(serde_json::json!({ "status": "fout" }))),
    };

    // Maak medewerker aan
    conn.execute(
        "INSERT INTO medewerkers (naam, rol) VALUES (?1, ?2)",
        (&body.naam, &body.rol),
    ).unwrap();
    let medewerker_id = conn.last_insert_rowid();

    // Maak gebruiker aan
    conn.execute(
        "INSERT INTO gebruikers (gebruikersnaam, wachtwoord_hash, rol, medewerker_id) VALUES (?1, ?2, ?3, ?4)",
        (&body.gebruikersnaam, &hash, &body.rol, medewerker_id),
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

#[post("/inloggen", data = "<body>")]
pub fn inloggen(body: Json<InloggenBody>) -> (Status, Json<serde_json::Value>) {
    let conn = match verbinding() {
        Ok(c) => c,
        Err(_) => return (Status::InternalServerError, Json(serde_json::json!({ "status": "fout" }))),
    };

    let result = conn.query_row(
        "SELECT id, wachtwoord_hash, rol, medewerker_id FROM gebruikers WHERE gebruikersnaam = ?1",
        [&body.gebruikersnaam],
        |row| Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<i64>>(3)?,
        )),
    );

    let (gebruiker_id, hash, rol, medewerker_id) = match result {
        Ok(r) => r,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({
            "status": "fout", "bericht": "Onjuiste gebruikersnaam of wachtwoord"
        }))),
    };

    if !verify(&body.wachtwoord, &hash).unwrap_or(false) {
        return (Status::Unauthorized, Json(serde_json::json!({
            "status": "fout", "bericht": "Onjuiste gebruikersnaam of wachtwoord"
        })));
    }

    let token = Uuid::new_v4().to_string();
    let nu = Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO sessies (token, gebruiker_id, aangemaakt_op) VALUES (?1, ?2, ?3)",
        (&token, gebruiker_id, &nu),
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({
        "status": "ok",
        "token": token,
        "rol": rol,
        "gebruikersnaam": body.gebruikersnaam,
        "medewerker_id": medewerker_id,
    })))
}

#[post("/uitloggen")]
pub fn uitloggen() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}
