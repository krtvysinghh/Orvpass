import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Item {
  id: string;
  title: string;
  username?: string;
  password?: string;
  type: string;
  pinned?: boolean;
}

function App() {
  const [theme, setTheme] = useState<'dark' | 'light' | 'system'>('dark');
  const [items, setItems] = useState<Item[]>([]);
  const [activeTab, setActiveTab] = useState('All Items');
  const [showAddModal, setShowAddModal] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [newItemType, setNewItemType] = useState('Logins');
  const [newItem, setNewItem] = useState({ title: '', username: '', password: '', notes: '', cc: '' });
  const [vaultUnlocked, setVaultUnlocked] = useState(false);

  // Initialize vault on startup
  useEffect(() => {
    invoke('initialize_vault', { password: "master_password_placeholder" })
      .then(() => {
        setVaultUnlocked(true);
        loadItems();
      })
      .catch(console.error);

    // Register global shortcut (desktop only)
    try {
      import('@tauri-apps/plugin-global-shortcut').then(async ({ register }) => {
        try {
          await register('CmdOrControl+Shift+Space', () => {
            import('@tauri-apps/api/window').then(({ getCurrentWindow }) => {
              const w = getCurrentWindow();
              w.isVisible().then(v => v ? w.hide() : w.show());
              w.setFocus();
            }).catch(() => {});
          });
        } catch (e) {
          // ignore on platforms without global shortcuts
        }
      }).catch(() => {});
    } catch (e) {
      // ignore
    }
  }, []);

  const loadItems = () => {
    invoke<any[]>('get_items')
      .then(res => {
        const mapped = res.map(r => ({
          id: r.id,
          title: r.title,
          username: r.data?.Login?.username || '',
          password: r.data?.Login?.password || '',
          type: 'Logins',
          pinned: false
        }));
        setItems(mapped);
      })
      .catch(console.error);
  };

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove('light', 'dark');
    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
      root.classList.add(systemTheme);
    } else {
      root.classList.add(theme);
    }
  }, [theme]);

  const handleAddItem = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newItem.title) return;
    try {
      await invoke('add_item', { title: newItem.title, username: newItem.username, pass: newItem.password });
      loadItems();
      setNewItem({ title: '', username: '', password: '', notes: '', cc: '' });
      setShowAddModal(false);
    } catch (err) {
      console.error(err);
    }
  };

  const handleGeneratePassword = async () => {
    try {
      const generated = await invoke<string>('generate_password', { length: 16 });
      setNewItem({ ...newItem, password: generated });
    } catch (err) {
      console.error(err);
    }
  };

  const handleOpenAddModal = () => {
    if (['Logins', 'Secure Notes', 'Credit Cards'].includes(activeTab)) {
      setNewItemType(activeTab);
    } else {
      setNewItemType('Logins');
    }
    setShowAddModal(true);
  };

  const handleImportExport = (action: string) => {
    alert(`This will open the native file picker to ${action}`);
  };

  const handleCheckUpdate = async () => {
    try {
      const { check } = await import('@tauri-apps/plugin-updater');
      const update = await check();
      if (update) {
        if (window.confirm(`Update ${update.version} is available! Do you want to download and install it now?`)) {
          await update.downloadAndInstall();
          const { relaunch } = await import('@tauri-apps/plugin-process');
          await relaunch();
        }
      } else {
        alert('You are on the latest version of Orvpass.');
      }
    } catch (e) {
      console.error(e);
      alert('Failed to check for updates. Make sure tauri.conf.json has update endpoints configured.');
    }
  };

  const handleDelete = (id: string) => {
    if (window.confirm("Move to Trash? It will be auto-deleted in 60 days.")) {
      setItems(items.filter(i => i.id !== id));
    }
  };

  const handleTogglePin = (id: string) => setItems(items.map(i => i.id === id ? { ...i, pinned: !i.pinned } : i));
  const copyToClipboard = (text: string) => navigator.clipboard.writeText(text);

  const filteredItems = activeTab === 'All Items' ? items 
    : activeTab === 'Favorites' ? items.filter(i => i.pinned)
    : items.filter(i => i.type === activeTab);

  const sortedItems = [...filteredItems].sort((a, b) => (a.pinned === b.pinned ? 0 : a.pinned ? -1 : 1));

  if (!vaultUnlocked) {
    return <div className="flex h-screen w-full items-center justify-center bg-black/90 text-white">Unlocking Vault...</div>;
  }

  return (
    <div className="flex h-screen w-full bg-white/70 dark:bg-zinc-900/80 backdrop-blur-xl text-zinc-900 dark:text-white font-sans antialiased transition-colors duration-300">
      
      {/* Settings Modal */}
      {showSettings && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/20 dark:bg-black/50 backdrop-blur-sm">
          <div className="bg-white/90 dark:bg-zinc-800/90 border border-black/10 dark:border-white/10 p-6 rounded-xl shadow-2xl w-[600px] flex flex-col max-h-[80vh]">
            <h3 className="text-xl font-semibold mb-6">Settings</h3>
            <div className="space-y-8 overflow-y-auto pr-4 flex-1">
              
              {/* Appearance */}
              <div>
                <h4 className="text-sm font-semibold mb-3 opacity-70 uppercase tracking-wider text-blue-500">Appearance</h4>
                <div className="flex gap-2">
                  {['light', 'dark', 'system'].map(t => (
                    <button key={t} onClick={() => setTheme(t as any)} className={`px-4 py-2 rounded-lg text-sm font-medium capitalize border transition-all ${theme === t ? 'border-blue-500 bg-blue-500/10 text-blue-600 dark:text-blue-400' : 'border-transparent bg-black/5 dark:bg-white/5 hover:bg-black/10 dark:hover:bg-white/10'}`}>
                      {t}
                    </button>
                  ))}
                </div>
              </div>

              {/* Security */}
              <div>
                <h4 className="text-sm font-semibold mb-3 opacity-70 uppercase tracking-wider text-blue-500">Security</h4>
                <div className="space-y-3">
                  <div className="flex justify-between items-center">
                    <span className="text-sm font-medium">Vault Auto-Lock</span>
                    <select className="bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-1 text-sm outline-none">
                      <option>Never</option>
                      <option>5 Minutes</option>
                      <option>15 Minutes</option>
                      <option>On Sleep</option>
                    </select>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-sm font-medium">Clear Clipboard After</span>
                    <select className="bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-1 text-sm outline-none">
                      <option>30 Seconds</option>
                      <option>60 Seconds</option>
                      <option>Never</option>
                    </select>
                  </div>
                </div>
              </div>

              {/* Data Management */}
              <div>
                <h4 className="text-sm font-semibold mb-3 opacity-70 uppercase tracking-wider text-blue-500">Data & Import</h4>
                <p className="text-xs opacity-60 mb-3">Import from CSV, 1Password, Bitwarden, LastPass, or Ente Auth.</p>
                <div className="grid grid-cols-2 gap-3">
                  <button onClick={() => handleImportExport('import Passwords')} className="px-4 py-2 rounded-lg text-sm font-medium bg-black/5 dark:bg-white/5 hover:bg-black/10 dark:hover:bg-white/10 transition-all text-left">⬇️ Import Passwords</button>
                  <button onClick={() => handleImportExport('export Passwords')} className="px-4 py-2 rounded-lg text-sm font-medium bg-black/5 dark:bg-white/5 hover:bg-black/10 dark:hover:bg-white/10 transition-all text-left">⬆️ Export Vault (CSV)</button>
                  <button onClick={() => handleImportExport('import TOTP')} className="px-4 py-2 rounded-lg text-sm font-medium bg-black/5 dark:bg-white/5 hover:bg-black/10 dark:hover:bg-white/10 transition-all text-left">⬇️ Import TOTP Auth</button>
                  <button onClick={() => handleImportExport('export TOTP')} className="px-4 py-2 rounded-lg text-sm font-medium bg-black/5 dark:bg-white/5 hover:bg-black/10 dark:hover:bg-white/10 transition-all text-left">⬆️ Export TOTP</button>
                </div>
              </div>

              {/* About */}
              <div>
                <h4 className="text-sm font-semibold mb-3 opacity-70 uppercase tracking-wider text-blue-500">About</h4>
                <div className="flex justify-between items-center">
                  <span className="text-sm font-medium">Orvpass v1.0.0</span>
                  <button onClick={handleCheckUpdate} className="px-4 py-1.5 rounded-lg text-sm font-medium bg-blue-600 hover:bg-blue-500 text-white transition-all shadow-sm">Check for Update</button>
                </div>
              </div>
            </div>

            <div className="mt-8 flex justify-between items-center pt-4 border-t border-black/10 dark:border-white/10">
              <span className="text-xs font-mono opacity-50 tracking-wider">@krtvysinghh</span>
              <button onClick={() => setShowSettings(false)} className="px-5 py-2 rounded-lg bg-blue-600 hover:bg-blue-500 text-white font-medium shadow">Done</button>
            </div>
          </div>
        </div>
      )}

      {/* Add Item Modal */}
      {showAddModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/20 dark:bg-black/50 backdrop-blur-sm">
          <div className="bg-white/90 dark:bg-zinc-800/90 border border-black/10 dark:border-white/10 p-6 rounded-xl shadow-2xl w-[600px] flex">
            
            {/* Left: Fields */}
            <div className="flex-1 pr-6 border-r border-black/10 dark:border-white/10">
              <div className="flex justify-between items-center mb-4">
                <h3 className="text-xl font-semibold">New Item</h3>
                <select value={newItemType} onChange={e => setNewItemType(e.target.value)} className="bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-2 py-1 text-sm outline-none">
                  <option>Logins</option>
                  <option>Secure Notes</option>
                  <option>Credit Cards</option>
                </select>
              </div>
              <form onSubmit={handleAddItem} className="space-y-4">
                <div>
                  <label className="block text-xs font-medium opacity-70 mb-1">Title</label>
                  <input autoFocus type="text" value={newItem.title} onChange={e => setNewItem({...newItem, title: e.target.value})} className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" required />
                </div>

                {newItemType === 'Logins' && (
                  <>
                    <div>
                      <label className="block text-xs font-medium opacity-70 mb-1">Username / Email</label>
                      <input type="text" value={newItem.username} onChange={e => setNewItem({...newItem, username: e.target.value})} className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                    </div>
                    <div>
                      <div className="flex items-center justify-between mb-1">
                        <label className="block text-xs font-medium opacity-70">Password</label>
                        <button type="button" onClick={() => setShowPassword(!showPassword)} className="text-xs text-blue-500 hover:text-blue-400 font-medium">{showPassword ? 'Hide' : 'Show'}</button>
                      </div>
                      <input type={showPassword ? "text" : "password"} value={newItem.password} onChange={e => setNewItem({...newItem, password: e.target.value})} className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                    </div>
                    <div>
                      <label className="block text-xs font-medium opacity-70 mb-1">Website URL</label>
                      <input type="url" placeholder="https://" className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                    </div>
                    <div>
                      <label className="block text-xs font-medium opacity-70 mb-1">TOTP Secret</label>
                      <input type="text" placeholder="JBSWY3DPEHPK3PXP" className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                    </div>
                  </>
                )}

                {newItemType === 'Secure Notes' && (
                  <div>
                    <label className="block text-xs font-medium opacity-70 mb-1">Note Content (Markdown)</label>
                    <textarea rows={6} className="w-full font-mono bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500"></textarea>
                  </div>
                )}

                {newItemType === 'Credit Cards' && (
                  <>
                    <div>
                      <label className="block text-xs font-medium opacity-70 mb-1">Card Number</label>
                      <input type="text" placeholder="0000 0000 0000 0000" className="w-full font-mono bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                    </div>
                    <div className="flex gap-4">
                      <div className="flex-1">
                        <label className="block text-xs font-medium opacity-70 mb-1">Expiry (MM/YY)</label>
                        <input type="text" placeholder="MM/YY" className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                      </div>
                      <div className="flex-1">
                        <label className="block text-xs font-medium opacity-70 mb-1">CVV</label>
                        <input type="password" placeholder="123" className="w-full bg-black/5 dark:bg-black/30 border border-black/10 dark:border-white/10 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500" />
                      </div>
                    </div>
                  </>
                )}
                
                <div className="flex justify-end space-x-3 mt-6">
                  <button type="button" onClick={() => setShowAddModal(false)} className="px-4 py-2 rounded text-sm hover:bg-black/5 dark:hover:bg-white/5 transition">Cancel</button>
                  <button type="submit" className="px-4 py-2 rounded text-sm bg-blue-600 hover:bg-blue-500 text-white font-medium transition shadow">Save</button>
                </div>
              </form>
            </div>

            {/* Right: Password Generator */}
            {newItemType === 'Logins' && (
              <div className="w-48 pl-6 flex flex-col">
                <h4 className="text-sm font-semibold opacity-70 uppercase tracking-wider mb-4">Generator</h4>
                <div className="space-y-4">
                  <div>
                    <label className="block text-xs mb-1">Length: 16</label>
                    <input type="range" min="8" max="64" defaultValue="16" className="w-full" />
                  </div>
                  <label className="flex items-center space-x-2 text-xs">
                    <input type="checkbox" defaultChecked /> <span>A-Z (Uppercase)</span>
                  </label>
                  <label className="flex items-center space-x-2 text-xs">
                    <input type="checkbox" defaultChecked /> <span>0-9 (Numbers)</span>
                  </label>
                  <label className="flex items-center space-x-2 text-xs">
                    <input type="checkbox" defaultChecked /> <span>!@# (Symbols)</span>
                  </label>
                  <button type="button" onClick={handleGeneratePassword} className="mt-4 w-full px-3 py-2 rounded text-sm bg-blue-600/20 text-blue-600 dark:text-blue-400 hover:bg-blue-600/30 font-medium transition">
                    Generate
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Sidebar (Desktop) / Bottom Nav (Mobile) */}
      <div className="w-full md:w-64 border-r border-black/5 dark:border-white/10 bg-black/5 dark:bg-black/20 flex md:flex-col pt-4 md:pt-8 pb-4 absolute md:relative bottom-0 z-40">
        <div className="hidden md:flex px-6 mb-8 justify-between items-center">
          <h1 className="text-xl font-semibold tracking-tight">Orvpass</h1>
          <button onClick={() => setShowSettings(true)} className="p-1.5 rounded-md hover:bg-black/10 dark:hover:bg-white/10 text-xl" title="Settings">⚙️</button>
        </div>
        <nav className="flex flex-row md:flex-col flex-1 justify-around space-y-0 md:space-y-1 px-3 w-full">
          {['All Items', 'Favorites', 'Health', 'Authenticator'].map((item) => (
            <button key={item} onClick={() => setActiveTab(item)} className={`md:w-full flex items-center justify-center md:justify-start px-3 py-3 md:py-2 text-xs md:text-sm font-medium rounded-md transition-colors ${activeTab === item ? 'bg-blue-500/10 text-blue-600 dark:bg-blue-500/20 dark:text-blue-400' : 'hover:bg-black/5 dark:hover:bg-white/5 opacity-70 hover:opacity-100'}`}>
              {item}
            </button>
          ))}
          <button onClick={() => setShowSettings(true)} className="md:hidden flex items-center justify-center px-3 py-3 text-xs font-medium rounded-md hover:bg-black/5 dark:hover:bg-white/5 opacity-70">
            Settings
          </button>
        </nav>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col mb-16 md:mb-0">
        <header data-tauri-drag-region className="h-14 border-b border-black/5 dark:border-white/10 flex items-center justify-between px-6 bg-white/30 dark:bg-black/10 mt-safe">
          <div className="flex-1" />
          <div className="relative w-full md:w-auto">
            <input type="text" placeholder="Search vault..." className="w-full md:w-64 bg-black/5 dark:bg-black/30 border border-black/5 dark:border-white/10 rounded-md py-1.5 pl-3 pr-3 text-sm placeholder-zinc-500 focus:outline-none focus:ring-1 focus:ring-blue-500 transition-all" />
          </div>
        </header>

        <main className="flex-1 overflow-y-auto p-4 md:p-8">
          <div className="max-w-4xl mx-auto space-y-6">
            <div className="flex items-center justify-between mb-8">
              <h2 className="text-2xl font-semibold tracking-tight">{activeTab}</h2>
              <button onClick={handleOpenAddModal} className="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors shadow-sm">
                Add Item
              </button>
            </div>
            
            {activeTab === 'Health' && (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-600 dark:text-red-400">
                  <h3 className="font-semibold text-lg">2 Breached</h3>
                  <p className="text-xs mt-1">Passwords found in data breaches</p>
                </div>
                <div className="p-4 rounded-xl bg-yellow-500/10 border border-yellow-500/20 text-yellow-600 dark:text-yellow-400">
                  <h3 className="font-semibold text-lg">5 Weak</h3>
                  <p className="text-xs mt-1">Passwords easily guessable</p>
                </div>
                <div className="p-4 rounded-xl bg-blue-500/10 border border-blue-500/20 text-blue-600 dark:text-blue-400">
                  <h3 className="font-semibold text-lg">85% Score</h3>
                  <p className="text-xs mt-1">Overall vault health</p>
                </div>
              </div>
            )}

            {activeTab === 'Authenticator' && (
              <div className="text-center py-12 opacity-50">
                <div className="text-4xl mb-4">⏱️</div>
                <p>No TOTP tokens configured yet.</p>
                <p className="text-xs mt-2">Edit a login to add a TOTP secret.</p>
              </div>
            )}

            <div className="grid gap-3">
              {sortedItems.length === 0 ? (
                <div className="text-center py-12 opacity-50">No items found.</div>
              ) : (
                sortedItems.map((item) => (
                  <div key={item.id} className="group p-4 rounded-xl border border-black/5 dark:border-white/10 bg-white/40 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 transition-colors flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                      <div className="w-10 h-10 rounded-full bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center font-semibold text-white shadow-inner">
                        {item.title.charAt(0).toUpperCase()}
                      </div>
                      <div>
                        <div className="flex items-center gap-2">
                          <h3 className="font-medium">{item.title}</h3>
                          {item.pinned && <span className="text-xs text-yellow-500" title="Pinned">📌</span>}
                        </div>
                        <p className="text-sm opacity-60">{item.username}</p>
                      </div>
                    </div>
                    <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                      <button onClick={() => copyToClipboard(item.username || '')} className="text-xs px-2 py-1 rounded border border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/10">Copy User</button>
                      <button onClick={() => copyToClipboard(item.password || '')} className="text-xs px-2 py-1 rounded border border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/10">Copy Pass</button>
                      <button onClick={() => handleTogglePin(item.id)} className="text-xs px-2 py-1 rounded border border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/10">{item.pinned ? 'Unpin' : 'Pin'}</button>
                      <button onClick={() => handleDelete(item.id)} className="text-xs px-2 py-1 rounded border border-red-500/30 text-red-500 hover:bg-red-500/10">Delete</button>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
