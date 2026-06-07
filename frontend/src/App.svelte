<script>
  import { sessie, isManager } from './stores.js';
  import { uitloggen as apiUitloggen, wijzigWachtwoord } from './lib/api.js';
  import Login from './routes/Login.svelte';
  import Klokken from './routes/Klokken.svelte';
  import Rooster from './routes/Rooster.svelte';
  import Medewerkers from './routes/Medewerkers.svelte';
  import Uren from './routes/Uren.svelte';
  import Overzicht from './routes/Overzicht.svelte';
  import Beschikbaarheid from './routes/Beschikbaarheid.svelte';

  let activePagina = 'klokken';
  let toonWachtwoordModal = false;
  let huidigWachtwoord = '';
  let nieuwWachtwoord = '';
  let wachtwoordFout = '';
  let wachtwoordSucces = false;
  let wachtwoordLaden = false;

  async function uitloggen() {
    await apiUitloggen();
    sessie.set(null);
    activePagina = 'klokken';
  }

  function openWachtwoordModal() {
    toonWachtwoordModal = true;
    huidigWachtwoord = '';
    nieuwWachtwoord = '';
    wachtwoordFout = '';
    wachtwoordSucces = false;
  }

  async function handleWachtwoordWijzigen() {
    if (!huidigWachtwoord || !nieuwWachtwoord) { wachtwoordFout = 'Vul beide velden in'; return; }
    wachtwoordLaden = true;
    wachtwoordFout = '';
    const res = await wijzigWachtwoord(huidigWachtwoord, nieuwWachtwoord);
    wachtwoordLaden = false;
    if (res.status === 'ok') {
      wachtwoordSucces = true;
      setTimeout(() => { toonWachtwoordModal = false; }, 1500);
    } else {
      wachtwoordFout = res.bericht || 'Wijzigen mislukt';
    }
  }

  const navItems = [
    { id: 'klokken',         label: 'Inklokken',    icon: '⏱',  managerOnly: false },
    { id: 'beschikbaarheid', label: 'Beschikbaar',  icon: '📋',  managerOnly: false },
    { id: 'rooster',         label: 'Rooster',      icon: '📅',  managerOnly: false },
    { id: 'overzicht',       label: 'Overzicht',    icon: '📊',  managerOnly: true  },
    { id: 'uren',            label: 'Uren',         icon: '🕐',  managerOnly: true  },
    { id: 'medewerkers',     label: 'Team',         icon: '👥',  managerOnly: true  },
  ];
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Nunito:wght@400;500;600;700;800;900&display=swap" rel="stylesheet">
</svelte:head>

{#if !$sessie}
  <Login />
{:else}
  <div class="app-shell">

    <header>
      <div class="header-inner">

        <div class="logo">
          <div class="logo-merk">
            <span class="logo-icon">🍟</span>
          </div>
          <div class="logo-tekst">
            <span class="logo-naam">Kolkie</span>
            <span class="logo-sub">Cafetaria</span>
          </div>
        </div>

        <nav>
          {#each navItems as item}
            {#if !item.managerOnly || $isManager}
              <button
                class="nav-knop"
                class:actief={activePagina === item.id}
                on:click={() => activePagina = item.id}
              >
                <span class="nav-icon">{item.icon}</span>
                <span class="nav-label">{item.label}</span>
              </button>
            {/if}
          {/each}
        </nav>

        <div class="header-rechts">
          <button class="gebruiker-chip" on:click={openWachtwoordModal} title="Wachtwoord wijzigen">
            <div class="gebruiker-avatar">{$sessie.gebruikersnaam[0].toUpperCase()}</div>
            <div class="gebruiker-info">
              <span class="gebruiker-naam">{$sessie.gebruikersnaam}</span>
              <span class="gebruiker-rol" class:manager={$sessie.rol === 'manager'}>
                {$sessie.rol === 'manager' ? '⭐ Manager' : '👤 Werknemer'}
              </span>
            </div>
          </button>
          <button class="uitlog-knop" on:click={uitloggen} title="Uitloggen">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
              <polyline points="16 17 21 12 16 7"/>
              <line x1="21" y1="12" x2="9" y2="12"/>
            </svg>
          </button>
        </div>

      </div>
    </header>

    <main>
      <div class="pagina-inhoud">
        {#if activePagina === 'klokken'}
          <Klokken />
        {:else if activePagina === 'beschikbaarheid'}
          <Beschikbaarheid />
        {:else if activePagina === 'rooster'}
          <Rooster />
        {:else if activePagina === 'overzicht' && $isManager}
          <Overzicht />
        {:else if activePagina === 'uren' && $isManager}
          <Uren />
        {:else if activePagina === 'medewerkers' && $isManager}
          <Medewerkers />
        {/if}
      </div>
    </main>

    <footer>
      <span>© {new Date().getFullYear()} Cafetaria Kolkie</span>
      <span class="footer-dot">·</span>
      <span>Personeelssysteem</span>
    </footer>

  </div>

  {#if toonWachtwoordModal}
    <div class="modal-overlay" on:click|self={() => toonWachtwoordModal = false}>
      <div class="modal-kaart">
        <div class="modal-header">
          <h2 class="modal-titel">Wachtwoord wijzigen</h2>
          <button class="modal-sluit" on:click={() => toonWachtwoordModal = false}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>

        {#if wachtwoordSucces}
          <div class="modal-succes">
            <span class="succes-icoon">✓</span>
            Wachtwoord succesvol gewijzigd!
          </div>
        {:else}
          <div class="modal-body">
            <label class="veld-label">
              Huidig wachtwoord
              <input type="password" bind:value={huidigWachtwoord} placeholder="••••••••" />
            </label>
            <label class="veld-label">
              Nieuw wachtwoord
              <input type="password" bind:value={nieuwWachtwoord} placeholder="Minimaal 6 tekens" />
            </label>
            {#if wachtwoordFout}
              <p class="fout-tekst">{wachtwoordFout}</p>
            {/if}
          </div>
          <div class="modal-footer">
            <button class="knop-annuleer" on:click={() => toonWachtwoordModal = false}>Annuleren</button>
            <button class="knop-primair" on:click={handleWachtwoordWijzigen} disabled={wachtwoordLaden}>
              {wachtwoordLaden ? 'Opslaan...' : 'Opslaan'}
            </button>
          </div>
        {/if}
      </div>
    </div>
  {/if}
{/if}

<style>
  /* ── Design tokens ── */
  :global(:root) {
    --rood:           #dc2626;
    --rood-donker:    #b91c1c;
    --rood-licht:     #fef2f2;
    --rood-glow:      rgba(220,38,38,0.18);

    --goud:           #d97706;
    --goud-helder:    #f59e0b;
    --goud-licht:     #fffbeb;
    --goud-glow:      rgba(245,158,11,0.2);

    --donker:         #111827;
    --donker-2:       #1f2937;
    --donker-3:       #374151;

    --tekst:          #374151;
    --tekst-zacht:    #6b7280;
    --tekst-extra:    #9ca3af;

    --rand:           #e5e7eb;
    --rand-licht:     #f3f4f6;
    --achtergrond:    #f7f4ef;
    --wit:            #ffffff;

    --groen:          #16a34a;
    --groen-licht:    #f0fdf4;
    --groen-rand:     #86efac;

    --blauw:          #2563eb;
    --blauw-licht:    #eff6ff;
    --blauw-rand:     #bfdbfe;

    --radius:         12px;
    --radius-lg:      18px;
    --radius-xl:      24px;

    --schaduw-xs:     0 1px 3px rgba(0,0,0,0.06), 0 1px 2px rgba(0,0,0,0.04);
    --schaduw:        0 2px 8px rgba(0,0,0,0.07), 0 1px 3px rgba(0,0,0,0.04);
    --schaduw-md:     0 4px 16px rgba(0,0,0,0.09), 0 2px 6px rgba(0,0,0,0.05);
    --schaduw-lg:     0 8px 32px rgba(0,0,0,0.12), 0 2px 8px rgba(0,0,0,0.06);
    --schaduw-xl:     0 20px 60px rgba(0,0,0,0.18), 0 4px 16px rgba(0,0,0,0.08);

    --font-display: 'Bebas Neue', sans-serif;
    --font-body:    'Nunito', sans-serif;

    --transition: 0.15s ease;
    --transition-md: 0.2s ease;
  }

  /* ── Reset ── */
  :global(*) { box-sizing: border-box; margin: 0; padding: 0; }

  :global(body) {
    font-family: var(--font-body);
    background:
      radial-gradient(ellipse at 8%  5%,  rgba(245, 197,  24, 0.18) 0%, transparent 45%),
      radial-gradient(ellipse at 92% 95%, rgba(220,  38,  38, 0.11) 0%, transparent 45%),
      radial-gradient(ellipse at 78% 12%, rgba(249, 115,  22, 0.10) 0%, transparent 38%),
      radial-gradient(ellipse at 22% 88%, rgba(234, 179,   8, 0.08) 0%, transparent 40%),
      #fdf6ec;
    background-attachment: fixed;
    color: var(--tekst);
    min-height: 100vh;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  /* ── Typography ── */
  :global(h1) {
    font-family: var(--font-display);
    font-size: 2rem;
    letter-spacing: 0.05em;
    color: var(--donker);
    margin-bottom: 1.5rem;
  }

  :global(h2) {
    font-family: var(--font-display);
    font-size: 1.35rem;
    letter-spacing: 0.04em;
    color: var(--donker);
    margin-top: 2rem;
    margin-bottom: 1rem;
  }

  /* ── Forms ── */
  :global(input), :global(select) {
    font-family: var(--font-body);
    padding: 0.7rem 1rem;
    border-radius: var(--radius);
    border: 1.5px solid var(--rand);
    font-size: 0.9rem;
    background: var(--wit);
    color: var(--tekst);
    transition: border-color var(--transition), box-shadow var(--transition);
    width: 100%;
    font-weight: 600;
  }

  :global(input:focus), :global(select:focus) {
    outline: none;
    border-color: var(--rood);
    box-shadow: 0 0 0 3px var(--rood-glow);
  }

  /* ── Global components ── */
  :global(.knop-primair) {
    font-family: var(--font-body);
    font-weight: 800;
    padding: 0.75rem 1.5rem;
    border-radius: var(--radius);
    border: none;
    cursor: pointer;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white;
    font-size: 0.95rem;
    transition: all var(--transition);
    box-shadow: 0 2px 8px var(--rood-glow);
  }

  :global(.knop-primair:hover) {
    background: linear-gradient(135deg, #e63535, var(--rood));
    transform: translateY(-1px);
    box-shadow: 0 4px 14px var(--rood-glow);
  }

  :global(.kaart) {
    background: var(--wit);
    border-radius: var(--radius-lg);
    border: 1px solid var(--rand);
    padding: 1rem 1.25rem;
    box-shadow: var(--schaduw);
    transition: box-shadow var(--transition-md), transform var(--transition-md);
  }

  :global(.kaart:hover) {
    box-shadow: var(--schaduw-md);
  }

  /* ── Shell ── */
  .app-shell {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  /* ── Header ── */
  header {
    background: linear-gradient(180deg, #111827 0%, #1a2233 100%);
    color: white;
    position: sticky;
    top: 0;
    z-index: 100;
    box-shadow: 0 1px 0 rgba(255,255,255,0.05), 0 4px 24px rgba(0,0,0,0.35);
  }

  .header-inner {
    max-width: 1080px;
    margin: 0 auto;
    padding: 0 1.5rem;
    display: flex;
    align-items: center;
    height: 66px;
    gap: 1rem;
  }

  /* Logo */
  .logo { display: flex; align-items: center; gap: 0.65rem; flex-shrink: 0; }

  .logo-merk {
    width: 40px;
    height: 40px;
    background: linear-gradient(135deg, var(--goud-helder), var(--goud));
    border-radius: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.4rem;
    box-shadow: 0 2px 10px var(--goud-glow);
    flex-shrink: 0;
  }

  .logo-tekst { display: flex; flex-direction: column; line-height: 1; }
  .logo-naam {
    font-family: var(--font-display);
    font-size: 1.5rem;
    letter-spacing: 0.08em;
    color: var(--goud-helder);
    line-height: 1;
  }
  .logo-sub {
    font-size: 0.58rem;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: rgba(255,255,255,0.35);
    margin-top: 2px;
  }

  /* Nav */
  nav {
    display: flex;
    gap: 0.15rem;
    flex: 1;
    justify-content: center;
    background: rgba(255,255,255,0.05);
    border-radius: 12px;
    padding: 0.3rem;
    border: 1px solid rgba(255,255,255,0.07);
  }

  .nav-knop {
    background: none;
    border: none;
    color: rgba(255,255,255,0.5);
    cursor: pointer;
    padding: 0.45rem 0.85rem;
    border-radius: 9px;
    font-family: var(--font-body);
    font-size: 0.78rem;
    font-weight: 700;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    transition: color var(--transition), background var(--transition);
    white-space: nowrap;
    min-width: 60px;
  }

  .nav-icon { font-size: 1rem; line-height: 1; }
  .nav-label { font-size: 0.68rem; letter-spacing: 0.04em; }

  .nav-knop:hover { color: rgba(255,255,255,0.85); background: rgba(255,255,255,0.07); }

  .nav-knop.actief {
    color: var(--donker);
    background: linear-gradient(135deg, var(--goud-helder), var(--goud));
    box-shadow: 0 2px 8px var(--goud-glow);
  }

  /* Header rechts */
  .header-rechts {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    flex-shrink: 0;
    margin-left: auto;
  }

  .gebruiker-chip {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 10px;
    padding: 0.35rem 0.65rem 0.35rem 0.35rem;
    cursor: pointer;
    transition: background var(--transition), border-color var(--transition);
  }

  .gebruiker-chip:hover {
    background: rgba(255,255,255,0.12);
    border-color: rgba(255,255,255,0.2);
  }

  .gebruiker-avatar {
    width: 30px;
    height: 30px;
    border-radius: 8px;
    background: linear-gradient(135deg, var(--rood), var(--rood-donker));
    color: white;
    font-size: 0.88rem;
    font-weight: 900;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-display);
    flex-shrink: 0;
  }

  .gebruiker-info { display: flex; flex-direction: column; line-height: 1.2; }
  .gebruiker-naam { font-size: 0.78rem; font-weight: 700; color: white; }
  .gebruiker-rol { font-size: 0.6rem; color: rgba(255,255,255,0.45); margin-top: 1px; }
  .gebruiker-rol.manager { color: var(--goud-helder); }

  .uitlog-knop {
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.1);
    color: rgba(255,255,255,0.5);
    border-radius: 9px;
    padding: 0.45rem 0.55rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition);
  }

  .uitlog-knop:hover {
    background: rgba(220,38,38,0.2);
    border-color: rgba(220,38,38,0.4);
    color: #fca5a5;
  }

  /* Main */
  main { flex: 1; }
  .pagina-inhoud {
    max-width: 1080px;
    width: 100%;
    margin: 0 auto;
    padding: 2.25rem 1.5rem;
  }

  /* Footer */
  footer {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 1rem;
    font-size: 0.75rem;
    color: var(--tekst-extra);
    border-top: 1px solid var(--rand);
    margin-top: auto;
  }
  .footer-dot { color: var(--rand); }

  /* ── Modal ── */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal-kaart {
    background: var(--wit);
    border-radius: var(--radius-xl);
    box-shadow: var(--schaduw-xl);
    width: 100%;
    max-width: 400px;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid var(--rand);
  }

  .modal-titel {
    font-family: var(--font-display);
    font-size: 1.2rem;
    letter-spacing: 0.04em;
    color: var(--donker);
    margin: 0;
  }

  .modal-sluit {
    background: var(--rand-licht);
    border: 1px solid var(--rand);
    border-radius: 8px;
    padding: 0.35rem;
    cursor: pointer;
    color: var(--tekst-zacht);
    display: flex;
    align-items: center;
    transition: all var(--transition);
  }

  .modal-sluit:hover { background: var(--rood-licht); color: var(--rood); border-color: var(--rood); }

  .modal-body { padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem; }

  .veld-label {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    font-size: 0.8rem;
    font-weight: 700;
    color: var(--tekst-zacht);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .fout-tekst { color: var(--rood); font-size: 0.82rem; font-weight: 600; }

  .modal-footer {
    display: flex;
    gap: 0.75rem;
    padding: 1.25rem 1.5rem;
    border-top: 1px solid var(--rand);
    background: var(--rand-licht);
  }

  .knop-annuleer {
    font-family: var(--font-body);
    font-weight: 700;
    padding: 0.7rem 1.25rem;
    border-radius: var(--radius);
    border: 1.5px solid var(--rand);
    background: var(--wit);
    color: var(--tekst-zacht);
    font-size: 0.9rem;
    cursor: pointer;
    transition: all var(--transition);
    flex: 1;
  }

  .knop-annuleer:hover { border-color: var(--tekst-zacht); color: var(--tekst); }

  .modal-footer .knop-primair { flex: 1; }

  .modal-succes {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 2rem 1.5rem;
    color: var(--groen);
    font-weight: 700;
    font-size: 1rem;
  }

  .succes-icoon {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--groen-licht);
    border: 2px solid var(--groen-rand);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.1rem;
    flex-shrink: 0;
  }

  /* ── Responsive ── */
  @media (max-width: 768px) {
    .header-inner { padding: 0 0.75rem; height: 58px; gap: 0.5rem; }
    .logo-sub, .gebruiker-info { display: none; }
    .logo-naam { font-size: 1.25rem; }
    nav { padding: 0.2rem; gap: 0.1rem; }
    .nav-knop { min-width: 46px; padding: 0.35rem 0.5rem; }
    .nav-label { display: none; }
    .nav-icon { font-size: 1.1rem; }
    .pagina-inhoud { padding: 1.25rem 0.875rem; }
  }
</style>
