<script>
  import { onMount } from 'svelte';
  import { getAlleKlokslagen, getRooster, getMedewerkers } from '../lib/api.js';

  let klokslagen = [];
  let rooster = [];
  let medewerkers = [];
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
    return d.toISOString().split('T')[0];
  }

  function formatWeek(maandag) {
    const zondag = weekZondag(maandag);
    return `${maandag.toLocaleDateString('nl-NL', { day: 'numeric', month: 'short' })} – ${zondag.toLocaleDateString('nl-NL', { day: 'numeric', month: 'short', year: 'numeric' })}`;
  }

  $: isHuidigeWeek = datumStr(gekozenWeek) === datumStr(huidigeMaandag());

  onMount(async () => {
    [klokslagen, rooster, medewerkers] = await Promise.all([
      getAlleKlokslagen(), getRooster(), getMedewerkers()
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
    return new Date(isoStr).toLocaleTimeString('nl-NL', { hour: '2-digit', minute: '2-digit' });
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

    const gewerktTotaalMs = weekKlokslagen.reduce((acc, k) => acc + gewerktMs(k), 0);
    const ingeplandTotaalMin = weekRooster.reduce((acc, r) => {
      const min = ingeplandMinuten(r);
      return min !== null ? acc + min : acc;
    }, 0);

    const heeftOpenKlokslag = weekKlokslagen.some(k => !k.uitgeklokt_op);
    const gewerktMin = Math.floor(gewerktTotaalMs / 60000);
    const verschilMin = ingeplandTotaalMin > 0 ? gewerktMin - ingeplandTotaalMin : null;

    // Per dag breakdown met vergelijking
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

      if (dagKlokslagen.length > 0 || dagRooster.length > 0) {
        dagen.push({ ds, dag, dagKlokslagen, dagRooster, dagGewerktMs, dagIngeplandMin, dagVerschil });
      }
    }

    return { med, gewerktTotaalMs, ingeplandTotaalMin, verschilMin, heeftOpenKlokslag, dagen, weekKlokslagen, weekRooster };
  }).filter(r => r.weekKlokslagen.length > 0 || r.weekRooster.length > 0);

  function avatarKleur(id) {
    const kleuren = ['#e01f1f','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
    return kleuren[id % kleuren.length];
  }

  function statusKleur(verschilMin) {
    if (verschilMin === null) return 'neutraal';
    if (verschilMin >= 0) return 'ok';
    if (verschilMin >= -30) return 'bijna';
    return 'tekort';
  }

  // ── Excel export ──
  function exporteerExcel() {
    const maandag = gekozenWeek;
    const zondag = weekZondag(maandag);
    const weekLabel = formatWeek(maandag);

    // CSV bouwen (opent in Excel)
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

    // Totaalregel per medewerker
    regels.push('');
    regels.push('TOTALEN PER MEDEWERKER');
    regels.push('Medewerker;Ingepland (uren);Gewerkt (uren);Verschil (uren)');
    weekData.forEach(({ med, gewerktTotaalMs, ingeplandTotaalMin, verschilMin }) => {
      const ingepland = ingeplandTotaalMin > 0 ? (ingeplandTotaalMin / 60).toFixed(2).replace('.', ',') : '0';
      const gewerkt = (gewerktTotaalMs / 3600000).toFixed(2).replace('.', ',');
      const verschil = verschilMin !== null ? (verschilMin / 60).toFixed(2).replace('.', ',') : '';
      regels.push(`${med.naam};${ingepland};${gewerkt};${verschil}`);
    });

    const csvInhoud = '\uFEFF' + regels.join('\n'); // BOM voor Excel
    const blob = new Blob([csvInhoud], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `kolkie-uren-${datumStr(maandag)}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="pagina-header">
  <h1>Urenoverzicht</h1>
  <button class="export-knop" on:click={exporteerExcel}>
    ⬇ Exporteer naar Excel
  </button>
</div>

<!-- Week navigator -->
<div class="week-navigator">
  <button class="nav-pijl" on:click={vorigeWeek}>‹</button>
  <div class="week-midden">
    <span class="week-label" class:huidig={isHuidigeWeek}>
      {#if isHuidigeWeek}<span class="huidig-badge">Deze week</span>{/if}
      {formatWeek(gekozenWeek)}
    </span>
    {#if !isHuidigeWeek}
      <button class="terug-knop" on:click={() => gekozenWeek = huidigeMaandag()}>Naar deze week</button>
    {/if}
  </div>
  <button class="nav-pijl" on:click={volgendeWeek}>›</button>
</div>

{#if laden}
  <div class="laden">Laden...</div>
{:else if weekData.length === 0}
  <div class="leeg">
    <span class="leeg-icoon">📊</span>
    <p>Geen data voor deze week</p>
  </div>
{:else}
  <div class="overzicht-lijst">
    {#each weekData as { med, gewerktTotaalMs, ingeplandTotaalMin, verschilMin, heeftOpenKlokslag, dagen }}
      <div class="medewerker-blok kaart">
        <!-- Header -->
        <div class="med-header">
          <div class="shift-avatar" style="background: {avatarKleur(med.id)}">{med.naam[0]}</div>
          <div class="med-info">
            <span class="med-naam">{med.naam}</span>
            <span class="med-rol">{med.rol === 'manager' ? '⭐ Manager' : '👤 Werknemer'}</span>
          </div>
          <div class="uren-samenvatting">
            <div class="uren-blok">
              <span class="uren-label">Gewerkt</span>
              <span class="uren-waarde">{gewerktTotaalMs > 0 ? formatUren(gewerktTotaalMs) : '—'}</span>
            </div>
            {#if ingeplandTotaalMin > 0}
              <div class="uren-blok">
                <span class="uren-label">Ingepland</span>
                <span class="uren-waarde">{formatUrenVanMin(ingeplandTotaalMin)}</span>
              </div>
              <div class="verschil-badge {statusKleur(verschilMin)}">
                {#if verschilMin === null}?
                {:else if verschilMin >= 0}+{formatUren(verschilMin * 60000)}
                {:else}-{formatUren(Math.abs(verschilMin) * 60000)}
                {/if}
              </div>
            {/if}
            {#if heeftOpenKlokslag}
              <span class="bezig-badge">Bezig</span>
            {/if}
          </div>
        </div>

        <!-- Per dag vergelijking -->
        <div class="dagen-tabel">
          <div class="tabel-header">
            <span>Dag</span>
            <span>Ingepland</span>
            <span>Gewerkt</span>
            <span>Verschil</span>
          </div>
          {#each dagen as { ds, dag, dagKlokslagen, dagRooster, dagGewerktMs, dagIngeplandMin, dagVerschil }}
            <div class="tabel-rij" class:rood={dagVerschil !== null && dagVerschil < -30} class:groen={dagVerschil !== null && dagVerschil >= 0}>
              <span class="dag-naam">{dag.toLocaleDateString('nl-NL', { weekday: 'short', day: 'numeric', month: 'short' })}</span>
              <div class="dag-cel">
                {#if dagRooster.length > 0}
                  {#each dagRooster as r}
                    <span class="tijd-chip ingepland">{r.start_tijd}–{r.eind_tijd ?? '?'}</span>
                  {/each}
                {:else}
                  <span class="geen">—</span>
                {/if}
              </div>
              <div class="dag-cel">
                {#if dagKlokslagen.length > 0}
                  {#each dagKlokslagen as k}
                    <span class="tijd-chip gewerkt">{tijdStr(k.ingeklokt_op)}–{k.uitgeklokt_op ? tijdStr(k.uitgeklokt_op) : '...'}</span>
                  {/each}
                {:else}
                  <span class="geen">—</span>
                {/if}
              </div>
              <div class="dag-cel verschil-cel">
                {#if dagVerschil === null}
                  <span class="geen">—</span>
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
  .pagina-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem; flex-wrap: wrap; gap: 1rem; }
  .pagina-header h1 { margin-bottom: 0; }

  .export-knop {
    background: #166534; color: white; border: none; border-radius: var(--radius);
    padding: 0.65rem 1.25rem; font-family: var(--font-body); font-weight: 700;
    font-size: 0.9rem; cursor: pointer; transition: background 0.15s, transform 0.1s;
    white-space: nowrap;
  }
  .export-knop:hover { background: #14532d; transform: translateY(-1px); }

  .week-navigator { display: flex; align-items: center; gap: 1rem; background: var(--wit); border: 1.5px solid var(--rand); border-radius: var(--radius); padding: 1rem 1.25rem; margin-bottom: 1.75rem; box-shadow: var(--schaduw); }
  .nav-pijl { background: none; border: 1.5px solid var(--rand); border-radius: 8px; width: 40px; height: 40px; font-size: 1.4rem; cursor: pointer; color: var(--tekst-zacht); display: flex; align-items: center; justify-content: center; transition: border-color 0.15s, color 0.15s, background 0.15s; flex-shrink: 0; line-height: 1; }
  .nav-pijl:hover { border-color: var(--rood); color: var(--rood); background: var(--rood-licht); }
  .week-midden { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 0.3rem; }
  .week-label { font-family: var(--font-display); font-size: 1.2rem; letter-spacing: 0.04em; color: var(--donker); display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; justify-content: center; text-align: center; }
  .week-label.huidig { color: var(--rood); }
  .huidig-badge { background: var(--rood); color: white; font-family: var(--font-body); font-size: 0.65rem; font-weight: 800; padding: 0.15rem 0.5rem; border-radius: 20px; text-transform: uppercase; letter-spacing: 0.06em; }
  .terug-knop { background: none; border: none; color: var(--tekst-zacht); font-size: 0.78rem; font-family: var(--font-body); cursor: pointer; text-decoration: underline; padding: 0; }
  .terug-knop:hover { color: var(--rood); }

  .laden, .leeg { display: flex; flex-direction: column; align-items: center; padding: 3rem; color: var(--tekst-zacht); gap: 0.5rem; }
  .leeg-icoon { font-size: 2.5rem; }
  .leeg p { font-weight: 600; }

  .overzicht-lijst { display: flex; flex-direction: column; gap: 1rem; }
  .medewerker-blok { padding: 1.25rem; }

  .med-header { display: flex; align-items: center; gap: 0.9rem; flex-wrap: wrap; margin-bottom: 1.25rem; padding-bottom: 1rem; border-bottom: 1.5px solid var(--rand); }
  .shift-avatar { width: 44px; height: 44px; border-radius: 50%; color: white; font-size: 1.2rem; font-weight: 800; display: flex; align-items: center; justify-content: center; font-family: var(--font-display); flex-shrink: 0; }
  .med-info { display: flex; flex-direction: column; gap: 0.15rem; flex: 1; }
  .med-naam { font-weight: 700; font-size: 1rem; color: var(--donker); }
  .med-rol { font-size: 0.75rem; color: var(--tekst-zacht); }
  .uren-samenvatting { display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; margin-left: auto; }
  .uren-blok { display: flex; flex-direction: column; align-items: center; }
  .uren-label { font-size: 0.65rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--tekst-zacht); font-weight: 700; }
  .uren-waarde { font-family: var(--font-display); font-size: 1.1rem; letter-spacing: 0.03em; color: var(--donker); }
  .verschil-badge { padding: 0.25rem 0.6rem; border-radius: 6px; font-size: 0.82rem; font-weight: 800; white-space: nowrap; }
  .verschil-badge.ok { background: #edfdf5; color: #166534; border: 1.5px solid #a7f3d0; }
  .verschil-badge.bijna { background: var(--geel-licht); color: #92400e; border: 1.5px solid #fde68a; }
  .verschil-badge.tekort { background: var(--rood-licht); color: var(--rood-donker); border: 1.5px solid #f5c6c6; }
  .verschil-badge.neutraal { background: #f3f4f6; color: var(--tekst-zacht); border: 1.5px solid var(--rand); }
  .bezig-badge { background: #edfdf5; color: #166534; border: 1.5px solid #a7f3d0; padding: 0.2rem 0.6rem; border-radius: 6px; font-size: 0.78rem; font-weight: 700; }

  /* Tabel */
  .dagen-tabel { display: flex; flex-direction: column; gap: 0; }
  .tabel-header { display: grid; grid-template-columns: 140px 1fr 1fr 100px; gap: 0.5rem; padding: 0.4rem 0.6rem; font-size: 0.72rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--tekst-zacht); border-bottom: 1.5px solid var(--rand); }
  .tabel-rij { display: grid; grid-template-columns: 140px 1fr 1fr 100px; gap: 0.5rem; padding: 0.6rem; border-radius: 6px; align-items: center; }
  .tabel-rij:nth-child(even) { background: #fafaf9; }
  .tabel-rij.rood { background: #fff5f5; }
  .tabel-rij.groen { background: #f0fdf4; }

  .dag-naam { font-size: 0.82rem; font-weight: 700; color: var(--tekst-zacht); }
  .dag-cel { display: flex; flex-direction: column; gap: 0.2rem; }
  .geen { color: #d1d5db; font-size: 0.85rem; }

  .tijd-chip { font-size: 0.78rem; font-weight: 600; padding: 0.15rem 0.4rem; border-radius: 4px; width: fit-content; }
  .tijd-chip.ingepland { background: #eff6ff; color: #1e40af; }
  .tijd-chip.gewerkt { background: #f0fdf4; color: #166534; }

  .verschil-cel { }
  .verschil-chip { font-size: 0.78rem; font-weight: 800; padding: 0.15rem 0.5rem; border-radius: 4px; }
  .verschil-chip.ok { background: #edfdf5; color: #166534; }
  .verschil-chip.bijna { background: var(--geel-licht); color: #92400e; }
  .verschil-chip.tekort { background: var(--rood-licht); color: var(--rood-donker); }

  @media (max-width: 600px) {
    .tabel-header, .tabel-rij { grid-template-columns: 90px 1fr 1fr 70px; font-size: 0.72rem; }
    .uren-samenvatting { width: 100%; justify-content: flex-start; }
  }
</style>
