<script>
  import { onMount } from 'svelte';
  import { getMedewerkers, inklokken, uitklokken, getKlokslagen } from '../lib/api.js';
  import { sessie, isManager } from '../stores.js';

  let medewerkers = [];
  let gekozenId = null;
  let gekozenNaam = '';
  let klokslagen = [];
  let bericht = '';
  let berichtType = '';
  let ingeklokt = false;
  let laden = false;

  onMount(async () => {
    medewerkers = await getMedewerkers();

    // Werknemer wordt automatisch ingelogd als zichzelf
    if (!$isManager && $sessie?.medewerker_id) {
      await kiesMedewerker($sessie.medewerker_id, $sessie.gebruikersnaam);
    }
  });

  async function kiesMedewerker(id, naam) {
    gekozenId = id;
    gekozenNaam = naam;
    const data = await getKlokslagen(id);
    klokslagen = Array.isArray(data) ? data : [];
    ingeklokt = klokslagen.length > 0 && !klokslagen[0].uitgeklokt_op;
    bericht = '';
  }

  async function handleKlok() {
    if (!gekozenId || laden) return;
    laden = true;
    const res = ingeklokt ? await uitklokken(gekozenId) : await inklokken(gekozenId);
    bericht = res.bericht || (ingeklokt ? '✓ Uitgeklokt!' : '✓ Ingeklokt!');
    berichtType = res.status === 'ok' ? 'ok' : 'fout';
    const data = await getKlokslagen(gekozenId);
    klokslagen = Array.isArray(data) ? data : [];
    ingeklokt = klokslagen.length > 0 && !klokslagen[0].uitgeklokt_op;
    laden = false;
    setTimeout(() => bericht = '', 4000);
  }

  function berekenUren(slag) {
    if (!slag.uitgeklokt_op) return null;
    const ms = new Date(slag.uitgeklokt_op) - new Date(slag.ingeklokt_op);
    const h = Math.floor(ms / 3600000);
    const m = Math.floor((ms % 3600000) / 60000);
    return `${h}u ${m}m`;
  }

  function formatTijd(isoStr) {
    return new Date(isoStr).toLocaleTimeString('nl-NL', { hour: '2-digit', minute: '2-digit' });
  }

  function formatDatum(isoStr) {
    return new Date(isoStr).toLocaleDateString('nl-NL', { weekday: 'short', day: 'numeric', month: 'short' });
  }
</script>

{#if !gekozenId}
  <div class="kies-scherm">
    <div class="kies-header">
      <h1>Wie ben jij?</h1>
      <p class="subtitel">Kies je naam om in of uit te klokken</p>
    </div>
    <div class="medewerker-grid">
      {#each medewerkers as m}
        <button class="medewerker-knop" on:click={() => kiesMedewerker(m.id, m.naam)}>
          <span class="avatar">{m.naam[0].toUpperCase()}</span>
          <span class="med-naam">{m.naam}</span>
          <span class="med-rol">{m.rol === 'manager' ? '⭐ Manager' : '👤 Werknemer'}</span>
        </button>
      {/each}
    </div>
  </div>
{:else}
  <div class="klok-scherm">
    <div class="welkom-balk kaart">
      {#if $isManager}
        <button class="terug-knop" on:click={() => gekozenId = null}>← Wissel</button>
      {/if}
      <div class="welkom-tekst">
        <span class="welkom-hoi">Hoi,</span>
        <span class="welkom-naam">{gekozenNaam}</span>
      </div>
      <div class="avatar-groot">{gekozenNaam[0].toUpperCase()}</div>
    </div>

    <div class="status-kaart" class:ingeklokt>
      <div class="status-label">{ingeklokt ? 'Aan het werk' : 'Niet ingeklokt'}</div>
      <div class="status-dot" class:groen={ingeklokt} class:grijs={!ingeklokt}></div>
    </div>

    <button class="grote-klok-knop" class:uitklok={ingeklokt} on:click={handleKlok} disabled={laden}>
      {#if laden}
        <span class="spinner"></span>
      {:else}
        <span class="klok-icoon">{ingeklokt ? '🔴' : '🟢'}</span>
      {/if}
      <span class="klok-tekst">{ingeklokt ? 'Uitklokken' : 'Inklokken'}</span>
    </button>

    {#if bericht}
      <div class="bericht" class:ok={berichtType === 'ok'} class:fout={berichtType === 'fout'}>{bericht}</div>
    {/if}

    {#if klokslagen.length > 0}
      <h2>Recente diensten</h2>
      <div class="shifts-lijst">
        {#each klokslagen.slice(0, 5) as k}
          <div class="shift-kaart kaart">
            <div class="shift-datum">{formatDatum(k.ingeklokt_op)}</div>
            <div class="shift-tijden">
              <div class="shift-tijd">
                <span class="tijd-label">In</span>
                <span class="tijd-waarde">{formatTijd(k.ingeklokt_op)}</span>
              </div>
              <div class="shift-pijl">→</div>
              <div class="shift-tijd">
                <span class="tijd-label">Uit</span>
                <span class="tijd-waarde">{k.uitgeklokt_op ? formatTijd(k.uitgeklokt_op) : '...'}</span>
              </div>
              {#if berekenUren(k)}
                <div class="shift-duur">{berekenUren(k)}</div>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .kies-scherm { max-width: 600px; }
  .kies-header { margin-bottom: 2rem; }
  .subtitel { color: var(--tekst-zacht); margin-top: 0.25rem; }

  .medewerker-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 1rem; }

  .medewerker-knop {
    background: var(--wit); border: 2px solid var(--rand); border-radius: var(--radius);
    padding: 1.25rem 1rem; cursor: pointer; display: flex; flex-direction: column;
    align-items: center; gap: 0.4rem; transition: border-color 0.2s, transform 0.15s, box-shadow 0.2s;
    font-family: var(--font-body); box-shadow: var(--schaduw);
  }
  .medewerker-knop:hover { border-color: var(--rood); transform: translateY(-3px); box-shadow: var(--schaduw-groot); }

  .avatar {
    width: 52px; height: 52px; border-radius: 50%; background: var(--rood); color: white;
    font-size: 1.4rem; font-weight: 800; display: flex; align-items: center; justify-content: center;
    font-family: var(--font-display);
  }
  .med-naam { font-weight: 700; font-size: 0.95rem; color: var(--donker); }
  .med-rol { font-size: 0.72rem; color: var(--tekst-zacht); }

  .klok-scherm { max-width: 500px; }

  .welkom-balk {
    display: flex; align-items: center; gap: 1rem; margin-bottom: 1.5rem;
  }
  .terug-knop {
    background: none; border: 1.5px solid var(--rand); border-radius: 8px;
    padding: 0.4rem 0.8rem; cursor: pointer; font-family: var(--font-body);
    font-size: 0.8rem; font-weight: 700; color: var(--tekst-zacht); white-space: nowrap;
    transition: border-color 0.15s, color 0.15s;
  }
  .terug-knop:hover { border-color: var(--rood); color: var(--rood); }

  .welkom-tekst { flex: 1; display: flex; flex-direction: column; }
  .welkom-hoi { font-size: 0.8rem; color: var(--tekst-zacht); }
  .welkom-naam { font-family: var(--font-display); font-size: 1.5rem; color: var(--donker); letter-spacing: 0.03em; }

  .avatar-groot {
    width: 44px; height: 44px; border-radius: 50%; background: var(--rood); color: white;
    font-size: 1.2rem; font-weight: 800; display: flex; align-items: center; justify-content: center;
    font-family: var(--font-display); flex-shrink: 0;
  }

  .status-kaart {
    display: flex; align-items: center; justify-content: space-between;
    padding: 0.75rem 1.25rem; border-radius: var(--radius);
    background: var(--rood-licht); border: 1.5px solid #f5c6c6; margin-bottom: 1.5rem;
  }
  .status-kaart.ingeklokt { background: #edfdf5; border-color: #a7f3d0; }
  .status-label { font-weight: 700; font-size: 0.9rem; }
  .status-dot { width: 12px; height: 12px; border-radius: 50%; }
  .status-dot.groen { background: #22c55e; box-shadow: 0 0 0 4px rgba(34,197,94,0.2); }
  .status-dot.grijs { background: #d1d5db; }

  .grote-klok-knop {
    width: 100%; padding: 1.4rem; border-radius: 16px; border: none; cursor: pointer;
    background: var(--rood); color: white; font-family: var(--font-display);
    font-size: 1.8rem; letter-spacing: 0.06em; display: flex; align-items: center;
    justify-content: center; gap: 0.75rem;
    transition: background 0.15s, transform 0.1s;
    box-shadow: 0 4px 20px rgba(224,31,31,0.35); margin-bottom: 1rem;
  }
  .grote-klok-knop.uitklok { background: #374151; box-shadow: 0 4px 20px rgba(0,0,0,0.2); }
  .grote-klok-knop:hover:not(:disabled) { transform: translateY(-2px); }
  .grote-klok-knop:disabled { opacity: 0.7; cursor: not-allowed; }

  .spinner { width: 22px; height: 22px; border: 3px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 0.6s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }

  .bericht { padding: 0.75rem 1rem; border-radius: var(--radius); font-weight: 700; font-size: 0.95rem; margin-bottom: 1rem; text-align: center; }
  .bericht.ok { background: #edfdf5; color: #166534; border: 1.5px solid #a7f3d0; }
  .bericht.fout { background: var(--rood-licht); color: var(--rood-donker); border: 1.5px solid #f5c6c6; }

  .shifts-lijst { display: flex; flex-direction: column; gap: 0.75rem; }
  .shift-kaart { display: flex; flex-direction: column; gap: 0.5rem; }
  .shift-datum { font-size: 0.8rem; font-weight: 700; color: var(--tekst-zacht); text-transform: uppercase; letter-spacing: 0.06em; }
  .shift-tijden { display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; }
  .shift-tijd { display: flex; flex-direction: column; }
  .tijd-label { font-size: 0.7rem; color: var(--tekst-zacht); font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; }
  .tijd-waarde { font-weight: 800; font-size: 1.05rem; color: var(--donker); }
  .shift-pijl { color: var(--tekst-zacht); }
  .shift-duur { margin-left: auto; background: var(--geel-licht); color: #92400e; border: 1.5px solid #fde68a; padding: 0.25rem 0.6rem; border-radius: 6px; font-size: 0.8rem; font-weight: 700; }

  @media (max-width: 480px) {
    .medewerker-grid { grid-template-columns: repeat(2, 1fr); }
  }
</style>
