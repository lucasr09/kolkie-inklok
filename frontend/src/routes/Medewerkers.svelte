<script>
  import { onMount } from 'svelte';
  import { getMedewerkers, verwijderMedewerker } from '../lib/api.js';
  import { isManager } from '../stores.js';

  let medewerkers = [];
  let bevestigVerwijder = null;

  const kleuren = ['#e01f1f','#2563eb','#059669','#d97706','#7c3aed','#db2777'];
  function avatarKleur(id) { return kleuren[id % kleuren.length]; }

  onMount(async () => {
    medewerkers = await getMedewerkers();
  });

  async function verwijder(id) {
    await verwijderMedewerker(id);
    medewerkers = await getMedewerkers();
    bevestigVerwijder = null;
  }
</script>

{#if !$isManager}
  <div class="geen-toegang">🔒 Alleen managers kunnen het team beheren.</div>
{:else}
  <h1>Team</h1>

  <div class="team-grid">
    {#each medewerkers as m}
      <div class="team-kaart kaart">
        <div class="team-avatar" style="background: {avatarKleur(m.id ?? 0)}">{m.naam[0].toUpperCase()}</div>
        <div class="team-info">
          <span class="team-naam">{m.naam}</span>
          <span class="team-rol" class:manager={m.rol === 'manager'}>
            {m.rol === 'manager' ? '⭐ Manager' : '👤 Werknemer'}
          </span>
        </div>
        {#if bevestigVerwijder === m.id}
          <div class="bevestig">
            <span>Zeker weten?</span>
            <button class="ja-knop" on:click={() => verwijder(m.id)}>Ja</button>
            <button class="nee-knop" on:click={() => bevestigVerwijder = null}>Nee</button>
          </div>
        {:else}
          <button class="verwijder-knop" on:click={() => bevestigVerwijder = m.id} title="Verwijderen">✕</button>
        {/if}
      </div>
    {/each}
  </div>

  <div class="info-blok">
    💡 Nieuwe medewerkers voegen zichzelf toe via <strong>Account aanmaken</strong> op de inlogpagina.
  </div>
{/if}

<style>
  .geen-toegang { padding: 2rem; text-align: center; color: var(--tekst-zacht); font-weight: 600; }

  .team-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 2rem; }

  .team-kaart { display: flex; align-items: center; gap: 0.9rem; padding: 0.9rem 1.1rem; }

  .team-avatar { width: 44px; height: 44px; border-radius: 50%; color: white; font-size: 1.2rem; font-weight: 800; display: flex; align-items: center; justify-content: center; font-family: var(--font-display); flex-shrink: 0; }

  .team-info { display: flex; flex-direction: column; gap: 0.2rem; flex: 1; overflow: hidden; }
  .team-naam { font-weight: 700; font-size: 0.95rem; color: var(--donker); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .team-rol { font-size: 0.78rem; color: var(--tekst-zacht); background: #f3f4f6; padding: 0.15rem 0.45rem; border-radius: 4px; width: fit-content; font-weight: 600; }
  .team-rol.manager { background: var(--geel-licht); color: #92400e; }

  .verwijder-knop { background: none; border: none; cursor: pointer; color: #d1d5db; font-size: 1rem; padding: 0.3rem; border-radius: 4px; transition: color 0.15s; margin-left: auto; flex-shrink: 0; }
  .verwijder-knop:hover { color: var(--rood); }

  .bevestig { display: flex; align-items: center; gap: 0.4rem; margin-left: auto; flex-shrink: 0; }
  .bevestig span { font-size: 0.8rem; color: var(--tekst-zacht); white-space: nowrap; }
  .ja-knop { background: var(--rood); color: white; border: none; border-radius: 6px; padding: 0.3rem 0.6rem; font-size: 0.8rem; font-weight: 700; cursor: pointer; }
  .nee-knop { background: #f3f4f6; color: var(--tekst); border: none; border-radius: 6px; padding: 0.3rem 0.6rem; font-size: 0.8rem; font-weight: 700; cursor: pointer; }

  .info-blok { background: var(--geel-licht); border: 1.5px solid #fde68a; border-radius: var(--radius); padding: 0.9rem 1.1rem; font-size: 0.9rem; color: #78350f; }

  @media (max-width: 480px) { .team-grid { grid-template-columns: 1fr; } }
</style>
