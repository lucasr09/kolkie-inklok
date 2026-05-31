<script>
  import { onMount } from 'svelte';
  import { getAlleKlokslagen, getMedewerkers, patchKlokslag, postKlokslag, deleteKlokslag } from '../lib/api.js';

  let klokslagen = [];
  let medewerkers = [];
  let bewerkId = null;
  let bewerkData = {};
  let nieuw = { medewerker_id: '', ingeklokt_op: '', uitgeklokt_op: '' };
  let filterMedewerker = '';
  let bevestigVerwijder = null;

  onMount(async () => {
    [klokslagen, medewerkers] = await Promise.all([getAlleKlokslagen(), getMedewerkers()]);
  });

  function naamVan(id) { return medewerkers.find(m => m.id === id)?.naam ?? '?'; }

  function avatarKleur(id) {
    const kleuren = ['#e01f1f','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
    return kleuren[id % kleuren.length];
  }

  // ISO datum naar datetime-local formaat
  function naarInput(isoStr) {
    if (!isoStr) return '';
    return new Date(isoStr).toISOString().slice(0, 16);
  }

  // datetime-local naar ISO
  function naarIso(inputStr) {
    if (!inputStr) return null;
    return new Date(inputStr).toISOString();
  }

  function startBewerk(k) {
    bewerkId = k.id;
    bewerkData = {
      ingeklokt_op: naarInput(k.ingeklokt_op),
      uitgeklokt_op: naarInput(k.uitgeklokt_op),
    };
  }

  async function slaBewerk() {
    await patchKlokslag(bewerkId, {
      ingeklokt_op: naarIso(bewerkData.ingeklokt_op),
      uitgeklokt_op: naarIso(bewerkData.uitgeklokt_op),
    });
    klokslagen = await getAlleKlokslagen();
    bewerkId = null;
  }

  async function voegToe() {
    if (!nieuw.medewerker_id || !nieuw.ingeklokt_op) return;
    await postKlokslag({
      medewerker_id: Number(nieuw.medewerker_id),
      ingeklokt_op: naarIso(nieuw.ingeklokt_op),
      uitgeklokt_op: naarIso(nieuw.uitgeklokt_op),
    });
    klokslagen = await getAlleKlokslagen();
    nieuw = { medewerker_id: '', ingeklokt_op: '', uitgeklokt_op: '' };
  }

  async function verwijder(id) {
    await deleteKlokslag(id);
    klokslagen = await getAlleKlokslagen();
    bevestigVerwijder = null;
  }

  function berekenUren(k) {
    if (!k.uitgeklokt_op) return null;
    const ms = new Date(k.uitgeklokt_op) - new Date(k.ingeklokt_op);
    const h = Math.floor(ms / 3600000);
    const m = Math.floor((ms % 3600000) / 60000);
    return `${h}u ${m}m`;
  }

  function formatDatumTijd(isoStr) {
    if (!isoStr) return '—';
    return new Date(isoStr).toLocaleString('nl-NL', { day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit' });
  }

  $: gefilterd = filterMedewerker
    ? klokslagen.filter(k => k.medewerker_id === Number(filterMedewerker))
    : klokslagen;
</script>

<h1>Uren beheren</h1>

<!-- Filter -->
<div class="filter-balk">
  <select bind:value={filterMedewerker}>
    <option value="">Alle medewerkers</option>
    {#each medewerkers as m}<option value={m.id}>{m.naam}</option>{/each}
  </select>
</div>

<!-- Klokslagen lijst -->
<div class="klokslagen-lijst">
  {#each gefilterd as k}
    <div class="klokslag-kaart kaart" class:bezig={!k.uitgeklokt_op}>
      {#if bewerkId === k.id}
        <!-- Bewerk modus -->
        <div class="bewerk-form">
          <div class="bewerk-header">
            <div class="shift-avatar" style="background: {avatarKleur(k.medewerker_id)}">{(k.naam ?? naamVan(k.medewerker_id))[0]}</div>
            <strong>{k.naam ?? naamVan(k.medewerker_id)}</strong>
          </div>
          <label class="bewerk-veld">
            <span class="veld-label">Ingeklokt op</span>
            <input type="datetime-local" bind:value={bewerkData.ingeklokt_op} />
          </label>
          <label class="bewerk-veld">
            <span class="veld-label">Uitgeklokt op <span class="optioneel">(optioneel)</span></span>
            <input type="datetime-local" bind:value={bewerkData.uitgeklokt_op} />
          </label>
          <div class="bewerk-knoppen">
            <button class="knop-primair opslaan" on:click={slaBewerk}>Opslaan</button>
            <button class="knop-annuleer" on:click={() => bewerkId = null}>Annuleren</button>
          </div>
        </div>
      {:else}
        <!-- Weergave modus -->
        <div class="shift-avatar" style="background: {avatarKleur(k.medewerker_id)}">{(k.naam ?? naamVan(k.medewerker_id))[0]}</div>
        <div class="klokslag-info">
          <span class="klokslag-naam">{k.naam ?? naamVan(k.medewerker_id)}</span>
          <span class="klokslag-tijden">
            {formatDatumTijd(k.ingeklokt_op)} → {k.uitgeklokt_op ? formatDatumTijd(k.uitgeklokt_op) : '...'}
          </span>
        </div>
        {#if berekenUren(k)}
          <span class="shift-duur">{berekenUren(k)}</span>
        {:else if !k.uitgeklokt_op}
          <span class="bezig-badge">Bezig</span>
        {/if}
        <div class="kaart-acties">
          <button class="edit-knop" on:click={() => startBewerk(k)} title="Aanpassen">✏️</button>
          {#if bevestigVerwijder === k.id}
            <span class="bevestig-tekst">Zeker?</span>
            <button class="ja-knop" on:click={() => verwijder(k.id)}>Ja</button>
            <button class="nee-knop" on:click={() => bevestigVerwijder = null}>Nee</button>
          {:else}
            <button class="verwijder-knop" on:click={() => bevestigVerwijder = k.id}>✕</button>
          {/if}
        </div>
      {/if}
    </div>
  {/each}
</div>

<!-- Nieuwe klokslag toevoegen -->
<div class="toevoeg-sectie">
  <h2>Klokslag toevoegen</h2>
  <div class="form">
    <div class="form-rij">
      <label class="form-veld">
        <span class="veld-label">Medewerker</span>
        <select bind:value={nieuw.medewerker_id}>
          <option value="">Kies medewerker</option>
          {#each medewerkers as m}<option value={m.id}>{m.naam}</option>{/each}
        </select>
      </label>
    </div>
    <div class="form-rij">
      <label class="form-veld">
        <span class="veld-label">Ingeklokt op</span>
        <input type="datetime-local" bind:value={nieuw.ingeklokt_op} />
      </label>
      <label class="form-veld">
        <span class="veld-label">Uitgeklokt op <span class="optioneel">(optioneel)</span></span>
        <input type="datetime-local" bind:value={nieuw.uitgeklokt_op} />
      </label>
    </div>
    <button class="knop-primair" on:click={voegToe}>+ Toevoegen</button>
  </div>
</div>

<style>
  .filter-balk {
    margin-bottom: 1.5rem;
    max-width: 280px;
  }

  .klokslagen-lijst {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 2rem;
  }

  .klokslag-kaart {
    display: flex;
    align-items: center;
    gap: 0.9rem;
    padding: 0.9rem 1.1rem;
    flex-wrap: wrap;
  }
  .klokslag-kaart.bezig { border-left: 3px solid #22c55e; }

  .shift-avatar {
    width: 40px; height: 40px; border-radius: 50%; color: white;
    font-size: 1.1rem; font-weight: 800; display: flex; align-items: center;
    justify-content: center; font-family: var(--font-display); flex-shrink: 0;
  }

  .klokslag-info { display: flex; flex-direction: column; gap: 0.15rem; flex: 1; min-width: 0; }
  .klokslag-naam { font-weight: 700; font-size: 0.95rem; color: var(--donker); }
  .klokslag-tijden { font-size: 0.82rem; color: var(--tekst-zacht); font-weight: 600; }

  .shift-duur { background: var(--geel-licht); color: #92400e; border: 1.5px solid #fde68a; padding: 0.2rem 0.6rem; border-radius: 6px; font-size: 0.78rem; font-weight: 700; white-space: nowrap; }
  .bezig-badge { background: #edfdf5; color: #166534; border: 1.5px solid #a7f3d0; padding: 0.2rem 0.6rem; border-radius: 6px; font-size: 0.78rem; font-weight: 700; }

  .kaart-acties { display: flex; align-items: center; gap: 0.3rem; margin-left: auto; flex-shrink: 0; }
  .edit-knop { background: none; border: none; cursor: pointer; font-size: 0.9rem; padding: 0.3rem; border-radius: 4px; opacity: 0.5; transition: opacity 0.15s; }
  .edit-knop:hover { opacity: 1; }
  .verwijder-knop { background: none; border: none; cursor: pointer; color: #d1d5db; font-size: 0.9rem; padding: 0.3rem; border-radius: 4px; transition: color 0.15s; }
  .verwijder-knop:hover { color: var(--rood); }
  .bevestig-tekst { font-size: 0.8rem; color: var(--tekst-zacht); white-space: nowrap; }
  .ja-knop { background: var(--rood); color: white; border: none; border-radius: 6px; padding: 0.25rem 0.5rem; font-size: 0.78rem; font-weight: 700; cursor: pointer; }
  .nee-knop { background: #f3f4f6; color: var(--tekst); border: none; border-radius: 6px; padding: 0.25rem 0.5rem; font-size: 0.78rem; font-weight: 700; cursor: pointer; }

  /* Bewerk modus */
  .bewerk-form { display: flex; flex-direction: column; gap: 0.75rem; width: 100%; }
  .bewerk-header { display: flex; align-items: center; gap: 0.75rem; }
  .bewerk-veld { display: flex; flex-direction: column; gap: 0.3rem; }
  .veld-label { font-size: 0.8rem; font-weight: 700; color: var(--tekst-zacht); text-transform: uppercase; letter-spacing: 0.05em; }
  .optioneel { font-weight: 400; text-transform: none; }
  .bewerk-knoppen { display: flex; gap: 0.5rem; }
  .knop-annuleer { background: #f3f4f6; color: var(--tekst); border: none; border-radius: var(--radius); padding: 0.6rem 1.2rem; font-family: var(--font-body); font-weight: 700; cursor: pointer; font-size: 0.95rem; }
  .opslaan { padding: 0.6rem 1.5rem; }

  /* Toevoegen */
  .toevoeg-sectie { background: var(--wit); border: 1.5px solid var(--rand); border-radius: var(--radius); padding: 1.5rem; box-shadow: var(--schaduw); }
  .toevoeg-sectie h2 { margin-top: 0; }
  .form { display: flex; flex-direction: column; gap: 0.75rem; }
  .form-rij { display: flex; gap: 0.75rem; flex-wrap: wrap; }
  .form-veld { display: flex; flex-direction: column; gap: 0.3rem; flex: 1; min-width: 200px; }
  .knop-primair { align-self: flex-start; padding: 0.75rem 1.75rem; }

  @media (max-width: 480px) {
    .form-rij { flex-direction: column; }
    .knop-primair { width: 100%; }
    .klokslag-tijden { font-size: 0.75rem; }
  }
</style>
