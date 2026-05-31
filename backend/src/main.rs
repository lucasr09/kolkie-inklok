#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info { name: "CORS", kind: Kind::Response }
    }
    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS, PATCH"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
    }
}

#[options("/<_..>")]
fn options() -> &'static str { "" }

#[launch]
fn rocket() -> _ {
    db::init().expect("Database kon niet worden aangemaakt");
    rocket::build()
        .attach(CORS)
        .mount("/", routes::alle_routes())
        .mount("/", rocket::routes![options])
}
