<script>
  import { onMount } from 'svelte';
  import { getRooster, getMedewerkers, voegRoosterRegelToe, verwijderRoosterRegel } from '../lib/api.js';
  import { sessie, isManager } from '../stores.js';

  let rooster = [];
  let medewerkers = [];
  let nieuw = { medewerker_id: '', datum: '', start_tijd: '', eind_tijd: '' };
  let huidigeDatum = new Date();

  onMount(async () => {
    [rooster, medewerkers] = await Promise.all([getRooster(), getMedewerkers()]);
    if ($sessie?.medewerker_id) nieuw.medewerker_id = String($sessie.medewerker_id);
    nieuw.datum = datumNaarString(huidigeDatum);
  });

  function datumNaarString(d) {
    return d.toISOString().split('T')[0];
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
    const kleuren = ['#e01f1f','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
    return kleuren[id % kleuren.length];
  }

  $: huidigeDatumStr = datumNaarString(huidigeDatum);

  $: dagShifts = rooster.filter(r => r.datum === huidigeDatumStr)
    .sort((a, b) => a.start_tijd.localeCompare(b.start_tijd));

  $: isVandaag = huidigeDatumStr === datumNaarString(new Date());

  function formatDagHeader(d) {
    return d.toLocaleDateString('nl-NL', { weekday: 'long', day: 'numeric', month: 'long' });
  }

  async function voegToe() {
    if (!nieuw.medewerker_id || !nieuw.datum || !nieuw.start_tijd) return;
    await voegRoosterRegelToe({ ...nieuw, medewerker_id: Number(nieuw.medewerker_id), eind_tijd: nieuw.eind_tijd || null });
    rooster = await getRooster();
    nieuw = { medewerker_id: $sessie?.medewerker_id ? String($sessie.medewerker_id) : '', datum: huidigeDatumStr, start_tijd: '', eind_tijd: '' };
  }

  async function verwijder(id) {
    await verwijderRoosterRegel(id);
    rooster = await getRooster();
  }
</script>

<h1>Dagplanning</h1>

<!-- Dag navigator -->
<div class="dag-navigator">
  <button class="nav-pijl" on:click={vorigedag}>‹</button>
  <div class="dag-midden">
    <div class="dag-naam" class:vandaag={isVandaag}>
      {#if isVandaag}<span class="vandaag-badge">Vandaag</span>{/if}
      {formatDagHeader(huidigeDatum)}
    </div>
    {#if !isVandaag}
      <button class="terug-naar-vandaag" on:click={naarVandaag}>Naar vandaag</button>
    {/if}
  </div>
  <button class="nav-pijl" on:click={volgendedag}>›</button>
</div>

<!-- Planning van de dag -->
{#if dagShifts.length === 0}
  <div class="leeg">
    <span class="leeg-icoon">📭</span>
    <p>Niemand ingepland op deze dag</p>
    {#if $isManager}<p class="leeg-sub">Voeg hieronder een shift toe</p>{/if}
  </div>
{:else}
  <div class="shift-lijst">
    {#each dagShifts as r}
      <div class="shift-kaart kaart">
        <div class="shift-avatar" style="background: {avatarKleur(r.medewerker_id)}">
          {naamVan(r.medewerker_id)[0]}
        </div>
        <div class="shift-info">
          <span class="shift-naam">{naamVan(r.medewerker_id)}</span>
          <span class="shift-tijd">{r.start_tijd} – {r.eind_tijd ?? '?'}</span>
        </div>
        {#if r.eind_tijd}
          {@const minuten = (parseInt(r.eind_tijd.split(':')[0]) - parseInt(r.start_tijd.split(':')[0])) * 60 + (parseInt(r.eind_tijd.split(':')[1]) - parseInt(r.start_tijd.split(':')[1]))}
          <span class="shift-duur">{Math.floor(minuten/60)}u{minuten%60 > 0 ? ` ${minuten%60}m` : ''}</span>
        {/if}
        {#if $isManager}
          <button class="verwijder-knop" on:click={() => verwijder(r.id)}>✕</button>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<!-- Shift toevoegen (alleen manager) -->
{#if $isManager}
  <div class="toevoeg-sectie">
    <h2>Shift toevoegen</h2>
    <div class="form">
      <div class="form-rij">
        <label class="form-veld">
          <span class="veld-label">Medewerker</span>
          <select bind:value={nieuw.medewerker_id}>
            <option value="">Kies medewerker</option>
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
      <button class="knop-primair" on:click={voegToe}>+ Shift toevoegen</button>
    </div>
  </div>
{/if}

<style>
  /* Dag navigator */
  .dag-navigator {
    display: flex;
    align-items: center;
    gap: 1rem;
    background: var(--wit);
    border: 1.5px solid var(--rand);
    border-radius: var(--radius);
    padding: 1rem 1.25rem;
    margin-bottom: 1.75rem;
    box-shadow: var(--schaduw);
  }

  .nav-pijl {
    background: none;
    border: 1.5px solid var(--rand);
    border-radius: 8px;
    width: 40px;
    height: 40px;
    font-size: 1.4rem;
    cursor: pointer;
    color: var(--tekst-zacht);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.15s, color 0.15s, background 0.15s;
    flex-shrink: 0;
    line-height: 1;
  }
  .nav-pijl:hover { border-color: var(--rood); color: var(--rood); background: var(--rood-licht); }

  .dag-midden {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.3rem;
  }

  .dag-naam {
    font-family: var(--font-display);
    font-size: 1.3rem;
    letter-spacing: 0.04em;
    color: var(--donker);
    text-transform: capitalize;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
    justify-content: center;
    text-align: center;
  }
  .dag-naam.vandaag { color: var(--rood); }

  .vandaag-badge {
    background: var(--rood);
    color: white;
    font-family: var(--font-body);
    font-size: 0.65rem;
    font-weight: 800;
    padding: 0.15rem 0.5rem;
    border-radius: 20px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .terug-naar-vandaag {
    background: none;
    border: none;
    color: var(--tekst-zacht);
    font-size: 0.78rem;
    font-family: var(--font-body);
    cursor: pointer;
    text-decoration: underline;
    padding: 0;
  }
  .terug-naar-vandaag:hover { color: var(--rood); }

  /* Shifts */
  .leeg {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 3rem 1rem;
    color: var(--tekst-zacht);
    gap: 0.5rem;
  }
  .leeg-icoon { font-size: 2.5rem; }
  .leeg p { font-weight: 600; margin: 0; }
  .leeg-sub { font-weight: 400; font-size: 0.85rem; }

  .shift-lijst {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 2rem;
  }

  .shift-kaart {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.25rem;
  }

  .shift-avatar {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    color: white;
    font-size: 1.2rem;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
  }

  .shift-info {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    flex: 1;
  }
  .shift-naam { font-weight: 700; font-size: 1rem; color: var(--donker); }
  .shift-tijd { font-size: 0.85rem; color: var(--tekst-zacht); font-weight: 600; }

  .shift-duur {
    background: var(--geel-licht);
    color: #92400e;
    border: 1.5px solid #fde68a;
    padding: 0.2rem 0.6rem;
    border-radius: 6px;
    font-size: 0.78rem;
    font-weight: 700;
    white-space: nowrap;
  }

  .verwijder-knop {
    background: none;
    border: none;
    cursor: pointer;
    color: #d1d5db;
    font-size: 0.9rem;
    padding: 0.3rem;
    border-radius: 4px;
    transition: color 0.15s;
    flex-shrink: 0;
  }
  .verwijder-knop:hover { color: var(--rood); }

  /* Toevoegen */
  .toevoeg-sectie {
    background: var(--wit);
    border: 1.5px solid var(--rand);
    border-radius: var(--radius);
    padding: 1.5rem;
    box-shadow: var(--schaduw);
  }
  .toevoeg-sectie h2 { margin-top: 0; }
  .form { display: flex; flex-direction: column; gap: 0.75rem; }
  .form-rij { display: flex; gap: 0.75rem; flex-wrap: wrap; }
  .form-veld { display: flex; flex-direction: column; gap: 0.3rem; flex: 1; min-width: 150px; }
  .veld-label { font-size: 0.8rem; font-weight: 700; color: var(--tekst-zacht); text-transform: uppercase; letter-spacing: 0.05em; }
  .optioneel { font-weight: 400; text-transform: none; }
  .knop-primair { align-self: flex-start; padding: 0.75rem 1.75rem; }

  @media (max-width: 480px) {
    .dag-naam { font-size: 1rem; }
    .form-rij { flex-direction: column; }
    .knop-primair { width: 100%; }
  }
</style>
