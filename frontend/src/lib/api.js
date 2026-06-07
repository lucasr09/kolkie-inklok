const BASE = '/api';

function token() {
    return localStorage.getItem('kolkie_token');
}

function authUrl(pad) {
    const t = token();
    return t ? `${BASE}${pad}?token=${t}` : `${BASE}${pad}`;
}

// Auth
export async function registreren(gebruikersnaam, wachtwoord, naam) {
    const res = await fetch(`${BASE}/registreren`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ gebruikersnaam, wachtwoord, naam }),
    });
    return res.json();
}

export async function inloggen(gebruikersnaam, wachtwoord) {
    const res = await fetch(`${BASE}/inloggen`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ gebruikersnaam, wachtwoord }),
    });
    return res.json();
}

export async function uitloggen() {
    const t = token();
    if (t) {
        await fetch(`${BASE}/uitloggen?token=${t}`, { method: 'POST' });
    }
}

export async function wijzigWachtwoord(huidig, nieuw) {
    const res = await fetch(authUrl('/wachtwoord'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ huidig, nieuw }),
    });
    return res.json();
}

// Medewerkers
export async function getMedewerkers() {
    const res = await fetch(authUrl('/medewerkers'));
    return res.json();
}

export async function verwijderMedewerker(id) {
    const res = await fetch(authUrl(`/medewerkers/${id}`), { method: 'DELETE' });
    return res.json();
}

export async function updateMedewerkerRol(id, rol) {
    const res = await fetch(authUrl(`/medewerkers/${id}/rol`), {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ rol }),
    });
    return res.json();
}

// Klokken
export async function inklokken(medewerker_id) {
    const res = await fetch(authUrl('/inklokken'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ medewerker_id }),
    });
    return res.json();
}

export async function uitklokken(medewerker_id) {
    const res = await fetch(authUrl('/uitklokken'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ medewerker_id }),
    });
    return res.json();
}

export async function startPauze(medewerker_id) {
    const res = await fetch(authUrl('/pauze/starten'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ medewerker_id }),
    });
    return res.json();
}

export async function stopPauze(medewerker_id) {
    const res = await fetch(authUrl('/pauze/stoppen'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ medewerker_id }),
    });
    return res.json();
}

export async function getNuIngeklokt() {
    const res = await fetch(authUrl('/nu-ingeklokt'));
    return res.json();
}

export async function getKlokslagen(medewerker_id) {
    const res = await fetch(authUrl(`/klokslagen/${medewerker_id}`));
    return res.json();
}

export async function getAlleKlokslagen() {
    const res = await fetch(authUrl('/klokslagen'));
    return res.json();
}

export async function patchKlokslag(id, data) {
    const res = await fetch(authUrl(`/klokslagen/${id}`), {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function postKlokslag(data) {
    const res = await fetch(authUrl('/klokslagen'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function deleteKlokslag(id) {
    const res = await fetch(authUrl(`/klokslagen/${id}`), { method: 'DELETE' });
    return res.json();
}

// Rooster
export async function getRooster() {
    const res = await fetch(authUrl('/rooster'));
    return res.json();
}

export async function voegRoosterRegelToe(regel) {
    const res = await fetch(authUrl('/rooster'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(regel),
    });
    return res.json();
}

export async function verwijderRoosterRegel(id) {
    const res = await fetch(authUrl(`/rooster/${id}`), { method: 'DELETE' });
    return res.json();
}

// Beschikbaarheid
export async function getBeschikbaarheid() {
    const res = await fetch(authUrl('/beschikbaarheid'));
    return res.json();
}

export async function setBeschikbaarheid(data) {
    const res = await fetch(authUrl('/beschikbaarheid'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function verwijderBeschikbaarheid(id) {
    const res = await fetch(authUrl(`/beschikbaarheid/${id}`), { method: 'DELETE' });
    return res.json();
}
