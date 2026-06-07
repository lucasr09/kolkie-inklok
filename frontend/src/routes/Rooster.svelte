<script>
  import { onMount } from 'svelte';
  import { getRooster, getMedewerkers, voegRoosterRegelToe, verwijderRoosterRegel, getBeschikbaarheid } from '../lib/api.js';
  import { sessie, isManager } from '../stores.js';

  const FUNCTIES = ['Kassa/Bediening', 'Friet', 'Snacks', 'Bakplaat', 'Afwas'];
  const FUNCTIE_KLEUREN = {
    'Kassa/Bediening': { bg: '#eff6ff', kleur: '#2563eb', rand: '#bfdbfe' },
    'Friet':           { bg: '#fffbeb', kleur: '#d97706', rand: '#fde68a' },
    'Snacks':          { bg: '#fdf4ff', kleur: '#9333ea', rand: '#e9d5ff' },
    'Bakplaat':        { bg: '#fff7ed', kleur: '#ea580c', rand: '#fed7aa' },
    'Afwas':           { bg: '#f0fdf4', kleur: '#16a34a', rand: '#86efac' },
  };

  let rooster = [];
  let medewerkers = [];
  let beschikbaarheid = [];
  let nieuw = { medewerker_id: '', datum: '', start_tijd: '', eind_tijd: '', functie: '' };
  let huidigeDatum = new Date();
  let laden = false;
  let formRef;

  onMount(async () => {
    [rooster, medewerkers, beschikbaarheid] = await Promise.all([getRooster(), getMedewerkers(), getBeschikbaarheid()]);
    nieuw.datum = datumNaarString(huidigeDatum);
  });

  function datumNaarString(d) {
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  }

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

  // Beschikbaarheid per medewerker voor deze dag
  $: dagBeschikbaar = medewerkers.map(m => {
    const entry = beschikbaarheid.find(b => b.medewerker_id === m.id && b.datum === huidigeDatumStr) ?? null;
    return { med: m, entry };
  });

  $: heeftBeschikbaarEntries = dagBeschikbaar.some(d => d.entry !== null);

  function inplannen(b) {
    nieuw.medewerker_id = b.med.id;
    if (b.entry && b.entry.status === 'beschikbaar') {
      if (!b.entry.hele_dag && b.entry.van_tijd) nieuw.start_tijd = b.entry.van_tijd;
      if (!b.entry.hele_dag && b.entry.tot_tijd) nieuw.eind_tijd = b.entry.tot_tijd;
      if (b.entry.gewenste_functie) nieuw.functie = b.entry.gewenste_functie;
    }
    if (formRef) formRef.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }

  async function voegToe() {
    if (!nieuw.medewerker_id || !nieuw.datum || !nieuw.start_tijd) return;
    laden = true;
    await voegRoosterRegelToe({
      ...nieuw,
      medewerker_id: nieuw.medewerker_id,
      eind_tijd: nieuw.eind_tijd || null,
      functie: nieuw.functie || null,
    });
    rooster = await getRooster();
    nieuw = { medewerker_id: '', datum: huidigeDatumStr, start_tijd: '', eind_tijd: '', functie: '' };
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

<!-- Beschikbaarheid overzicht -->
{#if heeftBeschikbaarEntries || medewerkers.length > 0}
  <div class="beschikbaar-panel kaart">
    <div class="panel-header">
      <span class="panel-titel">Beschikbaarheid collega's</span>
      <span class="panel-sub">Wie heeft opgegeven beschikbaar te zijn?</span>
    </div>
    <div class="beschikbaar-rijen">
      {#each dagBeschikbaar as { med, entry }}
        <div class="beschikbaar-rij" class:heeft-entry={entry !== null}>
          <div class="b-avatar" style="background: {avatarKleur(med.id)}">{med.naam[0]}</div>
          <span class="b-naam">{med.naam}</span>
          {#if entry === null}
            <span class="b-status grijs">Niet opgegeven</span>
          {:else if entry.status === 'onbeschikbaar'}
            <span class="b-status rood">✕ Onbeschikbaar</span>
          {:else}
            <span class="b-status groen">
              ✓ {entry.hele_dag ? 'Hele dag' : `${entry.van_tijd}–${entry.tot_tijd}`}
            </span>
            {#if entry.gewenste_functie}
              <span class="gewenste-functie-chip">{entry.gewenste_functie}</span>
            {/if}
            {#if $isManager}
              <button class="inplan-knop" on:click={() => inplannen({ med, entry })}>
                + Inplannen
              </button>
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  </div>
{/if}

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
        {#if r.functie && FUNCTIE_KLEUREN[r.functie]}
          <div class="functie-chip" style="background:{FUNCTIE_KLEUREN[r.functie].bg};color:{FUNCTIE_KLEUREN[r.functie].kleur};border-color:{FUNCTIE_KLEUREN[r.functie].rand}">
            {r.functie}
          </div>
        {/if}
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
  <div class="toevoeg-kaart" bind:this={formRef}>
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
      <div class="form-rij">
        <label class="form-veld">
          <span class="veld-label">Functie <span class="optioneel">(optioneel)</span></span>
          <select bind:value={nieuw.functie}>
            <option value="">Geen functie...</option>
            {#each FUNCTIES as f}<option value={f}>{f}</option>{/each}
          </select>
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

  .dag-midden { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 0.25rem; }
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
    background: none; border: none; color: var(--tekst-zacht);
    font-size: 0.78rem; font-family: var(--font-body);
    cursor: pointer; transition: color var(--transition); padding: 0; font-weight: 600;
  }
  .terug-link:hover { color: var(--rood); }

  /* Beschikbaarheid panel */
  .beschikbaar-panel {
    padding: 1.25rem;
    margin-bottom: 1.5rem;
  }

  .panel-header {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
    margin-bottom: 1rem;
    flex-wrap: wrap;
  }

  .panel-titel {
    font-size: 0.72rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--tekst-zacht);
  }

  .panel-sub {
    font-size: 0.75rem;
    color: var(--tekst-extra);
  }

  .beschikbaar-rijen { display: flex; flex-direction: column; gap: 0.4rem; }

  .beschikbaar-rij {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    padding: 0.55rem 0.75rem;
    border-radius: 10px;
    background: var(--rand-licht);
    border: 1px solid transparent;
    transition: background var(--transition);
  }

  .beschikbaar-rij.heeft-entry { background: #fafafa; border-color: var(--rand); }

  .b-avatar {
    width: 30px;
    height: 30px;
    border-radius: 8px;
    color: white;
    font-size: 0.85rem;
    font-weight: 900;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
  }

  .b-naam { font-weight: 700; font-size: 0.88rem; color: var(--donker); flex: 1; min-width: 0; }

  .b-status {
    font-size: 0.78rem;
    font-weight: 800;
    padding: 0.2rem 0.6rem;
    border-radius: 20px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .b-status.groen { background: #ecfdf5; color: #059669; border: 1px solid #6ee7b7; }
  .b-status.rood  { background: var(--rood-licht); color: var(--rood-donker); border: 1px solid #fca5a5; }
  .b-status.grijs { background: var(--rand-licht); color: var(--tekst-extra); border: 1px solid var(--rand); font-weight: 600; }

  .gewenste-functie-chip {
    font-size: 0.68rem;
    font-weight: 800;
    padding: 0.15rem 0.5rem;
    border-radius: 20px;
    background: var(--goud-licht);
    color: var(--goud);
    border: 1px solid #fde68a;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .inplan-knop {
    padding: 0.3rem 0.75rem;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white;
    border: none;
    border-radius: 8px;
    font-family: var(--font-body);
    font-weight: 800;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all var(--transition);
    box-shadow: 0 1px 4px var(--rood-glow);
    white-space: nowrap;
    flex-shrink: 0;
  }
  .inplan-knop:hover { transform: translateY(-1px); box-shadow: 0 3px 8px var(--rood-glow); }

  /* Leeg */
  .leeg-staat {
    display: flex; flex-direction: column; align-items: center;
    gap: 0.5rem; padding: 3.5rem 2rem; text-align: center;
    background: var(--wit); border-radius: var(--radius-lg);
    border: 1.5px dashed var(--rand); margin-bottom: 2rem;
  }
  .leeg-icoon { font-size: 2.5rem; }
  .leeg-titel { font-weight: 800; font-size: 1rem; color: var(--donker); }
  .leeg-sub { font-size: 0.85rem; color: var(--tekst-zacht); max-width: 260px; }

  /* Shifts */
  .shift-lijst { display: flex; flex-direction: column; gap: 0.65rem; margin-bottom: 2rem; }

  .shift-kaart {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.9rem 1.25rem;
    flex-wrap: wrap;
  }
  .shift-kaart:hover { transform: translateY(-1px); }

  .shift-avatar {
    width: 44px; height: 44px; border-radius: 13px;
    color: white; font-size: 1.2rem; font-weight: 900;
    display: flex; align-items: center; justify-content: center;
    font-family: var(--font-display); flex-shrink: 0;
    box-shadow: 0 3px 10px rgba(0,0,0,0.12);
  }

  .shift-info { display: flex; flex-direction: column; gap: 0.25rem; flex: 1; }
  .shift-naam { font-weight: 800; font-size: 0.98rem; color: var(--donker); }

  .shift-tijdblok {
    display: flex; align-items: center; gap: 0.4rem; color: var(--tekst-zacht);
  }

  .shift-tijd {
    font-weight: 700; color: var(--tekst-zacht);
    font-family: var(--font-display); letter-spacing: 0.04em; font-size: 1rem;
  }

  .functie-chip {
    font-size: 0.72rem;
    font-weight: 800;
    padding: 0.22rem 0.65rem;
    border-radius: 20px;
    border: 1.5px solid;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .duur-chip {
    background: var(--goud-licht); color: var(--goud);
    border: 1.5px solid #fde68a; padding: 0.25rem 0.7rem;
    border-radius: 20px; font-size: 0.78rem; font-weight: 800;
    white-space: nowrap; flex-shrink: 0;
  }

  .verwijder-knop {
    background: none; border: none; cursor: pointer; color: #d1d5db;
    padding: 0.4rem; border-radius: 8px; display: flex;
    align-items: center; transition: all var(--transition); flex-shrink: 0;
  }
  .verwijder-knop:hover { color: var(--rood); background: var(--rood-licht); }

  /* Toevoegen */
  .toevoeg-kaart {
    background: var(--wit); border-radius: var(--radius-lg);
    border: 1px solid var(--rand); padding: 1.5rem; box-shadow: var(--schaduw);
  }

  .toevoeg-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1.25rem; }

  .toevoeg-icoon {
    width: 34px; height: 34px;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white; border-radius: 10px;
    display: flex; align-items: center; justify-content: center;
    font-size: 1.3rem; font-weight: 900; flex-shrink: 0;
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
    color: white; border: none; border-radius: var(--radius);
    padding: 0.75rem 1.75rem;
    font-family: var(--font-body); font-weight: 800; font-size: 0.95rem;
    cursor: pointer; transition: all var(--transition);
    box-shadow: 0 2px 8px var(--rood-glow);
  }
  .knop-toevoegen:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 14px var(--rood-glow); }
  .knop-toevoegen:disabled { opacity: 0.5; cursor: not-allowed; }

  @media (max-width: 480px) {
    .dag-naam-tekst { font-size: 1rem; }
    .form-rij { flex-direction: column; }
    .knop-toevoegen { width: 100%; }
    .beschikbaar-rij { flex-wrap: wrap; }
    .inplan-knop { width: 100%; margin-top: 0.25rem; }
  }
</style>
