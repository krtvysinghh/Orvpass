document.addEventListener('DOMContentLoaded', () => {
  const searchInput = document.getElementById('searchInput');
  const vaultList = document.getElementById('vaultList');

  // Dummy data representing connection to desktop app
  const vaultItems = [
    { id: 1, title: 'Github', user: 'krtvysinghh' },
    { id: 2, title: 'Gmail', user: 'test@example.com' },
  ];

  function renderItems(query = '') {
    vaultList.innerHTML = '';
    const filtered = vaultItems.filter(i => i.title.toLowerCase().includes(query.toLowerCase()));
    
    if (filtered.length === 0) {
      vaultList.innerHTML = '<div style="text-align:center; padding: 20px; color: #9ca3af; font-size: 13px;">No items found</div>';
      return;
    }

    filtered.forEach(item => {
      const div = document.createElement('div');
      div.className = 'item';
      div.innerHTML = `
        <div>
          <div class="item-title">${item.title}</div>
          <div class="item-user">${item.user}</div>
        </div>
        <button class="autofill-btn">Autofill</button>
      `;
      div.querySelector('.autofill-btn').addEventListener('click', () => {
        chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
          chrome.scripting.executeScript({
            target: {tabId: tabs[0].id},
            function: triggerAutofill,
            args: [item.user, "dummy_password_from_vault"]
          });
          window.close();
        });
      });
      vaultList.appendChild(div);
    });
  }

  searchInput.addEventListener('input', (e) => renderItems(e.target.value));
  renderItems();
});

// This runs in the context of the web page
function triggerAutofill(username, password) {
  const userFields = document.querySelectorAll('input[type="text"], input[type="email"], input[name*="user"]');
  const passFields = document.querySelectorAll('input[type="password"]');
  
  if (userFields.length > 0) {
    userFields[0].value = username;
    userFields[0].dispatchEvent(new Event('input', { bubbles: true }));
  }
  
  if (passFields.length > 0) {
    passFields[0].value = password;
    passFields[0].dispatchEvent(new Event('input', { bubbles: true }));
  }
}
