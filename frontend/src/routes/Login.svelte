<script>
  import { inloggen, registreren } from '../lib/api.js';
  import { sessie } from '../stores.js';

  let modus = 'inloggen';
  let gebruikersnaam = '';
  let wachtwoord = '';
  let naam = '';
  let fout = '';
  let laden = false;

  async function handleInloggen() {
    if (!gebruikersnaam || !wachtwoord) { fout = 'Vul alle velden in'; return; }
    laden = true;
    fout = '';
    const res = await inloggen(gebruikersnaam, wachtwoord);
    laden = false;
    if (res.status === 'ok') {
      sessie.set({ token: res.token, rol: res.rol, gebruikersnaam: res.gebruikersnaam, medewerker_id: res.medewerker_id });
    } else {
      fout = res.bericht || 'Inloggen mislukt';
    }
  }

  async function handleRegistreren() {
    if (!gebruikersnaam || !wachtwoord || !naam) { fout = 'Vul alle velden in'; return; }
    if (wachtwoord.length < 6) { fout = 'Wachtwoord moet minimaal 6 tekens zijn'; return; }
    laden = true;
    fout = '';
    const res = await registreren(gebruikersnaam, wachtwoord, naam);
    laden = false;
    if (res.status === 'ok') {
      const login = await inloggen(gebruikersnaam, wachtwoord);
      if (login.status === 'ok') {
        sessie.set({ token: login.token, rol: login.rol, gebruikersnaam: login.gebruikersnaam, medewerker_id: login.medewerker_id });
      }
    } else {
      fout = res.bericht || 'Registreren mislukt';
    }
  }
</script>

<div class="achtergrond">
  <div class="login-kaart">
    <div class="login-logo">
      <div class="logo-cirkel">🍟</div>
      <div class="logo-teksten">
        <div class="logo-naam">Kolkie</div>
        <div class="logo-sub">Cafetaria · Personeelssysteem</div>
      </div>
    </div>

    <div class="tabbladen">
      <button class:actief={modus === 'inloggen'} on:click={() => { modus = 'inloggen'; fout = ''; }}>
        Inloggen
      </button>
      <button class:actief={modus === 'registreren'} on:click={() => { modus = 'registreren'; fout = ''; }}>
        Nieuw account
      </button>
    </div>

    {#if fout}
      <div class="fout-bericht">
        <span class="fout-icoon">!</span>
        {fout}
      </div>
    {/if}

    {#if modus === 'inloggen'}
      <div class="form">
        <label class="form-veld">
          <span class="veld-label">Gebruikersnaam</span>
          <input bind:value={gebruikersnaam} placeholder="bijv. jan.devries" on:keydown={(e) => e.key === 'Enter' && handleInloggen()} autocomplete="username" />
        </label>
        <label class="form-veld">
          <span class="veld-label">Wachtwoord</span>
          <input type="password" bind:value={wachtwoord} placeholder="••••••••" on:keydown={(e) => e.key === 'Enter' && handleInloggen()} autocomplete="current-password" />
        </label>
        <button class="knop-submit" on:click={handleInloggen} disabled={laden}>
          {#if laden}
            <span class="spinner"></span> Bezig...
          {:else}
            Inloggen →
          {/if}
        </button>
      </div>
      <p class="hint">Standaard admin: <strong>admin</strong> / <strong>admin123</strong></p>
    {:else}
      <div class="form">
        <label class="form-veld">
          <span class="veld-label">Volledige naam</span>
          <input bind:value={naam} placeholder="bijv. Jan de Vries" autocomplete="name" />
        </label>
        <label class="form-veld">
          <span class="veld-label">Gebruikersnaam</span>
          <input bind:value={gebruikersnaam} placeholder="bijv. jan.devries" autocomplete="username" />
        </label>
        <label class="form-veld">
          <span class="veld-label">Wachtwoord</span>
          <input type="password" bind:value={wachtwoord} placeholder="minimaal 6 tekens" autocomplete="new-password" />
        </label>
        <div class="rol-info">
          <span class="rol-icoon">👤</span>
          <span>Nieuw account wordt aangemaakt als <strong>Werknemer</strong>. Een manager kan je daarna promoveren.</span>
        </div>
        <button class="knop-submit" on:click={handleRegistreren} disabled={laden}>
          {#if laden}
            <span class="spinner"></span> Bezig...
          {:else}
            Account aanmaken →
          {/if}
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .achtergrond {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #1a1208 0%, #2e2210 50%, #3d2d14 100%);
    padding: 1rem;
    position: relative;
    overflow: hidden;
  }

  .achtergrond::before {
    content: '';
    position: absolute;
    inset: 0;
    background: radial-gradient(ellipse at 20% 50%, rgba(245,197,24,0.08) 0%, transparent 60%),
                radial-gradient(ellipse at 80% 20%, rgba(224,31,31,0.06) 0%, transparent 50%);
    pointer-events: none;
  }

  .login-kaart {
    background: #ffffff;
    border-radius: 24px;
    padding: 2.5rem 2.25rem;
    width: 100%;
    max-width: 420px;
    box-shadow: 0 24px 64px rgba(0,0,0,0.4), 0 4px 16px rgba(0,0,0,0.2);
    position: relative;
    z-index: 1;
  }

  .login-logo {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
    justify-content: center;
  }

  .logo-cirkel {
    width: 56px;
    height: 56px;
    background: linear-gradient(135deg, #f5c518, #e8a800);
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.8rem;
    box-shadow: 0 4px 12px rgba(245,197,24,0.35);
    flex-shrink: 0;
  }

  .logo-teksten { display: flex; flex-direction: column; }

  .logo-naam {
    font-family: var(--font-display);
    font-size: 2.2rem;
    color: var(--donker);
    letter-spacing: 0.06em;
    line-height: 1;
  }

  .logo-sub {
    font-size: 0.65rem;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--tekst-zacht);
    margin-top: 2px;
  }

  .tabbladen {
    display: flex;
    background: #f3f4f6;
    border-radius: 12px;
    padding: 0.3rem;
    margin-bottom: 1.75rem;
    gap: 0.25rem;
  }

  .tabbladen button {
    flex: 1;
    background: none;
    border: none;
    padding: 0.6rem;
    border-radius: 9px;
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
    box-shadow: 0 1px 6px rgba(0,0,0,0.12);
  }

  .fout-bericht {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    background: #fff0f0;
    color: #b81515;
    border: 1.5px solid #fca5a5;
    border-radius: 10px;
    padding: 0.7rem 1rem;
    font-size: 0.88rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .fout-icoon {
    width: 20px;
    height: 20px;
    background: #b81515;
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    font-weight: 800;
    flex-shrink: 0;
  }

  .form { display: flex; flex-direction: column; gap: 1.1rem; }

  .form-veld {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .veld-label {
    font-size: 0.78rem;
    font-weight: 700;
    color: var(--tekst-zacht);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .rol-info {
    display: flex;
    align-items: flex-start;
    gap: 0.6rem;
    background: #fffbea;
    border: 1.5px solid #fde68a;
    border-radius: 10px;
    padding: 0.75rem 1rem;
    font-size: 0.85rem;
    color: #78350f;
    line-height: 1.4;
  }

  .rol-icoon { font-size: 1.1rem; flex-shrink: 0; margin-top: 1px; }

  .knop-submit {
    width: 100%;
    padding: 0.95rem;
    background: linear-gradient(135deg, #e01f1f, #c01515);
    color: white;
    border: none;
    border-radius: 12px;
    font-family: var(--font-display);
    font-size: 1.15rem;
    letter-spacing: 0.06em;
    cursor: pointer;
    transition: all 0.15s;
    margin-top: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    box-shadow: 0 4px 12px rgba(224,31,31,0.3);
  }

  .knop-submit:hover:not(:disabled) {
    background: linear-gradient(135deg, #c01515, #a01010);
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(224,31,31,0.4);
  }

  .knop-submit:disabled { opacity: 0.6; cursor: not-allowed; transform: none; }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: draaien 0.7s linear infinite;
  }

  @keyframes draaien { to { transform: rotate(360deg); } }

  .hint {
    text-align: center;
    font-size: 0.78rem;
    color: var(--tekst-zacht);
    margin-top: 1.25rem;
  }

  @media (max-width: 480px) {
    .login-kaart { padding: 2rem 1.5rem; border-radius: 20px; }
  }
</style>
