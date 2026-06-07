<script>
  import { onMount } from 'svelte';
  import { getAlleKlokslagen, getRooster, getMedewerkers, getNuIngeklokt, getBeschikbaarheid } from '../lib/api.js';

  let klokslagen = [];
  let rooster = [];
  let medewerkers = [];
  let nuAanwezig = [];
  let beschikbaarheid = [];
  let gekozenWeek = huidigeMaandag();
  let laden = true;

  function huidigeMaandag() {
    const d = new Date();
    const dag = d.getDay();
    const diff = dag === 0 ? -6 : 1 - dag;
    d.setDate(d.getDate() + diff);
    d.setHours(0, 0, 0, 0);
    return d;
  }

  function weekZondag(maandag) {
    const d = new Date(maandag);
    d.setDate(d.getDate() + 6);
    return d;
  }

  function vorigeWeek() {
    const d = new Date(gekozenWeek);
    d.setDate(d.getDate() - 7);
    gekozenWeek = d;
  }

  function volgendeWeek() {
    const d = new Date(gekozenWeek);
    d.setDate(d.getDate() + 7);
    gekozenWeek = d;
  }

  function datumStr(d) {
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  }

  function formatWeek(maandag) {
    const zondag = weekZondag(maandag);
    return `${maandag.toLocaleDateString('nl-NL', { day: 'numeric', month: 'short' })} – ${zondag.toLocaleDateString('nl-NL', { day: 'numeric', month: 'short', year: 'numeric' })}`;
  }

  $: isHuidigeWeek = datumStr(gekozenWeek) === datumStr(huidigeMaandag());

  onMount(async () => {
    [klokslagen, rooster, medewerkers, nuAanwezig, beschikbaarheid] = await Promise.all([
      getAlleKlokslagen(), getRooster(), getMedewerkers(), getNuIngeklokt(), getBeschikbaarheid()
    ]);
    laden = false;
  });

  function gewerktMs(k) {
    if (!k.uitgeklokt_op) return 0;
    return new Date(k.uitgeklokt_op) - new Date(k.ingeklokt_op);
  }

  function ingeplandMinuten(r) {
    if (!r.eind_tijd) return null;
    const [sh, sm] = r.start_tijd.split(':').map(Number);
    const [eh, em] = r.eind_tijd.split(':').map(Number);
    return (eh * 60 + em) - (sh * 60 + sm);
  }

  function formatUren(ms) {
    const totaalMin = Math.floor(ms / 60000);
    const h = Math.floor(totaalMin / 60);
    const m = totaalMin % 60;
    return m > 0 ? `${h}u ${m}m` : `${h}u`;
  }

  function formatUrenVanMin(min) {
    if (min === null) return '?';
    const h = Math.floor(min / 60);
    const m = min % 60;
    return m > 0 ? `${h}u ${m}m` : `${h}u`;
  }

  function tijdStr(isoStr) {
    return new Date(isoStr).toLocaleTimeString('nl-NL', { hour: '2-digit', minute: '2-digit', hour12: false });
  }

  function avatarKleur(id) {
    const kleuren = ['#dc2626','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
    return kleuren[(id ?? 0) % kleuren.length];
  }

  function statusKleur(verschilMin) {
    if (verschilMin === null) return 'neutraal';
    if (verschilMin >= 0) return 'ok';
    if (verschilMin >= -30) return 'bijna';
    return 'tekort';
  }

  $: weekData = medewerkers.map(med => {
    const maandag = gekozenWeek;
    const zondag = weekZondag(maandag);

    const weekKlokslagen = klokslagen.filter(k => {
      const d = new Date(k.ingeklokt_op);
      return k.medewerker_id === med.id && d >= maandag && d <= zondag;
    });

    const weekRooster = rooster.filter(r => {
      const d = new Date(r.datum + 'T00:00:00');
      return r.medewerker_id === med.id && d >= maandag && d <= zondag;
    });

    const weekBeschikbaar = beschikbaarheid.filter(b => {
      const d = new Date(b.datum + 'T00:00:00');
      return b.medewerker_id === med.id && d >= maandag && d <= zondag;
    });

    const gewerktTotaalMs = weekKlokslagen.reduce((acc, k) => acc + gewerktMs(k), 0);
    const ingeplandTotaalMin = weekRooster.reduce((acc, r) => {
      const min = ingeplandMinuten(r);
      return min !== null ? acc + min : acc;
    }, 0);

    const heeftOpenKlokslag = weekKlokslagen.some(k => !k.uitgeklokt_op);
    const gewerktMin = Math.floor(gewerktTotaalMs / 60000);
    const verschilMin = ingeplandTotaalMin > 0 ? gewerktMin - ingeplandTotaalMin : null;

    const dagen = [];
    for (let i = 0; i < 7; i++) {
      const dag = new Date(maandag);
      dag.setDate(dag.getDate() + i);
      const ds = datumStr(dag);
      const dagKlokslagen = weekKlokslagen.filter(k => k.ingeklokt_op.startsWith(ds));
      const dagRooster = weekRooster.filter(r => r.datum === ds);
      const dagGewerktMs = dagKlokslagen.reduce((acc, k) => acc + gewerktMs(k), 0);
      const dagIngeplandMin = dagRooster.reduce((acc, r) => {
        const min = ingeplandMinuten(r);
        return min !== null ? acc + min : acc;
      }, 0);
      const dagVerschil = dagIngeplandMin > 0 ? Math.floor(dagGewerktMs / 60000) - dagIngeplandMin : null;
      const dagBeschikbaar = weekBeschikbaar.find(b => b.datum === ds) ?? null;
      if (dagKlokslagen.length > 0 || dagRooster.length > 0 || dagBeschikbaar) {
        dagen.push({ ds, dag, dagKlokslagen, dagRooster, dagGewerktMs, dagIngeplandMin, dagVerschil, dagBeschikbaar });
      }
    }

    return { med, gewerktTotaalMs, ingeplandTotaalMin, verschilMin, heeftOpenKlokslag, dagen, weekKlokslagen, weekRooster, weekBeschikbaar };
  }).filter(r => r.weekKlokslagen.length > 0 || r.weekRooster.length > 0 || r.weekBeschikbaar.length > 0);

  function exporteerExcel() {
    const maandag = gekozenWeek;
    const zondag = weekZondag(maandag);
    const weekLabel = formatWeek(maandag);
    const regels = [];
    regels.push(`Urenoverzicht week: ${weekLabel}`);
    regels.push('');
    regels.push('Medewerker;Datum;Dag;Ingepland van;Ingepland tot;Ingepland (uren);Ingeklokt;Uitgeklokt;Gewerkt (uren);Verschil (min)');
    weekData.forEach(({ med, dagen }) => {
      if (dagen.length === 0) {
        regels.push(`${med.naam};;;;;;;;; `);
      } else {
        dagen.forEach(({ ds, dag, dagKlokslagen, dagRooster, dagGewerktMs, dagIngeplandMin, dagVerschil }) => {
          const dagNaam = dag.toLocaleDateString('nl-NL', { weekday: 'long' });
          const ingeplandVan = dagRooster.map(r => r.start_tijd).join(', ') || '';
          const ingeplandTot = dagRooster.map(r => r.eind_tijd ?? '?').join(', ') || '';
          const ingeplandUren = dagIngeplandMin > 0 ? (dagIngeplandMin / 60).toFixed(2).replace('.', ',') : '';
          const ingeklokt = dagKlokslagen.map(k => tijdStr(k.ingeklokt_op)).join(', ');
          const uitgeklokt = dagKlokslagen.map(k => k.uitgeklokt_op ? tijdStr(k.uitgeklokt_op) : '...').join(', ');
          const gewerktUren = dagGewerktMs > 0 ? (dagGewerktMs / 3600000).toFixed(2).replace('.', ',') : '';
          const verschil = dagVerschil !== null ? dagVerschil : '';
          regels.push(`${med.naam};${ds};${dagNaam};${ingeplandVan};${ingeplandTot};${ingeplandUren};${ingeklokt};${uitgeklokt};${gewerktUren};${verschil}`);
        });
      }
    });
    regels.push('');
    regels.push('TOTALEN');
    regels.push('Medewerker;Ingepland (uren);Gewerkt (uren);Verschil (uren)');
    weekData.forEach(({ med, gewerktTotaalMs, ingeplandTotaalMin, verschilMin }) => {
      const ingepland = ingeplandTotaalMin > 0 ? (ingeplandTotaalMin / 60).toFixed(2).replace('.', ',') : '0';
      const gewerkt = (gewerktTotaalMs / 3600000).toFixed(2).replace('.', ',');
      const verschil = verschilMin !== null ? (verschilMin / 60).toFixed(2).replace('.', ',') : '';
      regels.push(`${med.naam};${ingepland};${gewerkt};${verschil}`);
    });
    const csvInhoud = '﻿' + regels.join('\n');
    const blob = new Blob([csvInhoud], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `kolkie-uren-${datumStr(maandag)}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

{#if nuAanwezig.length > 0}
  <div class="nu-aanwezig-banner kaart">
    <div class="nu-aanwezig-titel">
      <div class="live-dot"></div>
      <span>Nu aan het werk</span>
      <span class="na-count">{nuAanwezig.length}</span>
    </div>
    <div class="na-personen">
      {#each nuAanwezig as p}
        <div class="na-persoon" class:op-pauze={!!p.pauze_start}>
          <div class="na-avatar" style="background: {avatarKleur(p.medewerker_id)}">{p.naam[0]}</div>
          <div class="na-tekst">
            <span class="na-naam">{p.naam}</span>
            <span class="na-sub">{p.pauze_start ? '☕ Pauze' : `Sinds ${tijdStr(p.ingeklokt_op)}`}</span>
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<div class="pagina-header">
  <h1>Urenoverzicht</h1>
  <button class="export-knop" on:click={exporteerExcel}>
    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="7 10 12 15 17 10"/>
      <line x1="12" y1="15" x2="12" y2="3"/>
    </svg>
    Exporteer Excel
  </button>
</div>

<!-- Week navigator -->
<div class="week-nav kaart">
  <button class="nav-pijl" on:click={vorigeWeek}>
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><polyline points="15 18 9 12 15 6"/></svg>
  </button>
  <div class="nav-midden">
    <div class="week-header-rij">
      {#if isHuidigeWeek}<span class="huidige-week-chip">Deze week</span>{/if}
      <span class="week-label-tekst" class:huidig={isHuidigeWeek}>{formatWeek(gekozenWeek)}</span>
    </div>
    {#if !isHuidigeWeek}
      <button class="terug-link" on:click={() => gekozenWeek = huidigeMaandag()}>→ Naar deze week</button>
    {/if}
  </div>
  <button class="nav-pijl" on:click={volgendeWeek}>
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><polyline points="9 18 15 12 9 6"/></svg>
  </button>
</div>

{#if laden}
  <div class="laad-staat">
    <div class="laad-spinner"></div>
    <span>Gegevens laden...</span>
  </div>
{:else if weekData.length === 0}
  <div class="leeg-staat">
    <div class="leeg-icoon">📊</div>
    <div class="leeg-titel">Geen data voor deze week</div>
    <div class="leeg-sub">Er zijn geen klokslagen of geplande shifts in deze periode.</div>
  </div>
{:else}
  <div class="overzicht-lijst">
    {#each weekData as { med, gewerktTotaalMs, ingeplandTotaalMin, verschilMin, heeftOpenKlokslag, dagen }}
      <div class="medewerker-kaart kaart">
        <!-- Header -->
        <div class="med-header">
          <div class="med-avatar" style="background: {avatarKleur(med.id)}">{med.naam[0]}</div>
          <div class="med-info">
            <span class="med-naam">{med.naam}</span>
            <span class="med-rol-label">{med.rol === 'manager' ? '⭐ Manager' : '👤 Werknemer'}</span>
          </div>
          <div class="uren-stats">
            <div class="stat-blok">
              <span class="stat-label">Gewerkt</span>
              <span class="stat-waarde">{gewerktTotaalMs > 0 ? formatUren(gewerktTotaalMs) : '—'}</span>
            </div>
            {#if ingeplandTotaalMin > 0}
              <div class="stat-scheidslijn"></div>
              <div class="stat-blok">
                <span class="stat-label">Ingepland</span>
                <span class="stat-waarde">{formatUrenVanMin(ingeplandTotaalMin)}</span>
              </div>
              <div class="verschil-badge {statusKleur(verschilMin)}">
                {#if verschilMin === null}?
                {:else if verschilMin >= 0}+{formatUren(verschilMin * 60000)}
                {:else}-{formatUren(Math.abs(verschilMin) * 60000)}
                {/if}
              </div>
            {/if}
            {#if heeftOpenKlokslag}
              <span class="bezig-chip">● Bezig</span>
            {/if}
          </div>
        </div>

        <!-- Dag tabel -->
        <div class="dag-tabel">
          <div class="tabel-header">
            <span>Dag</span>
            <span>Ingepland</span>
            <span>Gewerkt</span>
            <span class="rechts">Verschil</span>
          </div>
          {#each dagen as { ds, dag, dagKlokslagen, dagRooster, dagGewerktMs, dagIngeplandMin, dagVerschil, dagBeschikbaar }}
            <div class="tabel-rij" class:rood={dagVerschil !== null && dagVerschil < -30} class:groen={dagVerschil !== null && dagVerschil >= 0}>
              <span class="dag-naam-cel">{dag.toLocaleDateString('nl-NL', { weekday: 'short', day: 'numeric', month: 'short' })}</span>
              <div class="dag-cel">
                {#if dagRooster.length > 0}
                  {#each dagRooster as r}
                    <span class="tijd-chip blauw">{r.start_tijd}–{r.eind_tijd ?? '?'}</span>
                  {/each}
                {/if}
                {#if dagBeschikbaar}
                  <span class="tijd-chip beschikbaar">✓ {dagBeschikbaar.hele_dag ? 'Hele dag' : `${dagBeschikbaar.van_tijd}–${dagBeschikbaar.tot_tijd}`}</span>
                {/if}
                {#if dagRooster.length === 0 && !dagBeschikbaar}
                  <span class="geen-data">—</span>
                {/if}
              </div>
              <div class="dag-cel">
                {#if dagKlokslagen.length > 0}
                  {#each dagKlokslagen as k}
                    <span class="tijd-chip groen-chip">{tijdStr(k.ingeklokt_op)}–{k.uitgeklokt_op ? tijdStr(k.uitgeklokt_op) : '...'}</span>
                  {/each}
                {:else}
                  <span class="geen-data">—</span>
                {/if}
              </div>
              <div class="dag-cel rechts">
                {#if dagVerschil === null}
                  <span class="geen-data">—</span>
                {:else if dagVerschil >= 0}
                  <span class="verschil-chip ok">+{formatUrenVanMin(dagVerschil)}</span>
                {:else if dagVerschil >= -30}
                  <span class="verschil-chip bijna">{formatUrenVanMin(dagVerschil)}</span>
                {:else}
                  <span class="verschil-chip tekort">{formatUrenVanMin(dagVerschil)}</span>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  </div>
{/if}

<style>
  /* Nu aanwezig banner */
  .nu-aanwezig-banner {
    margin-bottom: 1.5rem;
    padding: 1rem 1.25rem;
  }

  .nu-aanwezig-titel {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    font-size: 0.78rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.07em;
    color: var(--tekst-zacht);
    margin-bottom: 0.85rem;
  }

  .live-dot {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background: var(--groen);
    box-shadow: 0 0 0 3px rgba(22,163,74,0.2);
    flex-shrink: 0;
    animation: puls-dot 2s ease-out infinite;
  }

  @keyframes puls-dot {
    0%, 100% { box-shadow: 0 0 0 3px rgba(22,163,74,0.2); }
    50% { box-shadow: 0 0 0 6px rgba(22,163,74,0.08); }
  }

  .na-count {
    background: var(--groen);
    color: white;
    font-size: 0.65rem;
    font-weight: 900;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .na-personen {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
  }

  .na-persoon {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: var(--groen-licht);
    border: 1.5px solid var(--groen-rand);
    border-radius: 10px;
    padding: 0.45rem 0.75rem 0.45rem 0.45rem;
  }

  .na-persoon.op-pauze {
    background: var(--goud-licht);
    border-color: #fde68a;
  }

  .na-avatar {
    width: 30px;
    height: 30px;
    border-radius: 8px;
    color: white;
    font-size: 0.88rem;
    font-weight: 900;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
  }

  .na-tekst { display: flex; flex-direction: column; line-height: 1.2; }
  .na-naam { font-weight: 800; font-size: 0.82rem; color: var(--donker); }
  .na-sub { font-size: 0.68rem; color: var(--tekst-zacht); }

  .pagina-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .pagina-header h1 { margin-bottom: 0; }

  .export-knop {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: linear-gradient(135deg, #166534, #14532d);
    color: white;
    border: none;
    border-radius: var(--radius);
    padding: 0.65rem 1.25rem;
    font-family: var(--font-body);
    font-weight: 800;
    font-size: 0.88rem;
    cursor: pointer;
    transition: all var(--transition);
    box-shadow: 0 2px 8px rgba(22,101,52,0.3);
    white-space: nowrap;
  }

  .export-knop:hover { transform: translateY(-1px); box-shadow: 0 4px 14px rgba(22,101,52,0.4); }

  /* Week navigator */
  .week-nav {
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

  .nav-midden { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 0.25rem; }

  .week-header-rij { display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap; justify-content: center; }

  .huidige-week-chip {
    background: var(--rood);
    color: white;
    font-size: 0.65rem;
    font-weight: 800;
    padding: 0.15rem 0.55rem;
    border-radius: 20px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    white-space: nowrap;
  }

  .week-label-tekst {
    font-family: var(--font-display);
    font-size: 1.25rem;
    letter-spacing: 0.04em;
    color: var(--donker);
    text-align: center;
  }

  .week-label-tekst.huidig { color: var(--rood); }

  .terug-link {
    background: none;
    border: none;
    color: var(--tekst-zacht);
    font-size: 0.78rem;
    font-family: var(--font-body);
    cursor: pointer;
    font-weight: 600;
    padding: 0;
    transition: color var(--transition);
  }

  .terug-link:hover { color: var(--rood); }

  /* States */
  .laad-staat {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 4rem;
    color: var(--tekst-zacht);
    font-weight: 600;
  }

  .laad-spinner {
    width: 22px; height: 22px;
    border: 2.5px solid var(--rand);
    border-top-color: var(--rood);
    border-radius: 50%;
    animation: draaien 0.7s linear infinite;
  }

  @keyframes draaien { to { transform: rotate(360deg); } }

  .leeg-staat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 4rem 2rem;
    text-align: center;
    background: var(--wit);
    border-radius: var(--radius-lg);
    border: 1.5px dashed var(--rand);
  }

  .leeg-icoon { font-size: 2.5rem; }
  .leeg-titel { font-weight: 800; color: var(--donker); font-size: 1rem; }
  .leeg-sub { font-size: 0.85rem; color: var(--tekst-zacht); max-width: 300px; }

  /* Overzicht */
  .overzicht-lijst { display: flex; flex-direction: column; gap: 1rem; }

  .medewerker-kaart { padding: 1.25rem; }

  .med-header {
    display: flex;
    align-items: center;
    gap: 0.9rem;
    flex-wrap: wrap;
    margin-bottom: 1rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--rand);
  }

  .med-avatar {
    width: 44px; height: 44px;
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

  .med-info { display: flex; flex-direction: column; gap: 0.15rem; flex: 1; min-width: 0; }
  .med-naam { font-weight: 800; font-size: 1rem; color: var(--donker); }
  .med-rol-label { font-size: 0.75rem; color: var(--tekst-zacht); font-weight: 600; }

  .uren-stats {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
    margin-left: auto;
  }

  .stat-blok { display: flex; flex-direction: column; align-items: center; min-width: 60px; }
  .stat-label { font-size: 0.62rem; text-transform: uppercase; letter-spacing: 0.06em; color: var(--tekst-zacht); font-weight: 800; }
  .stat-waarde { font-family: var(--font-display); font-size: 1.1rem; letter-spacing: 0.03em; color: var(--donker); }

  .stat-scheidslijn {
    width: 1px;
    height: 30px;
    background: var(--rand);
    flex-shrink: 0;
  }

  .verschil-badge {
    padding: 0.3rem 0.75rem;
    border-radius: 20px;
    font-size: 0.82rem;
    font-weight: 800;
    white-space: nowrap;
  }

  .verschil-badge.ok { background: var(--groen-licht); color: var(--groen); border: 1.5px solid var(--groen-rand); }
  .verschil-badge.bijna { background: var(--goud-licht); color: var(--goud); border: 1.5px solid #fde68a; }
  .verschil-badge.tekort { background: var(--rood-licht); color: var(--rood-donker); border: 1.5px solid #fca5a5; }
  .verschil-badge.neutraal { background: var(--rand-licht); color: var(--tekst-zacht); border: 1.5px solid var(--rand); }

  .bezig-chip {
    background: var(--groen-licht);
    color: var(--groen);
    border: 1.5px solid var(--groen-rand);
    padding: 0.25rem 0.65rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 800;
    white-space: nowrap;
  }

  /* Dag tabel */
  .dag-tabel { display: flex; flex-direction: column; border-radius: 10px; overflow: hidden; border: 1px solid var(--rand); }

  .tabel-header {
    display: grid;
    grid-template-columns: 150px 1fr 1fr 90px;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: var(--rand-licht);
    font-size: 0.7rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--tekst-zacht);
    border-bottom: 1px solid var(--rand);
  }

  .tabel-rij {
    display: grid;
    grid-template-columns: 150px 1fr 1fr 90px;
    gap: 0.5rem;
    padding: 0.6rem 0.75rem;
    align-items: center;
    border-bottom: 1px solid var(--rand);
    transition: background var(--transition);
  }

  .tabel-rij:last-child { border-bottom: none; }
  .tabel-rij:nth-child(even) { background: #fafaf8; }
  .tabel-rij.rood { background: #fff5f5; }
  .tabel-rij.groen { background: #f7fef9; }

  .dag-naam-cel { font-size: 0.8rem; font-weight: 700; color: var(--tekst-zacht); }
  .dag-cel { display: flex; flex-direction: column; gap: 0.2rem; }
  .rechts { text-align: right; justify-content: flex-end; align-items: flex-end; }
  .geen-data { color: #d1d5db; font-size: 0.85rem; }

  .tijd-chip {
    font-size: 0.75rem;
    font-weight: 700;
    padding: 0.15rem 0.45rem;
    border-radius: 5px;
    width: fit-content;
  }

  .blauw { background: var(--blauw-licht); color: var(--blauw); }
  .groen-chip { background: var(--groen-licht); color: var(--groen); }
  .beschikbaar { background: #ecfdf5; color: #059669; border: 1px solid #6ee7b7; font-style: italic; }

  .verschil-chip {
    font-size: 0.75rem;
    font-weight: 800;
    padding: 0.15rem 0.55rem;
    border-radius: 5px;
  }

  .verschil-chip.ok { background: var(--groen-licht); color: var(--groen); }
  .verschil-chip.bijna { background: var(--goud-licht); color: var(--goud); }
  .verschil-chip.tekort { background: var(--rood-licht); color: var(--rood-donker); }

  @media (max-width: 640px) {
    .tabel-header, .tabel-rij { grid-template-columns: 80px 1fr 1fr 60px; font-size: 0.7rem; }
    .uren-stats { width: 100%; justify-content: flex-start; }
  }
</style>
