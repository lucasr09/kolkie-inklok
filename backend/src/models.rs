use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gebruiker {
    pub id: Option<i64>,
    pub gebruikersnaam: String,
    pub rol: String,
    pub medewerker_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrerenBody {
    pub gebruikersnaam: String,
    pub wachtwoord: String,
    pub naam: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InloggenBody {
    pub gebruikersnaam: String,
    pub wachtwoord: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WachtwoordBody {
    pub huidig: String,
    pub nieuw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Medewerker {
    pub id: Option<i64>,
    pub naam: String,
    pub rol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Klokslag {
    pub id: Option<i64>,
    pub medewerker_id: i64,
    pub ingeklokt_op: String,
    pub uitgeklokt_op: Option<String>,
    pub pauze_start: Option<String>,
    pub pauze_totaal_ms: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolUpdateBody {
    pub rol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InklokkenBody {
    pub medewerker_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PauzeBody {
    pub medewerker_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoosterRegel {
    pub id: Option<i64>,
    pub medewerker_id: i64,
    pub datum: String,
    pub start_tijd: String,
    pub eind_tijd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NieuweRoosterRegel {
    pub medewerker_id: i64,
    pub datum: String,
    pub start_tijd: String,
    pub eind_tijd: Option<String>,
}
