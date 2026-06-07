<script>
  import { onMount } from 'svelte';
  import { getMedewerkers, verwijderMedewerker, updateMedewerkerRol } from '../lib/api.js';
  import { isManager } from '../stores.js';

  let medewerkers = [];
  let bevestigVerwijder = null;
  let laden = false;
  let succes = '';

  const kleuren = ['#e01f1f','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
  function avatarKleur(id) { return kleuren[id % kleuren.length]; }

  onMount(laadMedewerkers);

  async function laadMedewerkers() {
    medewerkers = await getMedewerkers();
  }

  async function verwijder(id) {
    await verwijderMedewerker(id);
    bevestigVerwijder = null;
    await laadMedewerkers();
    toonSucces('Medewerker verwijderd');
  }

  async function promoveer(id, nieuweRol) {
    laden = id;
    await updateMedewerkerRol(id, nieuweRol);
    await laadMedewerkers();
    laden = false;
    toonSucces(nieuweRol === 'manager' ? 'Gepromoveerd tot manager' : 'Teruggezet naar werknemer');
  }

  function toonSucces(bericht) {
    succes = bericht;
    setTimeout(() => succes = '', 3000);
  }

  $: managers = medewerkers.filter(m => m.rol === 'manager');
  $: werknemers = medewerkers.filter(m => m.rol === 'werknemer');
</script>

{#if !$isManager}
  <div class="geen-toegang">
    <div class="geen-toegang-icoon">🔒</div>
    <p>Alleen managers kunnen het team beheren.</p>
  </div>
{:else}
  <div class="pagina-header">
    <h1>Team beheer</h1>
    <div class="teller-badges">
      <span class="badge badge-manager">⭐ {managers.length} manager{managers.length !== 1 ? 's' : ''}</span>
      <span class="badge badge-werknemer">👤 {werknemers.length} werknemer{werknemers.length !== 1 ? 's' : ''}</span>
    </div>
  </div>

  {#if succes}
    <div class="succes-bericht">{succes}</div>
  {/if}

  {#if managers.length > 0}
    <h2>Managers</h2>
    <div class="team-grid">
      {#each managers as m}
        <div class="team-kaart kaart manager-kaart">
          <div class="team-avatar" style="background: {avatarKleur(m.id ?? 0)}">{m.naam[0].toUpperCase()}</div>
          <div class="team-info">
            <span class="team-naam">{m.naam}</span>
            <span class="team-rol manager-badge">⭐ Manager</span>
          </div>
          <div class="acties">
            {#if bevestigVerwijder === m.id}
              <div class="bevestig">
                <span>Verwijderen?</span>
                <button class="ja-knop" on:click={() => verwijder(m.id)}>Ja</button>
                <button class="nee-knop" on:click={() => bevestigVerwijder = null}>Nee</button>
              </div>
            {:else}
              <button
                class="rol-knop demoteer"
                on:click={() => promoveer(m.id, 'werknemer')}
                disabled={laden === m.id}
                title="Terugzetten naar werknemer"
              >
                {laden === m.id ? '...' : '↓ Werknemer'}
              </button>
              <button class="verwijder-knop" on:click={() => bevestigVerwijder = m.id} title="Verwijderen">✕</button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if werknemers.length > 0}
    <h2>Werknemers</h2>
    <div class="team-grid">
      {#each werknemers as m}
        <div class="team-kaart kaart">
          <div class="team-avatar" style="background: {avatarKleur(m.id ?? 0)}">{m.naam[0].toUpperCase()}</div>
          <div class="team-info">
            <span class="team-naam">{m.naam}</span>
            <span class="team-rol">👤 Werknemer</span>
          </div>
          <div class="acties">
            {#if bevestigVerwijder === m.id}
              <div class="bevestig">
                <span>Verwijderen?</span>
                <button class="ja-knop" on:click={() => verwijder(m.id)}>Ja</button>
                <button class="nee-knop" on:click={() => bevestigVerwijder = null}>Nee</button>
              </div>
            {:else}
              <button
                class="rol-knop promoveer"
                on:click={() => promoveer(m.id, 'manager')}
                disabled={laden === m.id}
                title="Promoveren tot manager"
              >
                {laden === m.id ? '...' : '↑ Manager'}
              </button>
              <button class="verwijder-knop" on:click={() => bevestigVerwijder = m.id} title="Verwijderen">✕</button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if medewerkers.length === 0}
    <div class="leeg">Geen medewerkers gevonden.</div>
  {/if}

  <div class="info-blok">
    Nieuwe medewerkers kunnen zichzelf registreren via <strong>Nieuw account</strong> op de inlogpagina. Gebruik de <strong>↑ Manager</strong> knop om iemand te promoveren.
  </div>
{/if}

<style>
  .geen-toegang {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    padding: 4rem 2rem;
    text-align: center;
    color: var(--tekst-zacht);
  }
  .geen-toegang-icoon { font-size: 3rem; }
  .geen-toegang p { font-weight: 600; font-size: 1.05rem; }

  .pagina-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
  }
  .pagina-header h1 { margin-bottom: 0; }

  .teller-badges { display: flex; gap: 0.5rem; flex-wrap: wrap; }

  .badge {
    padding: 0.3rem 0.8rem;
    border-radius: 20px;
    font-size: 0.78rem;
    font-weight: 700;
  }
  .badge-manager { background: #fffbea; color: #92400e; border: 1.5px solid #fde68a; }
  .badge-werknemer { background: #f0f4ff; color: #1e40af; border: 1.5px solid #bfdbfe; }

  .succes-bericht {
    background: #f0fdf4;
    color: #166534;
    border: 1.5px solid #86efac;
    border-radius: 10px;
    padding: 0.7rem 1rem;
    font-size: 0.9rem;
    font-weight: 600;
    margin-bottom: 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .succes-bericht::before { content: '✓'; font-weight: 800; }

  .team-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 0.85rem;
    margin-bottom: 1.5rem;
  }

  .team-kaart {
    display: flex;
    align-items: center;
    gap: 0.9rem;
    padding: 0.9rem 1rem;
    transition: box-shadow 0.15s, transform 0.15s;
  }
  .team-kaart:hover { box-shadow: 0 4px 20px rgba(0,0,0,0.1); transform: translateY(-1px); }

  .manager-kaart {
    border-color: #fde68a;
    background: linear-gradient(135deg, #fffbea 0%, #ffffff 100%);
  }

  .team-avatar {
    width: 46px;
    height: 46px;
    border-radius: 50%;
    color: white;
    font-size: 1.2rem;
    font-weight: 800;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(0,0,0,0.15);
  }

  .team-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
    overflow: hidden;
  }

  .team-naam {
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--donker);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .team-rol {
    font-size: 0.75rem;
    color: var(--tekst-zacht);
    background: #f3f4f6;
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
    width: fit-content;
    font-weight: 600;
  }

  .manager-badge {
    background: #fffbea;
    color: #92400e;
  }

  .acties {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    flex-shrink: 0;
  }

  .rol-knop {
    border: none;
    border-radius: 7px;
    padding: 0.3rem 0.65rem;
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.15s;
    white-space: nowrap;
    font-family: var(--font-body);
  }

  .promoveer {
    background: #fffbea;
    color: #92400e;
    border: 1.5px solid #fde68a;
  }
  .promoveer:hover:not(:disabled) {
    background: #fde68a;
    color: #78350f;
  }

  .demoteer {
    background: #f0f4ff;
    color: #1e40af;
    border: 1.5px solid #bfdbfe;
  }
  .demoteer:hover:not(:disabled) {
    background: #bfdbfe;
    color: #1e3a8a;
  }

  .rol-knop:disabled { opacity: 0.5; cursor: not-allowed; }

  .verwijder-knop {
    background: none;
    border: none;
    cursor: pointer;
    color: #d1d5db;
    font-size: 0.95rem;
    padding: 0.3rem 0.35rem;
    border-radius: 5px;
    transition: color 0.15s, background 0.15s;
    line-height: 1;
  }
  .verwijder-knop:hover { color: var(--rood); background: var(--rood-licht); }

  .bevestig {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    flex-shrink: 0;
  }
  .bevestig span { font-size: 0.75rem; color: var(--tekst-zacht); white-space: nowrap; }

  .ja-knop {
    background: var(--rood);
    color: white;
    border: none;
    border-radius: 6px;
    padding: 0.28rem 0.55rem;
    font-size: 0.78rem;
    font-weight: 700;
    cursor: pointer;
    font-family: var(--font-body);
  }
  .nee-knop {
    background: #f3f4f6;
    color: var(--tekst);
    border: none;
    border-radius: 6px;
    padding: 0.28rem 0.55rem;
    font-size: 0.78rem;
    font-weight: 700;
    cursor: pointer;
    font-family: var(--font-body);
  }

  .leeg {
    text-align: center;
    color: var(--tekst-zacht);
    padding: 2rem;
    font-weight: 600;
  }

  .info-blok {
    background: linear-gradient(135deg, #fffbea, #fef9e7);
    border: 1.5px solid #fde68a;
    border-radius: var(--radius);
    padding: 0.9rem 1.1rem;
    font-size: 0.88rem;
    color: #78350f;
    margin-top: 0.5rem;
    line-height: 1.5;
  }

  @media (max-width: 480px) {
    .team-grid { grid-template-columns: 1fr; }
    .pagina-header { flex-direction: column; align-items: flex-start; }
  }
</style>
