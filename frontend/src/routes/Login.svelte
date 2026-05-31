<script>
  import { inloggen, registreren } from '../lib/api.js';
  import { sessie } from '../stores.js';

  let modus = 'inloggen'; // 'inloggen' | 'registreren'
  let gebruikersnaam = '';
  let wachtwoord = '';
  let naam = '';
  let rol = 'werknemer';
  let fout = '';
  let laden = false;

  async function handleInloggen() {
    if (!gebruikersnaam || !wachtwoord) { fout = 'Vul alle velden in'; return; }
    laden = true;
    fout = '';
    const res = await inloggen(gebruikersnaam, wachtwoord);
    laden = false;
    if (res.status === 'ok') {
      sessie.set({ token: res.token, rol: res.rol, gebruikersnaam: res.gebruikersnaam, medewerker_id: res.medewerker_id, naam });
    } else {
      fout = res.bericht || 'Inloggen mislukt';
    }
  }

  async function handleRegistreren() {
    if (!gebruikersnaam || !wachtwoord || !naam) { fout = 'Vul alle velden in'; return; }
    laden = true;
    fout = '';
    const res = await registreren(gebruikersnaam, wachtwoord, rol, naam);
    laden = false;
    if (res.status === 'ok') {
      // Direct inloggen na registreren
      const login = await inloggen(gebruikersnaam, wachtwoord);
      if (login.status === 'ok') {
        sessie.set({ token: login.token, rol: login.rol, gebruikersnaam: login.gebruikersnaam, medewerker_id: login.medewerker_id, naam });
      }
    } else {
      fout = res.bericht || 'Registreren mislukt';
    }
  }
</script>

<div class="login-wrapper">
  <div class="login-kaart">
    <div class="login-logo">
      <span class="logo-icoon">🍟</span>
      <div>
        <div class="logo-naam">Kolkie</div>
        <div class="logo-sub">Cafetaria</div>
      </div>
    </div>

    <div class="tabbladen">
      <button class:actief={modus === 'inloggen'} on:click={() => { modus = 'inloggen'; fout = ''; }}>
        Inloggen
      </button>
      <button class:actief={modus === 'registreren'} on:click={() => { modus = 'registreren'; fout = ''; }}>
        Account aanmaken
      </button>
    </div>

    {#if fout}
      <div class="fout-bericht">{fout}</div>
    {/if}

    {#if modus === 'inloggen'}
      <div class="form">
        <label class="form-veld">
          <span class="veld-label">Gebruikersnaam</span>
          <input bind:value={gebruikersnaam} placeholder="jouw gebruikersnaam" on:keydown={(e) => e.key === 'Enter' && handleInloggen()} />
        </label>
        <label class="form-veld">
          <span class="veld-label">Wachtwoord</span>
          <input type="password" bind:value={wachtwoord} placeholder="••••••••" on:keydown={(e) => e.key === 'Enter' && handleInloggen()} />
        </label>
        <button class="knop-inloggen" on:click={handleInloggen} disabled={laden}>
          {laden ? 'Bezig...' : 'Inloggen →'}
        </button>
      </div>
    {:else}
      <div class="form">
        <label class="form-veld">
          <span class="veld-label">Volledige naam</span>
          <input bind:value={naam} placeholder="bijv. Jan de Vries" />
        </label>
        <label class="form-veld">
          <span class="veld-label">Gebruikersnaam</span>
          <input bind:value={gebruikersnaam} placeholder="jouw gebruikersnaam" />
        </label>
        <label class="form-veld">
          <span class="veld-label">Wachtwoord</span>
          <input type="password" bind:value={wachtwoord} placeholder="minimaal 6 tekens" />
        </label>
        <label class="form-veld">
          <span class="veld-label">Rol</span>
          <div class="rol-keuze">
            <button
              class="rol-knop"
              class:actief={rol === 'werknemer'}
              on:click={() => rol = 'werknemer'}
            >
              <span class="rol-icoon">👤</span>
              <span class="rol-naam">Werknemer</span>
              <span class="rol-omschrijving">Inklokken & rooster bekijken</span>
            </button>
            <button
              class="rol-knop"
              class:actief={rol === 'manager'}
              on:click={() => rol = 'manager'}
            >
              <span class="rol-icoon">⭐</span>
              <span class="rol-naam">Manager</span>
              <span class="rol-omschrijving">Alles beheren</span>
            </button>
          </div>
        </label>
        <button class="knop-inloggen" on:click={handleRegistreren} disabled={laden}>
          {laden ? 'Bezig...' : 'Account aanmaken →'}
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .login-wrapper {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--achtergrond);
    padding: 1rem;
  }

  .login-kaart {
    background: var(--wit);
    border-radius: 20px;
    border: 1.5px solid var(--rand);
    padding: 2.5rem 2rem;
    width: 100%;
    max-width: 420px;
    box-shadow: var(--schaduw-groot);
  }

  .login-logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 2rem;
    justify-content: center;
  }
  .logo-icoon { font-size: 2.5rem; }
  .logo-naam {
    font-family: var(--font-display);
    font-size: 2rem;
    color: var(--donker);
    letter-spacing: 0.06em;
    line-height: 1;
  }
  .logo-sub {
    font-size: 0.65rem;
    letter-spacing: 0.15em;
    text-transform: uppercase;
    color: var(--tekst-zacht);
  }

  .tabbladen {
    display: flex;
    background: #f3f4f6;
    border-radius: 10px;
    padding: 0.25rem;
    margin-bottom: 1.75rem;
    gap: 0.25rem;
  }
  .tabbladen button {
    flex: 1;
    background: none;
    border: none;
    padding: 0.55rem;
    border-radius: 8px;
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 0.9rem;
    cursor: pointer;
    color: var(--tekst-zacht);
    transition: all 0.15s;
  }
  .tabbladen button.actief {
    background: var(--wit);
    color: var(--donker);
    box-shadow: 0 1px 4px rgba(0,0,0,0.1);
  }

  .fout-bericht {
    background: var(--rood-licht);
    color: var(--rood-donker);
    border: 1.5px solid #f5c6c6;
    border-radius: 8px;
    padding: 0.65rem 1rem;
    font-size: 0.9rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .form { display: flex; flex-direction: column; gap: 1rem; }

  .form-veld {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .veld-label {
    font-size: 0.8rem;
    font-weight: 700;
    color: var(--tekst-zacht);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .rol-keuze {
    display: flex;
    gap: 0.5rem;
  }

  .rol-knop {
    flex: 1;
    background: var(--wit);
    border: 2px solid var(--rand);
    border-radius: var(--radius);
    padding: 0.75rem;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.2rem;
    transition: border-color 0.15s, background 0.15s;
    font-family: var(--font-body);
  }
  .rol-knop.actief {
    border-color: var(--rood);
    background: var(--rood-licht);
  }
  .rol-icoon { font-size: 1.4rem; }
  .rol-naam { font-weight: 700; font-size: 0.85rem; color: var(--donker); }
  .rol-omschrijving { font-size: 0.7rem; color: var(--tekst-zacht); text-align: center; }

  .knop-inloggen {
    width: 100%;
    padding: 0.9rem;
    background: var(--rood);
    color: white;
    border: none;
    border-radius: var(--radius);
    font-family: var(--font-display);
    font-size: 1.2rem;
    letter-spacing: 0.06em;
    cursor: pointer;
    transition: background 0.15s, transform 0.1s;
    margin-top: 0.5rem;
  }
  .knop-inloggen:hover:not(:disabled) {
    background: var(--rood-donker);
    transform: translateY(-1px);
  }
  .knop-inloggen:disabled { opacity: 0.6; cursor: not-allowed; }
</style>
