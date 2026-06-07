use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::{Klokslag, InklokkenBody, PauzeBody};
use crate::routes::auth::check_auth;
use chrono::Local;

#[post("/inklokken?<token>", data = "<body>")]
pub fn inklokken(token: Option<String>, body: Json<InklokkenBody>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" }))),
    };

    if gebruiker.medewerker_id != Some(body.medewerker_id) {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Je kan alleen jezelf inklokken" })));
    }

    let conn = verbinding().unwrap();
    let nu = Local::now().to_rfc3339();

    let al_ingeklokt: bool = conn.query_row(
        "SELECT COUNT(*) FROM klokslagen WHERE medewerker_id = ?1 AND uitgeklokt_op IS NULL",
        [body.medewerker_id], |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0;

    if al_ingeklokt {
        return (Status::Ok, Json(serde_json::json!({ "status": "fout", "bericht": "Al ingeklokt" })));
    }

    conn.execute(
        "INSERT INTO klokslagen (medewerker_id, ingeklokt_op, pauze_totaal_ms) VALUES (?1, ?2, 0)",
        (body.medewerker_id, &nu),
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok", "tijd": nu })))
}

#[post("/uitklokken?<token>", data = "<body>")]
pub fn uitklokken(token: Option<String>, body: Json<InklokkenBody>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" }))),
    };

    if gebruiker.medewerker_id != Some(body.medewerker_id) {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Je kan alleen jezelf uitklokken" })));
    }

    let conn = verbinding().unwrap();
    let nu = Local::now();
    let nu_str = nu.to_rfc3339();

    // Stop eventuele actieve pauze automatisch bij uitklokken
    let pauze_start: Option<String> = conn.query_row(
        "SELECT pauze_start FROM klokslagen WHERE medewerker_id = ?1 AND uitgeklokt_op IS NULL",
        [body.medewerker_id],
        |row| row.get(0),
    ).ok().flatten();

    if let Some(pauze_start_str) = pauze_start {
        if let Ok(pauze_start_dt) = chrono::DateTime::parse_from_rfc3339(&pauze_start_str) {
            let elapsed_ms = (nu.timestamp_millis() - pauze_start_dt.timestamp_millis()).max(0);
            conn.execute(
                "UPDATE klokslagen SET pauze_start = NULL, pauze_totaal_ms = pauze_totaal_ms + ?1 WHERE medewerker_id = ?2 AND uitgeklokt_op IS NULL",
                rusqlite::params![elapsed_ms, body.medewerker_id],
            ).unwrap();
        }
    }

    let rijen = conn.execute(
        "UPDATE klokslagen SET uitgeklokt_op = ?1 WHERE medewerker_id = ?2 AND uitgeklokt_op IS NULL",
        (&nu_str, body.medewerker_id),
    ).unwrap();

    if rijen == 0 {
        return (Status::Ok, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingeklokt" })));
    }

    (Status::Ok, Json(serde_json::json!({ "status": "ok", "tijd": nu_str })))
}

#[post("/pauze/starten?<token>", data = "<body>")]
pub fn pauze_starten(token: Option<String>, body: Json<PauzeBody>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout" }))),
    };

    if gebruiker.medewerker_id != Some(body.medewerker_id) && gebruiker.rol != "manager" {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    let nu = Local::now().to_rfc3339();

    let rijen = conn.execute(
        "UPDATE klokslagen SET pauze_start = ?1 WHERE medewerker_id = ?2 AND uitgeklokt_op IS NULL AND pauze_start IS NULL",
        (&nu, body.medewerker_id),
    ).unwrap();

    if rijen == 0 {
        return (Status::Ok, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingeklokt of al op pauze" })));
    }

    (Status::Ok, Json(serde_json::json!({ "status": "ok", "pauze_start": nu })))
}

#[post("/pauze/stoppen?<token>", data = "<body>")]
pub fn pauze_stoppen(token: Option<String>, body: Json<PauzeBody>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout" }))),
    };

    if gebruiker.medewerker_id != Some(body.medewerker_id) && gebruiker.rol != "manager" {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    let nu = Local::now();

    let pauze_start: Option<String> = conn.query_row(
        "SELECT pauze_start FROM klokslagen WHERE medewerker_id = ?1 AND uitgeklokt_op IS NULL",
        [body.medewerker_id],
        |row| row.get(0),
    ).ok().flatten();

    let Some(pauze_start_str) = pauze_start else {
        return (Status::Ok, Json(serde_json::json!({ "status": "fout", "bericht": "Niet op pauze" })));
    };

    let elapsed_ms = chrono::DateTime::parse_from_rfc3339(&pauze_start_str)
        .map(|dt| (nu.timestamp_millis() - dt.timestamp_millis()).max(0))
        .unwrap_or(0);

    conn.execute(
        "UPDATE klokslagen SET pauze_start = NULL, pauze_totaal_ms = pauze_totaal_ms + ?1 WHERE medewerker_id = ?2 AND uitgeklokt_op IS NULL",
        rusqlite::params![elapsed_ms, body.medewerker_id],
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

#[get("/nu-ingeklokt?<token>")]
pub fn nu_ingeklokt(token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), None).is_err() {
        return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT k.medewerker_id, m.naam, k.ingeklokt_op, k.pauze_start, k.pauze_totaal_ms
         FROM klokslagen k
         JOIN medewerkers m ON m.id = k.medewerker_id
         WHERE k.uitgeklokt_op IS NULL
         ORDER BY k.ingeklokt_op ASC"
    ).unwrap();

    let aanwezig: Vec<serde_json::Value> = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "medewerker_id": row.get::<_, i64>(0)?,
            "naam": row.get::<_, String>(1)?,
            "ingeklokt_op": row.get::<_, String>(2)?,
            "pauze_start": row.get::<_, Option<String>>(3)?,
            "pauze_totaal_ms": row.get::<_, i64>(4)?,
        }))
    }).unwrap().filter_map(|r| r.ok()).collect();

    (Status::Ok, Json(serde_json::json!(aanwezig)))
}

#[get("/klokslagen/<medewerker_id>?<token>")]
pub fn get_klokslagen(medewerker_id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout" }))),
    };

    if gebruiker.rol == "werknemer" && gebruiker.medewerker_id != Some(medewerker_id) {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, medewerker_id, ingeklokt_op, uitgeklokt_op, pauze_start, pauze_totaal_ms
         FROM klokslagen WHERE medewerker_id = ?1 ORDER BY ingeklokt_op DESC"
    ).unwrap();

    let klokslagen: Vec<Klokslag> = stmt.query_map([medewerker_id], |row| {
        Ok(Klokslag {
            id: row.get(0)?,
            medewerker_id: row.get(1)?,
            ingeklokt_op: row.get(2)?,
            uitgeklokt_op: row.get(3)?,
            pauze_start: row.get(4)?,
            pauze_totaal_ms: row.get(5)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect();

    (Status::Ok, Json(serde_json::json!(klokslagen)))
}

#[get("/klokslagen?<token>")]
pub fn get_alle_klokslagen(token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT k.id, k.medewerker_id, m.naam, k.ingeklokt_op, k.uitgeklokt_op, k.pauze_start, k.pauze_totaal_ms
         FROM klokslagen k JOIN medewerkers m ON m.id = k.medewerker_id
         ORDER BY k.ingeklokt_op DESC"
    ).unwrap();

    let klokslagen: Vec<serde_json::Value> = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i64>(0)?,
            "medewerker_id": row.get::<_, i64>(1)?,
            "naam": row.get::<_, String>(2)?,
            "ingeklokt_op": row.get::<_, String>(3)?,
            "uitgeklokt_op": row.get::<_, Option<String>>(4)?,
            "pauze_start": row.get::<_, Option<String>>(5)?,
            "pauze_totaal_ms": row.get::<_, i64>(6)?,
        }))
    }).unwrap().filter_map(|r| r.ok()).collect();

    (Status::Ok, Json(serde_json::json!(klokslagen)))
}

#[patch("/klokslagen/<id>?<token>", data = "<body>")]
pub fn patch_klokslag(id: i64, token: Option<String>, body: Json<serde_json::Value>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    if let Some(in_tijd) = body.get("ingeklokt_op").and_then(|v| v.as_str()) {
        conn.execute("UPDATE klokslagen SET ingeklokt_op = ?1 WHERE id = ?2", (in_tijd, id)).unwrap();
    }
    if let Some(uit_tijd) = body.get("uitgeklokt_op").and_then(|v| v.as_str()) {
        conn.execute("UPDATE klokslagen SET uitgeklokt_op = ?1 WHERE id = ?2", (uit_tijd, id)).unwrap();
    }

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

#[post("/klokslagen?<token>", data = "<body>")]
pub fn post_klokslag(token: Option<String>, body: Json<serde_json::Value>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }

    let conn = verbinding().unwrap();
    let medewerker_id = body.get("medewerker_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let ingeklokt_op = body.get("ingeklokt_op").and_then(|v| v.as_str()).unwrap_or("");
    let uitgeklokt_op = body.get("uitgeklokt_op").and_then(|v| v.as_str());

    conn.execute(
        "INSERT INTO klokslagen (medewerker_id, ingeklokt_op, uitgeklokt_op, pauze_totaal_ms) VALUES (?1, ?2, ?3, 0)",
        (medewerker_id, ingeklokt_op, uitgeklokt_op),
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

#[delete("/klokslagen/<id>?<token>")]
pub fn delete_klokslag(id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout" })));
    }
    let conn = verbinding().unwrap();
    conn.execute("DELETE FROM klokslagen WHERE id = ?1", [id]).unwrap();
    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}
