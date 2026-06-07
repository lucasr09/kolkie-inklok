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
    const kleuren = ['#dc2626','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
    return kleuren[(id ?? 0) % kleuren.length];
  }

  function naarInput(isoStr) {
    if (!isoStr) return '';
    return new Date(isoStr).toISOString().slice(0, 16);
  }

  function naarIso(inputStr) {
    if (!inputStr) return null;
    return new Date(inputStr).toISOString();
  }

  function startBewerk(k) {
    bewerkId = k.id;
    bewerkData = { ingeklokt_op: naarInput(k.ingeklokt_op), uitgeklokt_op: naarInput(k.uitgeklokt_op) };
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
    return h > 0 ? `${h}u ${m}m` : `${m}m`;
  }

  function formatDatumTijd(isoStr) {
    if (!isoStr) return '—';
    return new Date(isoStr).toLocaleString('nl-NL', { day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit' });
  }

  $: gefilterd = filterMedewerker
    ? klokslagen.filter(k => k.medewerker_id === Number(filterMedewerker))
    : klokslagen;
</script>

<div class="pagina-header">
  <h1>Uren beheren</h1>
  <div class="teller">
    <span class="teller-getal">{gefilterd.length}</span>
    <span class="teller-label">klokslagen</span>
  </div>
</div>

<!-- Filter -->
<div class="filter-sectie">
  <div class="filter-wrapper">
    <svg class="filter-icoon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
      <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>
    </svg>
    <select bind:value={filterMedewerker} class="filter-select">
      <option value="">Alle medewerkers</option>
      {#each medewerkers as m}<option value={m.id}>{m.naam}</option>{/each}
    </select>
  </div>
</div>

<!-- Klokslagen -->
{#if gefilterd.length === 0}
  <div class="leeg-staat">
    <div class="leeg-icoon">🕐</div>
    <div class="leeg-titel">Geen klokslagen gevonden</div>
    <div class="leeg-sub">Er zijn nog geen klokslagen{filterMedewerker ? ' voor deze medewerker' : ''}.</div>
  </div>
{:else}
  <div class="klokslagen-lijst">
    {#each gefilterd as k}
      <div class="klokslag-kaart kaart" class:bezig={!k.uitgeklokt_op}>
        {#if bewerkId === k.id}
          <div class="bewerk-sectie">
            <div class="bewerk-wie">
              <div class="klein-avatar" style="background: {avatarKleur(k.medewerker_id)}">{(k.naam ?? naamVan(k.medewerker_id))[0]}</div>
              <strong class="bewerk-naam">{k.naam ?? naamVan(k.medewerker_id)}</strong>
            </div>
            <div class="bewerk-velden">
              <label class="form-veld">
                <span class="veld-label">Ingeklokt op</span>
                <input type="datetime-local" bind:value={bewerkData.ingeklokt_op} />
              </label>
              <label class="form-veld">
                <span class="veld-label">Uitgeklokt op <span class="optioneel">(optioneel)</span></span>
                <input type="datetime-local" bind:value={bewerkData.uitgeklokt_op} />
              </label>
            </div>
            <div class="bewerk-knoppen">
              <button class="knop-opslaan" on:click={slaBewerk}>Opslaan</button>
              <button class="knop-annuleer" on:click={() => bewerkId = null}>Annuleren</button>
            </div>
          </div>
        {:else}
          <div class="klein-avatar" style="background: {avatarKleur(k.medewerker_id)}">{(k.naam ?? naamVan(k.medewerker_id))[0]}</div>
          <div class="klokslag-inhoud">
            <span class="klokslag-naam">{k.naam ?? naamVan(k.medewerker_id)}</span>
            <span class="klokslag-tijden">
              {formatDatumTijd(k.ingeklokt_op)}
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
              {k.uitgeklokt_op ? formatDatumTijd(k.uitgeklokt_op) : '...'}
            </span>
          </div>
          {#if berekenUren(k)}
            <span class="duur-chip">{berekenUren(k)}</span>
          {:else if !k.uitgeklokt_op}
            <span class="bezig-chip">Bezig</span>
          {/if}
          <div class="kaart-acties">
            <button class="edit-knop" on:click={() => startBewerk(k)} title="Aanpassen">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
            </button>
            {#if bevestigVerwijder === k.id}
              <span class="bevestig-tekst">Zeker?</span>
              <button class="ja-knop" on:click={() => verwijder(k.id)}>Ja</button>
              <button class="nee-knop" on:click={() => bevestigVerwijder = null}>Nee</button>
            {:else}
              <button class="verwijder-knop" on:click={() => bevestigVerwijder = k.id} title="Verwijderen">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
                  <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </button>
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<!-- Toevoegen -->
<div class="toevoeg-kaart">
  <div class="toevoeg-header">
    <div class="toevoeg-icoon">+</div>
    <h2>Klokslag toevoegen</h2>
  </div>
  <div class="form">
    <label class="form-veld">
      <span class="veld-label">Medewerker</span>
      <select bind:value={nieuw.medewerker_id}>
        <option value="">Kies medewerker...</option>
        {#each medewerkers as m}<option value={m.id}>{m.naam}</option>{/each}
      </select>
    </label>
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
    <button class="knop-toevoegen" on:click={voegToe} disabled={!nieuw.medewerker_id || !nieuw.ingeklokt_op}>
      + Klokslag toevoegen
    </button>
  </div>
</div>

<style>
  .pagina-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .pagina-header h1 { margin-bottom: 0; }

  .teller {
    display: flex;
    align-items: baseline;
    gap: 0.3rem;
    background: var(--donker);
    color: white;
    padding: 0.4rem 0.9rem;
    border-radius: 20px;
  }

  .teller-getal { font-family: var(--font-display); font-size: 1.2rem; letter-spacing: 0.04em; }
  .teller-label { font-size: 0.72rem; font-weight: 700; opacity: 0.6; text-transform: uppercase; letter-spacing: 0.06em; }

  .filter-sectie { margin-bottom: 1.25rem; }

  .filter-wrapper {
    position: relative;
    max-width: 260px;
  }

  .filter-icoon {
    position: absolute;
    left: 0.85rem;
    top: 50%;
    transform: translateY(-50%);
    color: var(--tekst-zacht);
    pointer-events: none;
  }

  .filter-select {
    padding-left: 2.5rem;
    width: 100%;
  }

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
  .leeg-titel { font-weight: 800; color: var(--donker); }
  .leeg-sub { font-size: 0.85rem; color: var(--tekst-zacht); }

  /* Klokslagen */
  .klokslagen-lijst {
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
    margin-bottom: 2rem;
  }

  .klokslag-kaart {
    display: flex;
    align-items: center;
    gap: 0.85rem;
    padding: 0.85rem 1.1rem;
    flex-wrap: wrap;
  }

  .klokslag-kaart.bezig {
    border-left: 3px solid var(--groen);
  }

  .klein-avatar {
    width: 38px;
    height: 38px;
    border-radius: 11px;
    color: white;
    font-size: 1rem;
    font-weight: 900;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(0,0,0,0.12);
  }

  .klokslag-inhoud {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    flex: 1;
    min-width: 0;
  }

  .klokslag-naam { font-weight: 800; font-size: 0.9rem; color: var(--donker); }

  .klokslag-tijden {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.8rem;
    color: var(--tekst-zacht);
    font-weight: 600;
    flex-wrap: wrap;
  }

  .duur-chip {
    background: var(--goud-licht);
    color: var(--goud);
    border: 1.5px solid #fde68a;
    padding: 0.2rem 0.65rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 800;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .bezig-chip {
    background: var(--groen-licht);
    color: var(--groen);
    border: 1.5px solid var(--groen-rand);
    padding: 0.2rem 0.65rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 800;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .kaart-acties {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    margin-left: auto;
    flex-shrink: 0;
  }

  .edit-knop {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--tekst-extra);
    padding: 0.35rem;
    border-radius: 7px;
    display: flex;
    transition: all var(--transition);
  }

  .edit-knop:hover { color: var(--blauw); background: var(--blauw-licht); }

  .verwijder-knop {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--tekst-extra);
    padding: 0.35rem;
    border-radius: 7px;
    display: flex;
    transition: all var(--transition);
  }

  .verwijder-knop:hover { color: var(--rood); background: var(--rood-licht); }

  .bevestig-tekst { font-size: 0.78rem; color: var(--tekst-zacht); white-space: nowrap; font-weight: 600; }

  .ja-knop {
    background: var(--rood);
    color: white;
    border: none;
    border-radius: 7px;
    padding: 0.3rem 0.65rem;
    font-size: 0.78rem;
    font-weight: 800;
    cursor: pointer;
    font-family: var(--font-body);
  }

  .nee-knop {
    background: var(--rand-licht);
    color: var(--tekst);
    border: 1.5px solid var(--rand);
    border-radius: 7px;
    padding: 0.3rem 0.65rem;
    font-size: 0.78rem;
    font-weight: 700;
    cursor: pointer;
    font-family: var(--font-body);
  }

  /* Bewerk mode */
  .bewerk-sectie {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
    width: 100%;
    padding: 0.25rem 0;
  }

  .bewerk-wie { display: flex; align-items: center; gap: 0.6rem; }
  .bewerk-naam { font-size: 0.95rem; color: var(--donker); }

  .bewerk-velden {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .bewerk-knoppen { display: flex; gap: 0.5rem; }

  .knop-opslaan {
    background: linear-gradient(135deg, var(--groen), #15803d);
    color: white;
    border: none;
    border-radius: var(--radius);
    padding: 0.6rem 1.5rem;
    font-family: var(--font-body);
    font-weight: 800;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all var(--transition);
  }

  .knop-opslaan:hover { transform: translateY(-1px); }

  .knop-annuleer {
    background: var(--rand-licht);
    color: var(--tekst);
    border: 1.5px solid var(--rand);
    border-radius: var(--radius);
    padding: 0.6rem 1.2rem;
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all var(--transition);
  }

  /* Toevoegen */
  .toevoeg-kaart {
    background: var(--wit);
    border-radius: var(--radius-lg);
    border: 1px solid var(--rand);
    padding: 1.5rem;
    box-shadow: var(--schaduw);
  }

  .toevoeg-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1.25rem; }

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
  .form-veld { display: flex; flex-direction: column; gap: 0.35rem; flex: 1; min-width: 200px; }
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

  .knop-toevoegen:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 14px var(--rood-glow); }
  .knop-toevoegen:disabled { opacity: 0.45; cursor: not-allowed; }

  @media (max-width: 600px) {
    .bewerk-velden { grid-template-columns: 1fr; }
    .form-rij { flex-direction: column; }
    .knop-toevoegen { width: 100%; }
  }
</style>
