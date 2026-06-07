use rocket::serde::json::Json;
use rocket::http::Status;
use crate::db::verbinding;
use crate::models::{Beschikbaarheid, NieuweBeschikbaarheid};
use crate::routes::auth::check_auth;

#[get("/beschikbaarheid?<token>")]
pub fn get_beschikbaarheid(token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), None).is_err() {
        return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" })));
    }
    let conn = verbinding().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, medewerker_id, datum, status, hele_dag, van_tijd, tot_tijd, gewenste_functie FROM beschikbaarheid ORDER BY datum"
    ).unwrap();
    let items: Vec<Beschikbaarheid> = stmt.query_map([], |row| {
        Ok(Beschikbaarheid {
            id: row.get(0)?,
            medewerker_id: row.get(1)?,
            datum: row.get(2)?,
            status: row.get::<_, Option<String>>(3)?.unwrap_or_else(|| "beschikbaar".to_string()),
            hele_dag: row.get::<_, i64>(4)? != 0,
            van_tijd: row.get(5).ok().flatten(),
            tot_tijd: row.get(6).ok().flatten(),
            gewenste_functie: row.get(7).ok().flatten(),
        })
    }).unwrap().filter_map(|r| r.ok()).collect();
    (Status::Ok, Json(serde_json::json!(items)))
}

#[post("/beschikbaarheid?<token>", data = "<body>")]
pub fn post_beschikbaarheid(token: Option<String>, body: Json<NieuweBeschikbaarheid>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), None).is_err() {
        return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" })));
    }
    let conn = verbinding().unwrap();
    conn.execute(
        "DELETE FROM beschikbaarheid WHERE medewerker_id = ?1 AND datum = ?2",
        (&body.medewerker_id, &body.datum),
    ).unwrap();
    conn.execute(
        "INSERT INTO beschikbaarheid (medewerker_id, datum, status, hele_dag, van_tijd, tot_tijd, gewenste_functie) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&body.medewerker_id, &body.datum, &body.status, &(body.hele_dag as i64), &body.van_tijd, &body.tot_tijd, &body.gewenste_functie),
    ).unwrap();
    let id = conn.last_insert_rowid();
    (Status::Ok, Json(serde_json::json!({ "status": "ok", "id": id })))
}

#[delete("/beschikbaarheid/<id>?<token>")]
pub fn delete_beschikbaarheid(id: i64, token: Option<String>) -> (Status, Json<serde_json::Value>) {
    if check_auth(token.as_deref(), None).is_err() {
        return (Status::Unauthorized, Json(serde_json::json!({ "status": "fout", "bericht": "Niet ingelogd" })));
    }
    let conn = verbinding().unwrap();
    conn.execute("DELETE FROM beschikbaarheid WHERE id = ?1", [id]).unwrap();
    (Status::Ok, Json(serde_json::json!({ "status": "ok" })))
}
