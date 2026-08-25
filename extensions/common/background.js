// Background Service Worker for Orvpass Manifest V3 Extension
chrome.runtime.onInstalled.addListener(() => {
  console.log("Orvpass v5.0.0 Extension ready.");
});

// Keyboard shortcut listener (Cmd+Shift+L / Ctrl+Shift+L)
chrome.commands.onCommand.addListener(async (command) => {
  if (command === "auto_fill") {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    if (tab && tab.id) {
      chrome.tabs.sendMessage(tab.id, { action: "TRIGGER_AUTOFILL" }).catch(() => {
        // Content script injection fallback
      });
    }
  }
});

// Runtime message passing
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === "GET_VAULT_STATUS") {
    sendResponse({ status: "connected", version: "5.0.0" });
  }
  return true;
});
