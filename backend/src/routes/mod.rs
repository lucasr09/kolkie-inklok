pub mod auth;
pub mod klokken;
pub mod rooster;
pub mod medewerkers;
pub mod beschikbaarheid;

pub fn alle_routes() -> Vec<rocket::Route> {
    rocket::routes![
        auth::registreren,
        auth::inloggen,
        auth::uitloggen,
        auth::wachtwoord_wijzigen,
        medewerkers::get_medewerkers_auth,
        medewerkers::delete_medewerker,
        medewerkers::update_medewerker_rol,
        klokken::inklokken,
        klokken::uitklokken,
        klokken::pauze_starten,
        klokken::pauze_stoppen,
        klokken::nu_ingeklokt,
        klokken::get_klokslagen,
        klokken::get_alle_klokslagen,
        klokken::patch_klokslag,
        klokken::post_klokslag,
        klokken::delete_klokslag,
        rooster::get_rooster,
        rooster::post_rooster,
        rooster::delete_rooster,
        beschikbaarheid::get_beschikbaarheid,
        beschikbaarheid::post_beschikbaarheid,
        beschikbaarheid::delete_beschikbaarheid,
    ]
}
