document.addEventListener('DOMContentLoaded', async () => {
  const searchInput = document.getElementById('searchInput');
  const vaultList = document.getElementById('vaultList');
  const tabVault = document.getElementById('tabVault');
  const tabGen = document.getElementById('tabGen');
  const vaultView = document.getElementById('vaultView');
  const genView = document.getElementById('genView');
  const genResult = document.getElementById('genResult');
  const btnCopyGen = document.getElementById('btnCopyGen');
  const btnRegen = document.getElementById('btnRegen');

  // Load items from local encrypted session storage or defaults
  const storageData = await chrome.storage.local.get(['vault_items', 'active_domain']);
  let vaultItems = storageData.vault_items || [
    { id: '1', title: 'GitHub', user: 'krtvysinghh', pass: 'OrvpassSecure2026!', type: 'Logins' },
    { id: '2', title: 'Google', user: 'krtvysingh@gmail.com', pass: 'UltraPasskey123#', type: 'Logins' },
    { id: '3', title: 'AWS Cloud Admin', user: 'root', pass: 'AwsRootVaultKey99$', type: 'Logins' }
  ];

  // Tab switching
  tabVault.addEventListener('click', () => {
    tabVault.classList.add('active');
    tabGen.classList.remove('active');
    vaultView.style.display = 'block';
    genView.style.display = 'none';
  });

  tabGen.addEventListener('click', () => {
    tabGen.classList.add('active');
    tabVault.classList.remove('active');
    vaultView.style.display = 'none';
    genView.style.display = 'block';
    generatePassword();
  });

  function generatePassword() {
    const words = [
      "falcon", "shield", "crypto", "cipher", "matrix", "beacon", "galaxy", "orbit",
      "quantum", "vector", "shadow", "summit", "horizon", "glacier", "phoenix", "aurora",
      "nebula", "zenith", "vortex", "starlight", "timber", "cascade", "dynamo", "solace"
    ];
    const array = new Uint32Array(4);
    window.crypto.getRandomValues(array);
    const passphrase = Array.from(array).map(n => words[n % words.length]).join("-") + Math.floor(Math.random() * 90 + 10);
    genResult.innerText = passphrase;
  }

  btnRegen.addEventListener('click', generatePassword);
  btnCopyGen.addEventListener('click', async () => {
    await navigator.clipboard.writeText(genResult.innerText);
    btnCopyGen.innerText = 'Copied!';
    setTimeout(() => btnCopyGen.innerText = 'Copy to Clipboard', 2000);
  });

  function renderItems(query = '') {
    vaultList.innerHTML = '';
    const filtered = vaultItems.filter(i => 
      i.title.toLowerCase().includes(query.toLowerCase()) || 
      (i.user && i.user.toLowerCase().includes(query.toLowerCase()))
    );

    if (filtered.length === 0) {
      vaultList.innerHTML = '<div style="text-align:center; padding: 24px; color: #94a3b8; font-size: 12px;">No matching credentials</div>';
      return;
    }

    filtered.forEach(item => {
      const div = document.createElement('div');
      div.className = 'item';
      div.innerHTML = `
        <div style="min-width: 0; flex: 1;">
          <div class="item-title">${item.title}</div>
          <div class="item-user">${item.user || item.type}</div>
        </div>
        <div class="actions">
          <button class="btn btn-secondary btn-copy-user" title="Copy User">👤</button>
          <button class="btn btn-secondary btn-copy-pass" title="Copy Pass">🔑</button>
          <button class="btn btn-primary btn-fill">Fill</button>
        </div>
      `;

      div.querySelector('.btn-copy-user').addEventListener('click', (e) => {
        e.stopPropagation();
        navigator.clipboard.writeText(item.user || '');
      });

      div.querySelector('.btn-copy-pass').addEventListener('click', (e) => {
        e.stopPropagation();
        navigator.clipboard.writeText(item.pass || '');
      });

      div.querySelector('.btn-fill').addEventListener('click', async () => {
        const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
        if (tab && tab.id) {
          await chrome.scripting.executeScript({
            target: { tabId: tab.id },
            func: triggerAutofill,
            args: [item.user, item.pass]
          });
          window.close();
        }
      });

      vaultList.appendChild(div);
    });
  }

  searchInput.addEventListener('input', (e) => renderItems(e.target.value));
  renderItems();
});

function triggerAutofill(username, password) {
  const userFields = document.querySelectorAll('input[type="text"], input[type="email"], input[name*="user"], input[name*="login"], input[id*="user"]');
  const passFields = document.querySelectorAll('input[type="password"]');

  if (userFields.length > 0 && username) {
    userFields[0].value = username;
    userFields[0].dispatchEvent(new Event('input', { bubbles: true }));
    userFields[0].dispatchEvent(new Event('change', { bubbles: true }));
  }

  if (passFields.length > 0 && password) {
    passFields[0].value = password;
    passFields[0].dispatchEvent(new Event('input', { bubbles: true }));
    passFields[0].dispatchEvent(new Event('change', { bubbles: true }));
  }
}
