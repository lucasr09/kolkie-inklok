<script>
  import { onMount, onDestroy } from 'svelte';
  import { inklokken, uitklokken, getKlokslagen, startPauze, stopPauze } from '../lib/api.js';
  import { sessie } from '../stores.js';

  let gekozenId = $sessie?.medewerker_id ?? null;
  let gekozenNaam = $sessie?.gebruikersnaam ?? '';
  let klokslagen = [];
  let bericht = '';
  let berichtType = '';
  let ingeklokt = false;
  let opPauze = false;
  let laden = false;

  let nu = new Date();
  let klokInterval;

  const kleuren = ['#dc2626','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
  function avatarKleur(id) { return kleuren[(id ?? 0) % kleuren.length]; }

  onMount(async () => {
    if (gekozenId) {
      const data = await getKlokslagen(gekozenId);
      klokslagen = Array.isArray(data) ? data : [];
      const huidig = klokslagen[0];
      ingeklokt = !!huidig && !huidig.uitgeklokt_op;
      opPauze = ingeklokt && !!huidig?.pauze_start;
    }
    klokInterval = setInterval(() => { nu = new Date(); }, 1000);
  });

  onDestroy(() => {
    clearInterval(klokInterval);
  });

  async function verversKlokslagen() {
    const data = await getKlokslagen(gekozenId);
    klokslagen = Array.isArray(data) ? data : [];
    const huidig = klokslagen[0];
    ingeklokt = !!huidig && !huidig.uitgeklokt_op;
    opPauze = ingeklokt && !!huidig?.pauze_start;
  }

  async function handleKlok() {
    if (!gekozenId || laden) return;
    laden = true;
    const res = ingeklokt ? await uitklokken(gekozenId) : await inklokken(gekozenId);
    bericht = res.bericht || (ingeklokt ? 'Uitgeklokt!' : 'Ingeklokt!');
    berichtType = res.status === 'ok' ? 'ok' : 'fout';
    await verversKlokslagen();
    nuAanwezig = await getNuIngeklokt();
    laden = false;
    setTimeout(() => bericht = '', 4000);
  }

  async function handlePauze() {
    if (!gekozenId || laden) return;
    laden = true;
    const res = opPauze ? await stopPauze(gekozenId) : await startPauze(gekozenId);
    bericht = res.bericht || (opPauze ? 'Pauze gestopt!' : 'Pauze gestart!');
    berichtType = res.status === 'ok' ? 'ok' : 'fout';
    await verversKlokslagen();
    laden = false;
    setTimeout(() => bericht = '', 4000);
  }

  function berekenUren(slag) {
    if (!slag.uitgeklokt_op) return null;
    const brutoms = new Date(slag.uitgeklokt_op) - new Date(slag.ingeklokt_op);
    const nettoms = brutoms - (slag.pauze_totaal_ms || 0);
    const h = Math.floor(nettoms / 3600000);
    const m = Math.floor((nettoms % 3600000) / 60000);
    return h > 0 ? `${h}u ${m}m` : `${m}m`;
  }

  function formatTijd(isoStr) {
    return new Date(isoStr).toLocaleTimeString('nl-NL', { hour: '2-digit', minute: '2-digit', hour12: false });
  }

  function formatDatum(isoStr) {
    return new Date(isoStr).toLocaleDateString('nl-NL', { weekday: 'short', day: 'numeric', month: 'short' });
  }

  $: liveTijd = nu.toLocaleTimeString('nl-NL', { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false });
  $: liveDatum = nu.toLocaleDateString('nl-NL', { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' });
</script>

{#if gekozenId}
<!-- ── Klok scherm ── -->
  <div class="klok-wrapper">

    <!-- Live klok balk -->
    <div class="tijd-balk">
      <div class="tijd-groot">{liveTijd}</div>
      <div class="datum-sub">{liveDatum}</div>
    </div>

    <!-- Gebruiker balk -->
    <div class="gebruiker-balk kaart">
      <div class="welkom-inhoud">
        <div class="welkom-avatar" style="background: {avatarKleur(gekozenId)}">{gekozenNaam[0].toUpperCase()}</div>
        <div>
          <div class="welkom-label">Hoi,</div>
          <div class="welkom-naam">{gekozenNaam}</div>
        </div>
      </div>
    </div>

    <!-- Status indicator -->
    <div class="status-banner" class:actief={ingeklokt}>
      <div class="status-pulse-wrapper">
        <div class="status-dot" class:aan={ingeklokt}></div>
        {#if ingeklokt}<div class="status-ring"></div>{/if}
      </div>
      <span class="status-tekst">{ingeklokt ? 'Aan het werk' : 'Niet ingeklokt'}</span>
      {#if ingeklokt && klokslagen[0]}
        <span class="status-tijd">sinds {formatTijd(klokslagen[0].ingeklokt_op)}</span>
      {/if}
    </div>

    <!-- Grote knop -->
    <button
      class="klok-knop"
      class:uitklok={ingeklokt}
      on:click={handleKlok}
      disabled={laden}
    >
      {#if laden}
        <div class="knop-spinner"></div>
        <span>Bezig...</span>
      {:else}
        <div class="klok-knop-icoon">
          {#if ingeklokt}
            <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor"><rect x="4" y="4" width="16" height="16" rx="3"/></svg>
          {:else}
            <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,4 20,12 6,20"/></svg>
          {/if}
        </div>
        <span class="klok-knop-tekst">{ingeklokt ? 'Uitklokken' : 'Inklokken'}</span>
      {/if}
    </button>

    <!-- Pauze knop -->
    {#if ingeklokt}
      <button class="pauze-knop" class:stoppen={opPauze} on:click={handlePauze} disabled={laden}>
        {#if opPauze}
          Pauze beëindigen
        {:else}
          Pauze nemen
        {/if}
      </button>
    {/if}

    <!-- Feedback bericht -->
    {#if bericht}
      <div class="bericht" class:ok={berichtType === 'ok'} class:fout={berichtType === 'fout'}>
        {bericht}
      </div>
    {/if}

    <!-- Recente diensten -->
    {#if klokslagen.length > 0}
      <div class="shifts-sectie">
        <h2>Recente diensten</h2>
        <div class="shifts-lijst">
          {#each klokslagen.slice(0, 5) as k, i}
            <div class="shift-kaart" class:lopend={!k.uitgeklokt_op}>
              <div class="shift-index">{i + 1}</div>
              <div class="shift-datumblok">
                <span class="shift-datum">{formatDatum(k.ingeklokt_op)}</span>
              </div>
              <div class="shift-tijden">
                <div class="tijdblok">
                  <span class="tl">In</span>
                  <span class="tw">{formatTijd(k.ingeklokt_op)}</span>
                </div>
                <svg class="pijl-svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>
                </svg>
                <div class="tijdblok">
                  <span class="tl">Uit</span>
                  <span class="tw" class:nog-bezig={!k.uitgeklokt_op}>{k.uitgeklokt_op ? formatTijd(k.uitgeklokt_op) : '...'}</span>
                </div>
              </div>
              {#if berekenUren(k)}
                <div class="duur-badge">{berekenUren(k)}</div>
              {:else if !k.uitgeklokt_op}
                <div class="bezig-badge">Bezig</div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}


  </div>
{/if}

<style>
  /* ── Klok scherm ── */
  .klok-wrapper { max-width: 520px; display: flex; flex-direction: column; gap: 1rem; }

  .tijd-balk {
    background: linear-gradient(135deg, var(--donker) 0%, var(--donker-2) 100%);
    border-radius: var(--radius-xl);
    padding: 1.75rem 2rem;
    text-align: center;
    box-shadow: var(--schaduw-lg);
    border: 1px solid rgba(255,255,255,0.06);
  }

  .tijd-groot {
    font-family: var(--font-display);
    font-size: 3.5rem;
    letter-spacing: 0.12em;
    color: white;
    line-height: 1;
  }

  .datum-sub {
    font-size: 0.82rem;
    color: rgba(255,255,255,0.4);
    margin-top: 0.4rem;
    text-transform: capitalize;
    letter-spacing: 0.03em;
  }

  /* Gebruiker balk */
  .gebruiker-balk {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.85rem 1.1rem;
  }



  .welkom-inhoud {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
  }

  .welkom-avatar {
    width: 42px;
    height: 42px;
    border-radius: 12px;
    color: white;
    font-size: 1.2rem;
    font-weight: 900;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    box-shadow: 0 3px 10px rgba(0,0,0,0.12);
    flex-shrink: 0;
  }

  .welkom-label { font-size: 0.72rem; color: var(--tekst-zacht); font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; }
  .welkom-naam { font-family: var(--font-display); font-size: 1.4rem; letter-spacing: 0.03em; color: var(--donker); line-height: 1.1; }

  /* Status */
  .status-banner {
    display: flex;
    align-items: center;
    gap: 0.85rem;
    padding: 0.85rem 1.25rem;
    border-radius: var(--radius-lg);
    background: var(--rood-licht);
    border: 1.5px solid #fca5a5;
    transition: all var(--transition-md);
  }

  .status-banner.actief {
    background: var(--groen-licht);
    border-color: var(--groen-rand);
  }

  .status-pulse-wrapper { position: relative; width: 16px; height: 16px; flex-shrink: 0; }

  .status-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #d1d5db;
    position: absolute;
    top: 2px; left: 2px;
    transition: background var(--transition-md);
  }

  .status-dot.aan { background: var(--groen); }

  .status-ring {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    border: 2px solid var(--groen);
    animation: puls 1.5s ease-out infinite;
    opacity: 0;
  }

  @keyframes puls {
    0%   { transform: scale(0.8); opacity: 0.8; }
    100% { transform: scale(2.0); opacity: 0; }
  }

  .status-tekst { font-weight: 800; font-size: 0.9rem; color: var(--donker); }
  .status-tijd { margin-left: auto; font-size: 0.8rem; color: var(--tekst-zacht); font-weight: 600; }

  /* Grote klokknop */
  .klok-knop {
    width: 100%;
    padding: 1.5rem;
    border-radius: var(--radius-xl);
    border: none;
    cursor: pointer;
    background: linear-gradient(135deg, var(--rood) 0%, var(--rood-donker) 100%);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    transition: all var(--transition-md);
    box-shadow: 0 6px 24px var(--rood-glow), 0 2px 8px rgba(0,0,0,0.1);
    position: relative;
    overflow: hidden;
  }

  .klok-knop::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, rgba(255,255,255,0.12) 0%, transparent 60%);
    pointer-events: none;
  }

  .klok-knop.uitklok {
    background: linear-gradient(135deg, #374151 0%, #1f2937 100%);
    box-shadow: 0 6px 24px rgba(0,0,0,0.2), 0 2px 8px rgba(0,0,0,0.1);
  }

  .klok-knop:hover:not(:disabled) { transform: translateY(-2px); }
  .klok-knop:hover:not(:disabled):not(.uitklok) { box-shadow: 0 10px 32px var(--rood-glow); }
  .klok-knop:active:not(:disabled) { transform: translateY(0); }
  .klok-knop:disabled { opacity: 0.7; cursor: not-allowed; }

  .klok-knop-icoon { line-height: 1; display: flex; align-items: center; }
  .klok-knop-tekst { font-family: var(--font-display); font-size: 2rem; letter-spacing: 0.08em; }

  .knop-spinner {
    width: 26px; height: 26px;
    border: 3px solid rgba(255,255,255,0.25);
    border-top-color: white;
    border-radius: 50%;
    animation: draaien 0.6s linear infinite;
  }
  @keyframes draaien { to { transform: rotate(360deg); } }

  /* Bericht */
  .bericht {
    padding: 0.85rem 1.1rem;
    border-radius: var(--radius);
    font-weight: 700;
    font-size: 0.9rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .bericht.ok { background: var(--groen-licht); color: #166534; border: 1.5px solid var(--groen-rand); }
  .bericht.fout { background: var(--rood-licht); color: var(--rood-donker); border: 1.5px solid #fca5a5; }

  /* Shifts */
  .shifts-sectie h2 { margin-top: 0.5rem; }

  .shifts-lijst { display: flex; flex-direction: column; gap: 0.6rem; }

  .shift-kaart {
    background: var(--wit);
    border: 1px solid var(--rand);
    border-radius: var(--radius-lg);
    padding: 0.85rem 1.1rem;
    display: flex;
    align-items: center;
    gap: 1rem;
    box-shadow: var(--schaduw-xs);
    transition: box-shadow var(--transition);
  }

  .shift-kaart:hover { box-shadow: var(--schaduw); }

  .shift-kaart.lopend {
    border-left: 3px solid var(--groen);
    background: linear-gradient(90deg, #f0fdf4 0%, white 30%);
  }

  .shift-index {
    font-size: 0.7rem;
    font-weight: 800;
    color: var(--tekst-extra);
    width: 16px;
    text-align: center;
    flex-shrink: 0;
  }

  .shift-datumblok { flex-shrink: 0; }
  .shift-datum {
    font-size: 0.78rem;
    font-weight: 800;
    color: var(--tekst-zacht);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .shift-tijden {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    flex: 1;
  }

  .tijdblok { display: flex; flex-direction: column; }
  .tl { font-size: 0.62rem; color: var(--tekst-extra); font-weight: 800; text-transform: uppercase; letter-spacing: 0.06em; }
  .tw { font-weight: 800; font-size: 1rem; color: var(--donker); font-family: var(--font-display); letter-spacing: 0.04em; }
  .tw.nog-bezig { color: var(--groen); }

  .pijl-svg { color: var(--rand); flex-shrink: 0; }

  .duur-badge {
    background: var(--goud-licht);
    color: var(--goud);
    border: 1.5px solid #fde68a;
    padding: 0.2rem 0.65rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 800;
    white-space: nowrap;
    margin-left: auto;
    flex-shrink: 0;
  }

  .bezig-badge {
    background: var(--groen-licht);
    color: var(--groen);
    border: 1.5px solid var(--groen-rand);
    padding: 0.2rem 0.65rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 800;
    white-space: nowrap;
    margin-left: auto;
    flex-shrink: 0;
  }

  /* Pauze knop */
  .pauze-knop {
    width: 100%;
    padding: 0.85rem 1.5rem;
    border-radius: var(--radius-lg);
    border: 2px solid #d97706;
    background: var(--goud-licht);
    color: #92400e;
    font-family: var(--font-body);
    font-size: 0.95rem;
    font-weight: 800;
    cursor: pointer;
    transition: all var(--transition);
    letter-spacing: 0.02em;
  }

  .pauze-knop:hover:not(:disabled) {
    background: #fef3c7;
    border-color: #b45309;
    transform: translateY(-1px);
  }

  .pauze-knop.stoppen {
    background: #f0fdf4;
    border-color: var(--groen);
    color: #166534;
  }

  .pauze-knop.stoppen:hover:not(:disabled) {
    background: #dcfce7;
    border-color: #15803d;
  }

  .pauze-knop:disabled { opacity: 0.6; cursor: not-allowed; }

  @media (max-width: 480px) {
    .tijd-groot { font-size: 2.8rem; }
    .klok-knop-tekst { font-size: 1.6rem; }
  }
</style>
