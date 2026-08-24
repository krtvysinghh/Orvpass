// Background script to communicate with Orvpass desktop app via native messaging
chrome.runtime.onInstalled.addListener(() => {
  console.log("Orvpass extension installed.");
});
