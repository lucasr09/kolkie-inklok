<script>
  import { onMount } from 'svelte';
  import { sessie } from '../stores.js';
  import { getBeschikbaarheid, setBeschikbaarheid, verwijderBeschikbaarheid } from '../lib/api.js';

  let beschikbaarheid = [];
  let laden = true;

  const FUNCTIES = ['Kassa/Bediening', 'Friet', 'Snacks', 'Bakplaat', 'Afwas'];

  const nu = new Date();
  const vandaag = `${nu.getFullYear()}-${String(nu.getMonth() + 1).padStart(2, '0')}-${String(nu.getDate()).padStart(2, '0')}`;
  let bDatum = vandaag;
  let bStatus = 'beschikbaar';
  let bHeleDag = false;
  let bVan = '08:30';
  let bTot = '17:00';
  let bGewensteFunctie = '';
  let bLaden = false;
  let bBericht = '';
  let bBerichtType = '';

  const tijdOpties = [];
  for (let min = 510; min <= 1380; min += 30) {
    const h = String(Math.floor(min / 60)).padStart(2, '0');
    const m = String(min % 60).padStart(2, '0');
    tijdOpties.push(`${h}:${m}`);
  }

  onMount(async () => {
    beschikbaarheid = await getBeschikbaarheid();
    laden = false;
  });

  $: medewerker_id = $sessie?.medewerker_id;

  $: mijnBeschikbaarheid = beschikbaarheid
    .filter(b => b.medewerker_id === medewerker_id && b.datum >= vandaag)
    .sort((a, b) => a.datum.localeCompare(b.datum));

  $: verledenBeschikbaarheid = beschikbaarheid
    .filter(b => b.medewerker_id === medewerker_id && b.datum < vandaag)
    .sort((a, b) => b.datum.localeCompare(a.datum))
    .slice(0, 5);

  async function handleOpslaan() {
    if (!medewerker_id || bLaden) return;
    bLaden = true;
    bBericht = '';
    const res = await setBeschikbaarheid({
      medewerker_id,
      datum: bDatum,
      status: bStatus,
      hele_dag: bStatus === 'onbeschikbaar' ? false : bHeleDag,
      van_tijd: (bStatus === 'onbeschikbaar' || bHeleDag) ? null : bVan,
      tot_tijd: (bStatus === 'onbeschikbaar' || bHeleDag) ? null : bTot,
      gewenste_functie: bStatus === 'onbeschikbaar' ? null : (bGewensteFunctie || null),
    });
    if (res.status === 'ok') {
      beschikbaarheid = await getBeschikbaarheid();
      bBericht = 'Beschikbaarheid opgeslagen!';
      bBerichtType = 'ok';
    } else {
      bBericht = res.bericht || 'Opslaan mislukt';
      bBerichtType = 'fout';
    }
    bLaden = false;
    setTimeout(() => bBericht = '', 4000);
  }

  async function verwijder(id) {
    const res = await verwijderBeschikbaarheid(id);
    if (res.status === 'ok') beschikbaarheid = await getBeschikbaarheid();
  }

  function formatDatum(ds) {
    return new Date(ds + 'T00:00:00').toLocaleDateString('nl-NL', {
      weekday: 'long', day: 'numeric', month: 'long'
    });
  }

  function isVandaag(ds) { return ds === vandaag; }
</script>

<h1>Beschikbaarheid</h1>
<p class="pagina-sub">Geef aan wanneer je beschikbaar bent om te werken. De manager gebruikt dit bij het plannen van het rooster.</p>

<div class="layout">

  <!-- Formulier -->
  <div class="form-kaart kaart">
    <div class="form-titel">
      <span>Beschikbaarheid opgeven</span>
    </div>

    <div class="b-form">
      <div class="b-veld">
        <span class="b-label">Datum</span>
        <input type="date" bind:value={bDatum} min={vandaag} class="b-input" />
      </div>

      <div class="status-knoppen">
        <button
          class="status-knop beschikbaar-knop"
          class:actief={bStatus === 'beschikbaar'}
          on:click={() => bStatus = 'beschikbaar'}
          type="button"
        >
          <span class="status-dot groen"></span>
          Beschikbaar
        </button>
        <button
          class="status-knop onbeschikbaar-knop"
          class:actief={bStatus === 'onbeschikbaar'}
          on:click={() => bStatus = 'onbeschikbaar'}
          type="button"
        >
          <span class="status-dot rood"></span>
          Onbeschikbaar
        </button>
      </div>

      {#if bStatus === 'beschikbaar'}
        <div class="b-toggle-rij">
          <button
            class="b-toggle-knop"
            class:actief={bHeleDag}
            on:click={() => bHeleDag = !bHeleDag}
            type="button"
          >
            <span class="toggle-cirkel" class:actief={bHeleDag}></span>
            Hele dag beschikbaar
          </button>
        </div>

        {#if !bHeleDag}
          <div class="b-tijden">
            <div class="b-veld">
              <span class="b-label">Beschikbaar van</span>
              <select bind:value={bVan} class="b-input">
                {#each tijdOpties as t}
                  <option value={t}>{t}</option>
                {/each}
              </select>
            </div>
            <div class="b-streep-wrapper"><span class="b-streep">–</span></div>
            <div class="b-veld">
              <span class="b-label">Beschikbaar tot</span>
              <select bind:value={bTot} class="b-input">
                {#each tijdOpties as t}
                  <option value={t}>{t}</option>
                {/each}
              </select>
            </div>
          </div>
        {:else}
          <div class="hele-dag-preview">
            <span>Hele dag beschikbaar (08:30 – 23:00)</span>
          </div>
        {/if}
        <div class="b-veld">
          <span class="b-label">Voorkeursfunctie <span class="optioneel">(optioneel)</span></span>
          <select bind:value={bGewensteFunctie} class="b-input">
            <option value="">Geen voorkeur</option>
            {#each FUNCTIES as f}<option value={f}>{f}</option>{/each}
          </select>
        </div>
      {:else}
        <div class="onbeschikbaar-preview">
          <span>Je geeft aan dat je op deze dag <strong>niet</strong> beschikbaar bent.</span>
        </div>
      {/if}

      <button class="b-opslaan" on:click={handleOpslaan} disabled={bLaden}>
        {#if bLaden}
          <span class="spinner"></span> Bezig...
        {:else}
          Opslaan
        {/if}
      </button>

      {#if bBericht}
        <div class="b-melding" class:ok={bBerichtType === 'ok'} class:fout={bBerichtType === 'fout'}>
          {bBericht}
        </div>
      {/if}
    </div>
  </div>

  <!-- Lijst -->
  <div class="lijst-kolom">

    {#if laden}
      <div class="laad-staat">
        <div class="laad-spinner"></div>
        <span>Laden...</span>
      </div>
    {:else if mijnBeschikbaarheid.length === 0}
      <div class="leeg-staat kaart">
        <div class="leeg-titel">Nog niets opgegeven</div>
        <div class="leeg-sub">Geef aan wanneer je beschikbaar bent, zodat de manager je kan inplannen.</div>
      </div>
    {:else}
      <div class="lijst-kaart kaart">
        <div class="lijst-header">
          <span class="lijst-titel">Aankomende beschikbaarheid</span>
          <span class="lijst-count">{mijnBeschikbaarheid.length}</span>
        </div>
        <div class="b-lijst">
          {#each mijnBeschikbaarheid as b}
            <div class="b-item" class:vandaag={isVandaag(b.datum)}>
              {#if isVandaag(b.datum)}
                <span class="vandaag-chip">Vandaag</span>
              {/if}
              <div class="b-datum-tekst">{formatDatum(b.datum)}</div>
              <div class="b-item-footer">
                {#if b.status === 'onbeschikbaar'}
                  <span class="b-chip rood-chip">Onbeschikbaar</span>
                {:else}
                  <span class="b-chip">{b.hele_dag ? 'Hele dag' : `${b.van_tijd} – ${b.tot_tijd}`}</span>
                  {#if b.gewenste_functie}
                    <span class="b-chip functie-chip">{b.gewenste_functie}</span>
                  {/if}
                {/if}
                <button class="b-del" on:click={() => verwijder(b.id)} title="Verwijderen">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    {#if verledenBeschikbaarheid.length > 0}
      <div class="verleden-kaart kaart">
        <div class="lijst-header">
          <span class="lijst-titel verleden">Eerder opgegeven</span>
        </div>
        <div class="b-lijst verleden-lijst">
          {#each verledenBeschikbaarheid as b}
            <div class="b-item verleden">
              <div class="b-datum-tekst">{formatDatum(b.datum)}</div>
              <span class="b-chip verleden-chip">{b.hele_dag ? 'Hele dag' : `${b.van_tijd} – ${b.tot_tijd}`}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}

  </div>
</div>

<style>
  .pagina-sub {
    color: var(--tekst-zacht);
    margin-top: -1rem;
    margin-bottom: 2rem;
    font-size: 0.92rem;
    max-width: 560px;
  }

  .layout {
    display: grid;
    grid-template-columns: 360px 1fr;
    gap: 1.5rem;
    align-items: start;
  }

  /* Formulier */
  .form-kaart { padding: 1.5rem; }

  .form-titel {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    font-family: var(--font-display);
    font-size: 1.1rem;
    letter-spacing: 0.04em;
    color: var(--donker);
    margin-bottom: 1.5rem;
  }


  .b-form { display: flex; flex-direction: column; gap: 1rem; }

  .b-veld { display: flex; flex-direction: column; gap: 0.35rem; }
  .b-label { font-size: 0.72rem; font-weight: 800; text-transform: uppercase; letter-spacing: 0.06em; color: var(--tekst-zacht); }

  .b-input {
    padding: 0.6rem 0.85rem;
    border: 1.5px solid var(--rand);
    border-radius: 10px;
    font-family: var(--font-body);
    font-size: 0.9rem;
    color: var(--donker);
    background: var(--wit);
    transition: border-color var(--transition);
    width: 100%;
    font-weight: 600;
  }
  .b-input:focus { outline: none; border-color: var(--rood); box-shadow: 0 0 0 3px var(--rood-glow); }

  /* Status knoppen */
  .status-knoppen { display: flex; gap: 0.5rem; }

  .status-knop {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.7rem 1rem;
    border-radius: 10px;
    border: 2px solid var(--rand);
    background: var(--rand-licht);
    color: var(--tekst-zacht);
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 0.88rem;
    cursor: pointer;
    transition: all var(--transition);
  }

  .status-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .status-dot.groen { background: var(--groen); }
  .status-dot.rood { background: var(--rood); }

  .beschikbaar-knop.actief {
    background: var(--groen-licht);
    border-color: var(--groen);
    color: #166534;
  }

  .onbeschikbaar-knop.actief {
    background: var(--rood-licht);
    border-color: var(--rood);
    color: var(--rood-donker);
  }

  .status-knop:hover:not(.actief) {
    border-color: var(--tekst-zacht);
    color: var(--donker);
  }

  .onbeschikbaar-preview {
    padding: 0.75rem 1rem;
    background: var(--rood-licht);
    border: 1.5px solid #fca5a5;
    border-radius: 10px;
    font-size: 0.88rem;
    color: var(--rood-donker);
    font-weight: 600;
  }

  .optioneel { font-weight: 500; font-size: 0.65rem; text-transform: none; letter-spacing: 0; color: var(--tekst-extra); }

  .functie-chip {
    background: var(--goud-licht) !important;
    color: var(--goud) !important;
    border-color: #fde68a !important;
  }

  .rood-chip {
    background: var(--rood-licht) !important;
    color: var(--rood-donker) !important;
    border-color: #fca5a5 !important;
  }

  .b-toggle-rij { display: flex; }

  .b-toggle-knop {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    padding: 0.7rem 1rem;
    border-radius: 10px;
    border: 1.5px solid var(--rand);
    background: var(--rand-licht);
    color: var(--tekst-zacht);
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 0.88rem;
    cursor: pointer;
    transition: all var(--transition);
    width: 100%;
  }

  .b-toggle-knop.actief {
    background: var(--goud-licht);
    border-color: #d97706;
    color: #92400e;
  }

  .b-toggle-knop:hover:not(.actief) { border-color: var(--rood); color: var(--rood); background: var(--rood-licht); }

  .toggle-cirkel {
    width: 36px;
    height: 20px;
    border-radius: 10px;
    background: var(--rand);
    position: relative;
    flex-shrink: 0;
    transition: background var(--transition);
  }

  .toggle-cirkel::after {
    content: '';
    position: absolute;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: white;
    top: 3px;
    left: 3px;
    transition: transform var(--transition);
    box-shadow: 0 1px 4px rgba(0,0,0,0.2);
  }

  .toggle-cirkel.actief { background: #d97706; }
  .toggle-cirkel.actief::after { transform: translateX(16px); }

  .b-tijden { display: flex; align-items: flex-end; gap: 0.5rem; }
  .b-tijden .b-veld { flex: 1; }
  .b-streep-wrapper { padding-bottom: 0.65rem; }
  .b-streep { color: var(--tekst-zacht); font-weight: 700; font-size: 1.1rem; }

  .hele-dag-preview {
    padding: 0.75rem 1rem;
    background: var(--goud-licht);
    border: 1.5px solid #fde68a;
    border-radius: 10px;
    font-size: 0.88rem;
    font-weight: 700;
    color: #92400e;
  }

  .b-opslaan {
    width: 100%;
    padding: 0.85rem;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white;
    border: none;
    border-radius: 10px;
    font-family: var(--font-body);
    font-weight: 800;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all var(--transition);
    box-shadow: 0 2px 8px var(--rood-glow);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }
  .b-opslaan:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 14px var(--rood-glow); }
  .b-opslaan:disabled { opacity: 0.6; cursor: not-allowed; }

  .b-melding {
    font-size: 0.85rem;
    font-weight: 700;
    padding: 0.6rem 0.85rem;
    border-radius: 9px;
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }
  .b-melding.ok { background: var(--groen-licht); color: #166534; border: 1px solid var(--groen-rand); }
  .b-melding.fout { background: var(--rood-licht); color: var(--rood-donker); border: 1px solid #fca5a5; }

  .spinner {
    width: 14px; height: 14px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: draaien 0.6s linear infinite;
  }
  @keyframes draaien { to { transform: rotate(360deg); } }

  /* Lijst */
  .lijst-kolom { display: flex; flex-direction: column; gap: 1rem; }

  .laad-staat {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    color: var(--tekst-zacht);
    padding: 2rem;
    font-weight: 600;
  }

  .laad-spinner {
    width: 18px; height: 18px;
    border: 2px solid var(--rand);
    border-top-color: var(--rood);
    border-radius: 50%;
    animation: draaien 0.7s linear infinite;
  }

  .leeg-staat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 3rem 2rem;
    text-align: center;
  }
  .leeg-titel { font-weight: 800; color: var(--donker); }
  .leeg-sub { font-size: 0.85rem; color: var(--tekst-zacht); max-width: 280px; }

  .lijst-kaart, .verleden-kaart { padding: 1.25rem; }

  .lijst-header {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin-bottom: 1rem;
  }

  .lijst-titel {
    font-size: 0.72rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--tekst-zacht);
  }

  .lijst-titel.verleden { color: var(--tekst-extra); }

  .lijst-count {
    background: var(--rood);
    color: white;
    font-size: 0.62rem;
    font-weight: 900;
    min-width: 18px;
    height: 18px;
    border-radius: 9px;
    padding: 0 5px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .b-lijst { display: flex; flex-direction: column; gap: 0.5rem; }

  .b-item {
    padding: 0.85rem 1rem;
    border-radius: 12px;
    border: 1.5px solid var(--rand);
    background: var(--rand-licht);
    position: relative;
    transition: border-color var(--transition);
  }

  .b-item.vandaag {
    border-color: #86efac;
    background: var(--groen-licht);
  }

  .b-item.verleden { opacity: 0.6; }

  .vandaag-chip {
    font-size: 0.62rem;
    font-weight: 900;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    background: var(--groen);
    color: white;
    padding: 0.1rem 0.45rem;
    border-radius: 20px;
    display: inline-block;
    margin-bottom: 0.3rem;
  }

  .b-datum-tekst {
    font-weight: 700;
    font-size: 0.9rem;
    color: var(--donker);
    text-transform: capitalize;
    margin-bottom: 0.5rem;
  }

  .b-item-footer {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .b-chip {
    font-size: 0.78rem;
    font-weight: 800;
    padding: 0.25rem 0.65rem;
    border-radius: 20px;
    background: #ecfdf5;
    color: #059669;
    border: 1px solid #6ee7b7;
    flex: 1;
  }

  .verleden-chip {
    background: var(--rand-licht);
    color: var(--tekst-extra);
    border-color: var(--rand);
  }

  .b-del {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: none;
    background: rgba(220,38,38,0.08);
    color: var(--rood);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: all var(--transition);
  }
  .b-del:hover { background: var(--rood); color: white; }

  .verleden-lijst .b-item { padding: 0.6rem 0.85rem; }
  .verleden-lijst .b-datum-tekst { font-size: 0.82rem; margin-bottom: 0.3rem; }

  @media (max-width: 768px) {
    .layout { grid-template-columns: 1fr; }
  }
</style>
