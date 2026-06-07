<script>
  import { onMount } from 'svelte';
  import { getRooster, getMedewerkers, voegRoosterRegelToe, verwijderRoosterRegel } from '../lib/api.js';
  import { sessie, isManager } from '../stores.js';

  let rooster = [];
  let medewerkers = [];
  let nieuw = { medewerker_id: '', datum: '', start_tijd: '', eind_tijd: '' };
  let huidigeDatum = new Date();
  let laden = false;

  onMount(async () => {
    [rooster, medewerkers] = await Promise.all([getRooster(), getMedewerkers()]);
    if ($sessie?.medewerker_id) nieuw.medewerker_id = String($sessie.medewerker_id);
    nieuw.datum = datumNaarString(huidigeDatum);
  });

  function datumNaarString(d) { return d.toISOString().split('T')[0]; }

  function vorigedag() {
    const d = new Date(huidigeDatum);
    d.setDate(d.getDate() - 1);
    huidigeDatum = d;
    nieuw.datum = datumNaarString(d);
  }

  function volgendedag() {
    const d = new Date(huidigeDatum);
    d.setDate(d.getDate() + 1);
    huidigeDatum = d;
    nieuw.datum = datumNaarString(d);
  }

  function naarVandaag() {
    huidigeDatum = new Date();
    nieuw.datum = datumNaarString(huidigeDatum);
  }

  function naamVan(id) { return medewerkers.find(m => m.id === id)?.naam ?? '?'; }

  function avatarKleur(id) {
    const kleuren = ['#dc2626','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
    return kleuren[(id ?? 0) % kleuren.length];
  }

  function duurMinuten(r) {
    if (!r.eind_tijd) return null;
    const [sh, sm] = r.start_tijd.split(':').map(Number);
    const [eh, em] = r.eind_tijd.split(':').map(Number);
    return (eh * 60 + em) - (sh * 60 + sm);
  }

  function formatDuur(min) {
    if (min === null) return null;
    const h = Math.floor(min / 60);
    const m = min % 60;
    return m > 0 ? `${h}u ${m}m` : `${h}u`;
  }

  $: huidigeDatumStr = datumNaarString(huidigeDatum);
  $: dagShifts = rooster.filter(r => r.datum === huidigeDatumStr).sort((a, b) => a.start_tijd.localeCompare(b.start_tijd));
  $: isVandaag = huidigeDatumStr === datumNaarString(new Date());
  $: dagLabel = huidigeDatum.toLocaleDateString('nl-NL', { weekday: 'long', day: 'numeric', month: 'long' });

  async function voegToe() {
    if (!nieuw.medewerker_id || !nieuw.datum || !nieuw.start_tijd) return;
    laden = true;
    await voegRoosterRegelToe({ ...nieuw, medewerker_id: Number(nieuw.medewerker_id), eind_tijd: nieuw.eind_tijd || null });
    rooster = await getRooster();
    nieuw = { medewerker_id: $sessie?.medewerker_id ? String($sessie.medewerker_id) : '', datum: huidigeDatumStr, start_tijd: '', eind_tijd: '' };
    laden = false;
  }

  async function verwijder(id) {
    await verwijderRoosterRegel(id);
    rooster = await getRooster();
  }
</script>

<h1>Dagplanning</h1>

<!-- Dag navigator -->
<div class="dag-navigator kaart">
  <button class="nav-pijl" on:click={vorigedag}>
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><polyline points="15 18 9 12 15 6"/></svg>
  </button>
  <div class="dag-midden">
    <div class="dag-header-rij">
      {#if isVandaag}<span class="vandaag-chip">Vandaag</span>{/if}
      <span class="dag-naam-tekst" class:vandaag={isVandaag}>{dagLabel}</span>
    </div>
    {#if !isVandaag}
      <button class="terug-link" on:click={naarVandaag}>→ Naar vandaag</button>
    {/if}
  </div>
  <button class="nav-pijl" on:click={volgendedag}>
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><polyline points="9 18 15 12 9 6"/></svg>
  </button>
</div>

<!-- Shifts van de dag -->
{#if dagShifts.length === 0}
  <div class="leeg-staat">
    <div class="leeg-icoon">📭</div>
    <div class="leeg-titel">Niemand ingepland</div>
    <div class="leeg-sub">Er zijn geen shifts op deze dag{$isManager ? ' — voeg er een toe hieronder' : ''}.</div>
  </div>
{:else}
  <div class="shift-lijst">
    {#each dagShifts as r}
      <div class="shift-kaart kaart">
        <div class="shift-avatar" style="background: {avatarKleur(r.medewerker_id)}">{naamVan(r.medewerker_id)[0]}</div>
        <div class="shift-info">
          <span class="shift-naam">{naamVan(r.medewerker_id)}</span>
          <div class="shift-tijdblok">
            <span class="shift-tijd">{r.start_tijd}</span>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
              <line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>
            </svg>
            <span class="shift-tijd">{r.eind_tijd ?? '?'}</span>
          </div>
        </div>
        {#if duurMinuten(r) !== null}
          <div class="duur-chip">{formatDuur(duurMinuten(r))}</div>
        {/if}
        {#if $isManager}
          <button class="verwijder-knop" on:click={() => verwijder(r.id)} title="Verwijder shift">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
              <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<!-- Shift toevoegen -->
{#if $isManager}
  <div class="toevoeg-kaart">
    <div class="toevoeg-header">
      <div class="toevoeg-icoon">+</div>
      <h2>Shift toevoegen</h2>
    </div>
    <div class="form">
      <div class="form-rij">
        <label class="form-veld">
          <span class="veld-label">Medewerker</span>
          <select bind:value={nieuw.medewerker_id}>
            <option value="">Kies medewerker...</option>
            {#each medewerkers as m}<option value={m.id}>{m.naam}</option>{/each}
          </select>
        </label>
        <label class="form-veld">
          <span class="veld-label">Datum</span>
          <input type="date" bind:value={nieuw.datum} on:change={(e) => huidigeDatum = new Date(e.target.value + 'T00:00:00')} />
        </label>
      </div>
      <div class="form-rij">
        <label class="form-veld">
          <span class="veld-label">Starttijd</span>
          <input type="time" bind:value={nieuw.start_tijd} />
        </label>
        <label class="form-veld">
          <span class="veld-label">Eindtijd <span class="optioneel">(optioneel)</span></span>
          <input type="time" bind:value={nieuw.eind_tijd} />
        </label>
      </div>
      <button class="knop-toevoegen" on:click={voegToe} disabled={laden || !nieuw.medewerker_id || !nieuw.start_tijd}>
        {laden ? 'Bezig...' : '+ Shift toevoegen'}
      </button>
    </div>
  </div>
{/if}

<style>
  .dag-navigator {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.25rem;
    margin-bottom: 1.5rem;
  }

  .nav-pijl {
    background: var(--rand-licht);
    border: 1.5px solid var(--rand);
    border-radius: 10px;
    width: 40px;
    height: 40px;
    cursor: pointer;
    color: var(--tekst-zacht);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition);
    flex-shrink: 0;
  }

  .nav-pijl:hover { border-color: var(--rood); color: var(--rood); background: var(--rood-licht); }

  .dag-midden {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }

  .dag-header-rij { display: flex; align-items: center; gap: 0.6rem; }

  .vandaag-chip {
    background: var(--rood);
    color: white;
    font-family: var(--font-body);
    font-size: 0.65rem;
    font-weight: 800;
    padding: 0.15rem 0.55rem;
    border-radius: 20px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .dag-naam-tekst {
    font-family: var(--font-display);
    font-size: 1.35rem;
    letter-spacing: 0.04em;
    color: var(--donker);
    text-transform: capitalize;
  }

  .dag-naam-tekst.vandaag { color: var(--rood); }

  .terug-link {
    background: none;
    border: none;
    color: var(--tekst-zacht);
    font-size: 0.78rem;
    font-family: var(--font-body);
    cursor: pointer;
    transition: color var(--transition);
    padding: 0;
    font-weight: 600;
  }

  .terug-link:hover { color: var(--rood); }

  /* Leeg */
  .leeg-staat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 3.5rem 2rem;
    text-align: center;
    background: var(--wit);
    border-radius: var(--radius-lg);
    border: 1.5px dashed var(--rand);
    margin-bottom: 2rem;
  }

  .leeg-icoon { font-size: 2.5rem; }
  .leeg-titel { font-weight: 800; font-size: 1rem; color: var(--donker); }
  .leeg-sub { font-size: 0.85rem; color: var(--tekst-zacht); max-width: 260px; }

  /* Shifts */
  .shift-lijst {
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
    margin-bottom: 2rem;
  }

  .shift-kaart {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.9rem 1.25rem;
  }

  .shift-kaart:hover { transform: translateY(-1px); }

  .shift-avatar {
    width: 44px;
    height: 44px;
    border-radius: 13px;
    color: white;
    font-size: 1.2rem;
    font-weight: 900;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
    box-shadow: 0 3px 10px rgba(0,0,0,0.12);
  }

  .shift-info { display: flex; flex-direction: column; gap: 0.25rem; flex: 1; }
  .shift-naam { font-weight: 800; font-size: 0.98rem; color: var(--donker); }

  .shift-tijdblok {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    color: var(--tekst-zacht);
  }

  .shift-tijd {
    font-size: 0.88rem;
    font-weight: 700;
    color: var(--tekst-zacht);
    font-family: var(--font-display);
    letter-spacing: 0.04em;
    font-size: 1rem;
  }

  .duur-chip {
    background: var(--goud-licht);
    color: var(--goud);
    border: 1.5px solid #fde68a;
    padding: 0.25rem 0.7rem;
    border-radius: 20px;
    font-size: 0.78rem;
    font-weight: 800;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .verwijder-knop {
    background: none;
    border: none;
    cursor: pointer;
    color: #d1d5db;
    padding: 0.4rem;
    border-radius: 8px;
    display: flex;
    align-items: center;
    transition: all var(--transition);
    flex-shrink: 0;
  }

  .verwijder-knop:hover { color: var(--rood); background: var(--rood-licht); }

  /* Toevoegen */
  .toevoeg-kaart {
    background: var(--wit);
    border-radius: var(--radius-lg);
    border: 1px solid var(--rand);
    padding: 1.5rem;
    box-shadow: var(--schaduw);
  }

  .toevoeg-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  .toevoeg-icoon {
    width: 34px;
    height: 34px;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.3rem;
    font-weight: 900;
    flex-shrink: 0;
    box-shadow: 0 2px 8px var(--rood-glow);
  }

  .toevoeg-header h2 { margin: 0; }

  .form { display: flex; flex-direction: column; gap: 0.85rem; }
  .form-rij { display: flex; gap: 0.75rem; flex-wrap: wrap; }
  .form-veld { display: flex; flex-direction: column; gap: 0.35rem; flex: 1; min-width: 150px; }
  .veld-label { font-size: 0.75rem; font-weight: 800; color: var(--tekst-zacht); text-transform: uppercase; letter-spacing: 0.06em; }
  .optioneel { font-weight: 500; text-transform: none; letter-spacing: 0; color: var(--tekst-extra); }

  .knop-toevoegen {
    align-self: flex-start;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white;
    border: none;
    border-radius: var(--radius);
    padding: 0.75rem 1.75rem;
    font-family: var(--font-body);
    font-weight: 800;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all var(--transition);
    box-shadow: 0 2px 8px var(--rood-glow);
  }

  .knop-toevoegen:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 4px 14px var(--rood-glow);
  }

  .knop-toevoegen:disabled { opacity: 0.5; cursor: not-allowed; }

  @media (max-width: 480px) {
    .dag-naam-tekst { font-size: 1rem; }
    .form-rij { flex-direction: column; }
    .knop-toevoegen { width: 100%; }
  }
</style>
