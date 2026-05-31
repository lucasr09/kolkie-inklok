use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::{RoosterRegel, NieuweRoosterRegel};
use crate::routes::auth::check_auth;

#[get("/rooster?<token>")]
pub fn get_rooster(token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), None).is_err() {
        return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" })));
    }
    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, medewerker_id, datum, start_tijd, eind_tijd FROM rooster ORDER BY datum, start_tijd"
    ).unwrap();
    let regels: Vec<RoosterRegel> = stmt.query_map([], |row| {
        Ok(RoosterRegel { id: row.get(0)?, medewerker_id: row.get(1)?, datum: row.get(2)?, start_tijd: row.get(3)?, eind_tijd: row.get(4).ok() })
    }).unwrap().filter_map(|r| r.ok()).collect();
    (Status::Ok, Json(serde_json::json!(regels)))
}

#[post("/rooster?<token>", data = "<body>")]
pub fn post_rooster(token: Option<String>, body: Json<NieuweRoosterRegel>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Alleen managers kunnen het rooster aanpassen" })));
    }
    let conn = verbinding().unwrap();
    conn.execute(
        "INSERT INTO rooster (medewerker_id, datum, start_tijd, eind_tijd) VALUES (?1, ?2, ?3, ?4)",
        (&body.medewerker_id, &body.datum, &body.start_tijd, &body.eind_tijd),
    ).unwrap();
    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

#[delete("/rooster/<id>?<token>")]
pub fn delete_rooster(id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Alleen managers kunnen het rooster aanpassen" })));
    }
    let conn = verbinding().unwrap();
    conn.execute("DELETE FROM rooster WHERE id = ?1", [id]).unwrap();
    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}
