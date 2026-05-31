use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::{Klokslag, InklokkenBody};
use crate::routes::auth::check_auth;
use chrono::Local;

// Werknemer klokt zichzelf in
#[post("/inklokken?<token>", data = "<body>")]
pub fn inklokken(token: Option<String>, body: Json<InklokkenBody>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" }))),
    };

    // Alleen jezelf inklokkken — managers mogen dit niet voor anderen
    if gebruiker.medewerker_id != Some(body.medewerker_id) {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Je kan alleen jezelf inklokkken" })));
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
        "INSERT INTO klokslagen (medewerker_id, ingeklokt_op) VALUES (?1, ?2)",
        (body.medewerker_id, &nu),
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok", "tijd": nu })))
}

// Werknemer klokt zichzelf uit
#[post("/uitklokken?<token>", data = "<body>")]
pub fn uitklokken(token: Option<String>, body: Json<InklokkenBody>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" }))),
    };

    // Alleen jezelf uitklokken — managers mogen dit niet voor anderen
    if gebruiker.medewerker_id != Some(body.medewerker_id) {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Je kan alleen jezelf uitklokken" })));
    }

    let conn = verbinding().unwrap();
    let nu = Local::now().to_rfc3339();

    let rijen = conn.execute(
        "UPDATE klokslagen SET uitgeklokt_op = ?1 WHERE medewerker_id = ?2 AND uitgeklokt_op IS NULL",
        (&nu, body.medewerker_id),
    ).unwrap();

    if rijen == 0 {
        return (Status::Ok, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingeklokt" })));
    }

    (Status::Ok, Json(serde_json::json!({ "status": "ok", "tijd": nu })))
}

// Eigen klokslagen ophalen
#[get("/klokslagen/<medewerker_id>?<token>")]
pub fn get_klokslagen(medewerker_id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    let gebruiker = match check_auth(token.as_deref(), None) {
        Ok(g) => g,
        Err(_) => return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" }))),
    };

    if gebruiker.rol == "werknemer" && gebruiker.medewerker_id != Some(medewerker_id) {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Geen toegang" })));
    }

    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, medewerker_id, ingeklokt_op, uitgeklokt_op FROM klokslagen WHERE medewerker_id = ?1 ORDER BY ingeklokt_op DESC"
    ).unwrap();

    let klokslagen: Vec<Klokslag> = stmt.query_map([medewerker_id], |row| {
        Ok(Klokslag { id: row.get(0)?, medewerker_id: row.get(1)?, ingeklokt_op: row.get(2)?, uitgeklokt_op: row.get(3)? })
    }).unwrap().filter_map(|r| r.ok()).collect();

    (Status::Ok, Json(serde_json::json!(klokslagen)))
}

// Alle klokslagen ophalen (manager only)
#[get("/klokslagen?<token>")]
pub fn get_alle_klokslagen(token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Alleen voor managers" })));
    }

    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT k.id, k.medewerker_id, m.naam, k.ingeklokt_op, k.uitgeklokt_op
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
        }))
    }).unwrap().filter_map(|r| r.ok()).collect();

    (Status::Ok, Json(serde_json::json!(klokslagen)))
}

// Manager past klokslag aan
#[patch("/klokslagen/<id>?<token>", data = "<body>")]
pub fn patch_klokslag(id: i64, token: Option<String>, body: Json<serde_json::Value>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Alleen voor managers" })));
    }

    let conn = verbinding().unwrap();
    let ingeklokt_op = body.get("ingeklokt_op").and_then(|v| v.as_str());
    let uitgeklokt_op = body.get("uitgeklokt_op").and_then(|v| v.as_str());

    if let Some(in_tijd) = ingeklokt_op {
        conn.execute("UPDATE klokslagen SET ingeklokt_op = ?1 WHERE id = ?2", (in_tijd, id)).unwrap();
    }
    if let Some(uit_tijd) = uitgeklokt_op {
        conn.execute("UPDATE klokslagen SET uitgeklokt_op = ?1 WHERE id = ?2", (uit_tijd, id)).unwrap();
    }

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

// Manager voegt klokslag toe voor iemand anders
#[post("/klokslagen?<token>", data = "<body>")]
pub fn post_klokslag(token: Option<String>, body: Json<serde_json::Value>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Alleen voor managers" })));
    }

    let conn = verbinding().unwrap();
    let medewerker_id = body.get("medewerker_id").and_then(|v| v.as_i64()).unwrap_or(0);
    let ingeklokt_op = body.get("ingeklokt_op").and_then(|v| v.as_str()).unwrap_or("");
    let uitgeklokt_op = body.get("uitgeklokt_op").and_then(|v| v.as_str());

    conn.execute(
        "INSERT INTO klokslagen (medewerker_id, ingeklokt_op, uitgeklokt_op) VALUES (?1, ?2, ?3)",
        (medewerker_id, ingeklokt_op, uitgeklokt_op),
    ).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

// Manager verwijdert klokslag
#[delete("/klokslagen/<id>?<token>")]
pub fn delete_klokslag(id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Alleen voor managers" })));
    }
    let conn = verbinding().unwrap();
    conn.execute("DELETE FROM klokslagen WHERE id = ?1", [id]).unwrap();
    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}
