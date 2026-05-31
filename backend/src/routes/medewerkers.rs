use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::Medewerker;
use crate::routes::auth::check_auth;

#[get("/medewerkers?<token>")]
pub fn get_medewerkers_auth(token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), None).is_err() {
        return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" })));
    }
    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare("SELECT id, naam, rol FROM medewerkers").unwrap();
    let medewerkers: Vec<Medewerker> = stmt.query_map([], |row| {
        Ok(Medewerker { id: row.get(0)?, naam: row.get(1)?, rol: row.get(2)? })
    }).unwrap().filter_map(|r| r.ok()).collect();
    (Status::Ok, Json(serde_json::json!(medewerkers)))
}

#[delete("/medewerkers/<id>?<token>")]
pub fn delete_medewerker(id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Geen toegang" })));
    }
    let conn = verbinding().unwrap();
    conn.execute("DELETE FROM medewerkers WHERE id = ?1", [id]).unwrap();
    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}
