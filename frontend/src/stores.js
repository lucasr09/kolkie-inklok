import { writable, derived } from 'svelte/store';

// Laad sessie uit localStorage als die er al is
const opgeslagenSessie = localStorage.getItem('kolkie_sessie');
const initiëleSessie = opgeslagenSessie ? JSON.parse(opgeslagenSessie) : null;

export const sessie = writable(initiëleSessie);

// Sla sessie op bij elke wijziging
sessie.subscribe(val => {
    if (val) {
        localStorage.setItem('kolkie_sessie', JSON.stringify(val));
        localStorage.setItem('kolkie_token', val.token);
    } else {
        localStorage.removeItem('kolkie_sessie');
        localStorage.removeItem('kolkie_token');
    }
});

export const isIngelogd = derived(sessie, $s => $s !== null);
export const isManager = derived(sessie, $s => $s?.rol === 'manager');
export const huidigeMedewerker = derived(sessie, $s => $s ? { id: $s.medewerker_id, naam: $s.naam } : null);
