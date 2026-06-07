use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::{Medewerker, RolUpdateBody};
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

    // Cascade: verwijder sessies, gebruikersaccount, klokslagen en rooster
    conn.execute(
        "DELETE FROM sessies WHERE gebruiker_id IN (SELECT id FROM gebruikers WHERE medewerker_id = ?1)",
        [id],
    ).unwrap();
    conn.execute("DELETE FROM gebruikers WHERE medewerker_id = ?1", [id]).unwrap();
    conn.execute("DELETE FROM klokslagen WHERE medewerker_id = ?1", [id]).unwrap();
    conn.execute("DELETE FROM rooster WHERE medewerker_id = ?1", [id]).unwrap();
    conn.execute("DELETE FROM medewerkers WHERE id = ?1", [id]).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}

#[patch("/medewerkers/<id>/rol?<token>", data = "<body>")]
pub fn update_medewerker_rol(id: i64, token: Option<String>, body: Json<RolUpdateBody>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), Some("manager")).is_err() {
        return (Status::Forbidden, Json(serde_json::json!({ "status": "fout", "bericht": "Geen toegang" })));
    }

    if body.rol != "werknemer" && body.rol != "manager" {
        return (Status::BadRequest, Json(serde_json::json!({ "status": "fout", "bericht": "Ongeldige rol" })));
    }

    let conn = verbinding().unwrap();

    // Voorkom dat de laatste manager wordt gedegradeerd
    if body.rol == "werknemer" {
        let is_manager: bool = conn.query_row(
            "SELECT COUNT(*) FROM gebruikers WHERE medewerker_id = ?1 AND rol = 'manager'",
            [id],
            |row| row.get::<_, i64>(0),
        ).unwrap_or(0) > 0;

        if is_manager {
            let manager_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM gebruikers WHERE rol = 'manager'",
                [],
                |row| row.get(0),
            ).unwrap_or(0);

            if manager_count <= 1 {
                return (Status::BadRequest, Json(serde_json::json!({
                    "status": "fout",
                    "bericht": "Kan de laatste manager niet demoveren. Promoveer eerst een andere medewerker."
                })));
            }
        }
    }

    conn.execute("UPDATE medewerkers SET rol = ?1 WHERE id = ?2", rusqlite::params![body.rol, id]).unwrap();
    conn.execute("UPDATE gebruikers SET rol = ?1 WHERE medewerker_id = ?2", rusqlite::params![body.rol, id]).unwrap();

    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}
