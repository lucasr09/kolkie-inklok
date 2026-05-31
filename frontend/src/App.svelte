<script>
  import { sessie, isManager } from './stores.js';
  import Login from './routes/Login.svelte';
  import Klokken from './routes/Klokken.svelte';
  import Rooster from './routes/Rooster.svelte';
  import Medewerkers from './routes/Medewerkers.svelte';
  import Uren from './routes/Uren.svelte';
  import Overzicht from './routes/Overzicht.svelte';

  let activePagina = 'klokken';

  function uitloggen() {
    sessie.set(null);
    activePagina = 'klokken';
  }
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Nunito:wght@400;600;700;800&display=swap" rel="stylesheet">
</svelte:head>

{#if !$sessie}
  <Login />
{:else}
  <div class="app-shell">
    <header>
      <div class="header-inner">
        <div class="logo">
          <span class="logo-icon">🍟</span>
          <div class="logo-tekst">
            <span class="logo-naam">Kolkie</span>
            <span class="logo-sub">Cafetaria</span>
          </div>
        </div>
        <nav>
          <button class:actief={activePagina === 'klokken'} on:click={() => activePagina = 'klokken'}>
            <span class="nav-icon">⏱</span>
            <span class="nav-label">Inklokken</span>
          </button>
          <button class:actief={activePagina === 'rooster'} on:click={() => activePagina = 'rooster'}>
            <span class="nav-icon">📅</span>
            <span class="nav-label">Rooster</span>
          </button>
          {#if $isManager}
            <button class:actief={activePagina === 'overzicht'} on:click={() => activePagina = 'overzicht'}>
              <span class="nav-icon">📊</span>
              <span class="nav-label">Overzicht</span>
            </button>
            <button class:actief={activePagina === 'uren'} on:click={() => activePagina = 'uren'}>
              <span class="nav-icon">🕐</span>
              <span class="nav-label">Uren</span>
            </button>
            <button class:actief={activePagina === 'medewerkers'} on:click={() => activePagina = 'medewerkers'}>
              <span class="nav-icon">👥</span>
              <span class="nav-label">Team</span>
            </button>
          {/if}
        </nav>
        <div class="header-rechts">
          <div class="gebruiker-info">
            <div class="gebruiker-avatar">{$sessie.gebruikersnaam[0].toUpperCase()}</div>
            <div class="gebruiker-tekst">
              <span class="gebruiker-naam">{$sessie.gebruikersnaam}</span>
              <span class="gebruiker-rol">{$sessie.rol === 'manager' ? '⭐ Manager' : '👤 Werknemer'}</span>
            </div>
          </div>
          <button class="uitlog-knop" on:click={uitloggen} title="Uitloggen">↩</button>
        </div>
      </div>
    </header>

    <main>
      {#if activePagina === 'klokken'}
        <Klokken />
      {:else if activePagina === 'rooster'}
        <Rooster />
      {:else if activePagina === 'overzicht' && $isManager}
        <Overzicht />
      {:else if activePagina === 'uren' && $isManager}
        <Uren />
      {:else if activePagina === 'medewerkers' && $isManager}
        <Medewerkers />
      {/if}
    </main>

    <footer><span>© Cafetaria Kolkie</span></footer>
  </div>
{/if}

<style>
  :global(*) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(:root) {
    --rood: #e01f1f; --rood-donker: #b81515; --rood-licht: #fdecea;
    --geel: #f5c518; --geel-licht: #fffbea;
    --donker: #1a1208; --donker-2: #2e2210;
    --tekst: #2d2d2d; --tekst-zacht: #7a6f60;
    --rand: #e8e0d0; --achtergrond: #fdf8f2; --wit: #ffffff;
    --radius: 12px;
    --schaduw: 0 2px 12px rgba(0,0,0,0.08);
    --schaduw-groot: 0 4px 24px rgba(0,0,0,0.12);
    --font-display: 'Bebas Neue', sans-serif;
    --font-body: 'Nunito', sans-serif;
  }
  :global(body) { font-family: var(--font-body); background: var(--achtergrond); color: var(--tekst); min-height: 100vh; }
  :global(h1) { font-family: var(--font-display); font-size: 2.2rem; letter-spacing: 0.04em; color: var(--donker); margin-bottom: 1.5rem; }
  :global(h2) { font-family: var(--font-display); font-size: 1.4rem; letter-spacing: 0.03em; color: var(--donker); margin-top: 2rem; margin-bottom: 1rem; }
  :global(input), :global(select) { font-family: var(--font-body); padding: 0.7rem 1rem; border-radius: var(--radius); border: 2px solid var(--rand); font-size: 0.95rem; background: var(--wit); color: var(--tekst); transition: border-color 0.2s, box-shadow 0.2s; width: 100%; }
  :global(input:focus), :global(select:focus) { outline: none; border-color: var(--rood); box-shadow: 0 0 0 3px rgba(224,31,31,0.12); }
  :global(.knop-primair) { font-family: var(--font-body); font-weight: 800; padding: 0.75rem 1.5rem; border-radius: var(--radius); border: none; cursor: pointer; background: var(--rood); color: white; font-size: 1rem; transition: background 0.15s, transform 0.1s; }
  :global(.knop-primair:hover) { background: var(--rood-donker); transform: translateY(-1px); }
  :global(.kaart) { background: var(--wit); border-radius: var(--radius); border: 1.5px solid var(--rand); padding: 1rem 1.25rem; box-shadow: var(--schaduw); }

  .app-shell { display: flex; flex-direction: column; min-height: 100vh; }

  header { background: var(--donker); color: white; position: sticky; top: 0; z-index: 100; box-shadow: 0 2px 16px rgba(0,0,0,0.25); }
  .header-inner { max-width: 960px; margin: 0 auto; padding: 0 1.5rem; display: flex; align-items: center; height: 64px; gap: 1rem; }

  .logo { display: flex; align-items: center; gap: 0.6rem; }
  .logo-icon { font-size: 1.8rem; line-height: 1; }
  .logo-tekst { display: flex; flex-direction: column; line-height: 1; }
  .logo-naam { font-family: var(--font-display); font-size: 1.6rem; letter-spacing: 0.06em; color: var(--geel); }
  .logo-sub { font-size: 0.65rem; letter-spacing: 0.15em; text-transform: uppercase; color: rgba(255,255,255,0.45); margin-top: 1px; }

  nav { display: flex; gap: 0.25rem; flex: 1; justify-content: center; }
  nav button { background: none; border: none; color: rgba(255,255,255,0.55); cursor: pointer; padding: 0.5rem 0.75rem; border-radius: 8px; font-family: var(--font-body); font-size: 0.85rem; font-weight: 700; display: flex; flex-direction: column; align-items: center; gap: 2px; transition: color 0.15s, background 0.15s; white-space: nowrap; }
  nav button .nav-icon { font-size: 1.1rem; }
  nav button .nav-label { font-size: 0.72rem; letter-spacing: 0.05em; }
  nav button:hover { color: white; background: rgba(255,255,255,0.08); }
  nav button.actief { color: var(--geel); background: rgba(245,197,24,0.12); }

  .header-rechts { display: flex; align-items: center; gap: 0.75rem; margin-left: auto; }
  .gebruiker-info { display: flex; align-items: center; gap: 0.5rem; }
  .gebruiker-avatar { width: 34px; height: 34px; border-radius: 50%; background: var(--rood); color: white; font-size: 0.95rem; font-weight: 800; display: flex; align-items: center; justify-content: center; font-family: var(--font-display); flex-shrink: 0; }
  .gebruiker-tekst { display: flex; flex-direction: column; line-height: 1.2; }
  .gebruiker-naam { font-size: 0.8rem; font-weight: 700; color: white; }
  .gebruiker-rol { font-size: 0.65rem; color: rgba(255,255,255,0.5); }
  .uitlog-knop { background: rgba(255,255,255,0.08); border: 1px solid rgba(255,255,255,0.15); color: rgba(255,255,255,0.6); border-radius: 8px; padding: 0.4rem 0.6rem; cursor: pointer; font-size: 1rem; transition: all 0.15s; }
  .uitlog-knop:hover { background: rgba(255,255,255,0.15); color: white; }

  main { flex: 1; max-width: 960px; width: 100%; margin: 0 auto; padding: 2rem 1.5rem; }
  footer { text-align: center; padding: 1rem; font-size: 0.8rem; color: var(--tekst-zacht); border-top: 1px solid var(--rand); margin-top: auto; }

  @media (max-width: 600px) {
    .header-inner { padding: 0 1rem; height: 56px; }
    .logo-naam { font-size: 1.3rem; }
    .gebruiker-tekst { display: none; }
    main { padding: 1.25rem 1rem; }
  }
</style>
