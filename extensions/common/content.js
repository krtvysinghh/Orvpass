// Content Script for Orvpass Browser Extension
(() => {
  console.log("Orvpass v5.0.0 Autofill Engine active.");

  // Detect credential fields on the webpage
  function attachFieldBadges() {
    const inputs = document.querySelectorAll('input[type="password"], input[type="email"], input[name*="user"], input[name*="login"]');
    inputs.forEach(input => {
      if (input.dataset.orvpassAttached) return;
      input.dataset.orvpassAttached = "true";

      // Attach focus listener
      input.addEventListener('focus', () => {
        // Broadcast active domain to extension storage
        try {
          chrome.storage.local.set({ active_domain: window.location.hostname });
        } catch (e) {}
      });
    });
  }

  // Listen for background auto-fill commands (Cmd+Shift+L)
  chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    if (message.action === "TRIGGER_AUTOFILL") {
      chrome.storage.local.get(['vault_items'], (res) => {
        const items = res.vault_items || [];
        const host = window.location.hostname.toLowerCase();
        const matched = items.find(i => host.includes((i.title || '').toLowerCase()) || host.includes((i.url || '').toLowerCase())) || items[0];

        if (matched) {
          const userFields = document.querySelectorAll('input[type="text"], input[type="email"], input[name*="user"], input[name*="login"]');
          const passFields = document.querySelectorAll('input[type="password"]');

          if (userFields.length > 0 && matched.user) {
            userFields[0].value = matched.user;
            userFields[0].dispatchEvent(new Event('input', { bubbles: true }));
          }

          if (passFields.length > 0 && matched.pass) {
            passFields[0].value = matched.pass;
            passFields[0].dispatchEvent(new Event('input', { bubbles: true }));
          }
        }
      });
    }
  });

  // Observe dynamically loaded DOM forms (SPAs)
  const observer = new MutationObserver(() => attachFieldBadges());
  observer.observe(document.body || document.documentElement, { childList: true, subtree: true });
  attachFieldBadges();
})();
