pub mod auth;
pub mod klokken;
pub mod rooster;
pub mod medewerkers;

pub fn alle_routes() -> Vec<rocket::Route> {
    rocket::routes![
        auth::registreren,
        auth::inloggen,
        auth::uitloggen,
        medewerkers::get_medewerkers_auth,
        medewerkers::delete_medewerker,
        medewerkers::update_medewerker_rol,
        klokken::inklokken,
        klokken::uitklokken,
        klokken::get_klokslagen,
        klokken::get_alle_klokslagen,
        klokken::patch_klokslag,
        klokken::post_klokslag,
        klokken::delete_klokslag,
        rooster::get_rooster,
        rooster::post_rooster,
        rooster::delete_rooster,
    ]
}
