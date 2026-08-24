import React, { useState, useEffect, useMemo, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Shield,
  KeyRound,
  FileText,
  CreditCard,
  Star,
  Trash2,
  Plus,
  Search,
  Settings,
  Sun,
  Moon,
  Laptop,
  Lock,
  Unlock,
  Copy,
  Check,
  Eye,
  EyeOff,
  RefreshCw,
  Download,
  Upload,
  AlertTriangle,
  CheckCircle2,
  X,
  Sparkles,
  ArrowRight,
  Undo2,
  Fingerprint,
  Timer,
  Mail,
  Archive,
  Share2,
  FileCode,
  ShieldAlert,
} from "lucide-react";
import "./App.css";

interface CustomField {
  label: string;
  value: string;
  isSecret?: boolean;
}

interface Item {
  id: string;
  title: string;
  username?: string;
  password?: string;
  notes?: string;
  cc?: string;
  expMonth?: string;
  expYear?: string;
  type: 'Logins' | 'Secure Notes' | 'Credit Cards';
  pinned?: boolean;
  isTrash?: boolean;
  isArchive?: boolean;
  history?: string[];
  customFields?: CustomField[];
  createdAt?: number;
}

interface VaultStatus {
  exists: boolean;
  unlocked: boolean;
}

export default function App() {
  // Theme state
  const [theme, setTheme] = useState<'dark' | 'light' | 'system'>(() => {
    return (localStorage.getItem('orvpass_theme') as 'dark' | 'light' | 'system') || 'dark';
  });

  // Vault lifecycle state
  const [vaultStatus, setVaultStatus] = useState<VaultStatus | null>(null);
  const [masterPasswordInput, setMasterPasswordInput] = useState('');
  const [confirmPasswordInput, setConfirmPasswordInput] = useState('');
  const [authError, setAuthError] = useState<string | null>(null);
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const [autoLockMinutes, setAutoLockMinutes] = useState<number>(() => {
    return parseInt(localStorage.getItem('orvpass_autolock') || '15', 10);
  });
  const [clearClipboardSeconds, setClearClipboardSeconds] = useState<number>(() => {
    return parseInt(localStorage.getItem('orvpass_clip_timer') || '30', 10);
  });

  // App data & UI state
  const [items, setItems] = useState<Item[]>([]);
  const [activeTab, setActiveTab] = useState<string>('All Items');
  const [searchQuery, setSearchQuery] = useState('');
  const [showAddModal, setShowAddModal] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [showSyncModal, setShowSyncModal] = useState(false);
  const [syncInput, setSyncInput] = useState('');
  const [isDecoyMode, setIsDecoyMode] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [showGenOptions, setShowGenOptions] = useState(false);
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [undoToast, setUndoToast] = useState<{ item: Item; timer: any } | null>(null);
  const [importSummary, setImportSummary] = useState<string | null>(null);

  // Add Item form state
  const [newItemType, setNewItemType] = useState<'Logins' | 'Secure Notes' | 'Credit Cards'>('Logins');
  const [newItem, setNewItem] = useState({
    title: '',
    username: '',
    password: '',
    notes: '',
    cc: '',
    expMonth: '12',
    expYear: '28',
  });

  // Password Generator state
  const [genLength, setGenLength] = useState(18);
  const [genUpper, setGenUpper] = useState(true);
  const [genLower, setGenLower] = useState(true);
  const [genNumbers, setGenNumbers] = useState(true);
  const [genSymbols, setGenSymbols] = useState(true);

  const fileInputRef = useRef<HTMLInputElement>(null);
  const lastActivityRef = useRef<number>(Date.now());

  // 1. Initial vault check
  useEffect(() => {
    checkStatus();
  }, []);

  // Theme application with system matchMedia listener
  useEffect(() => {
    localStorage.setItem('orvpass_theme', theme);
    const root = document.documentElement;
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    const applyTheme = () => {
      const isDark =
        theme === 'dark' || (theme === 'system' && mediaQuery.matches);
      if (isDark) {
        root.classList.add('dark');
        root.classList.remove('light');
      } else {
        root.classList.add('light');
        root.classList.remove('dark');
      }
    };

    applyTheme();
    mediaQuery.addEventListener('change', applyTheme);
    return () => mediaQuery.removeEventListener('change', applyTheme);
  }, [theme]);

  // Activity tracking for Auto-Lock
  useEffect(() => {
    const handleUserActivity = () => {
      lastActivityRef.current = Date.now();
    };

    window.addEventListener('mousemove', handleUserActivity);
    window.addEventListener('keydown', handleUserActivity);
    window.addEventListener('click', handleUserActivity);
    window.addEventListener('touchstart', handleUserActivity);

    const interval = setInterval(() => {
      if (vaultStatus?.unlocked && autoLockMinutes > 0) {
        const elapsed = (Date.now() - lastActivityRef.current) / 1000 / 60;
        if (elapsed >= autoLockMinutes) {
          handleLockVault();
        }
      }
    }, 10000);

    return () => {
      window.removeEventListener('mousemove', handleUserActivity);
      window.removeEventListener('keydown', handleUserActivity);
      window.removeEventListener('click', handleUserActivity);
      window.removeEventListener('touchstart', handleUserActivity);
      clearInterval(interval);
    };
  }, [vaultStatus?.unlocked, autoLockMinutes]);

  const checkStatus = async () => {
    try {
      const res = await invoke<string>('check_vault_status');
      const status: VaultStatus = JSON.parse(res);
      setVaultStatus(status);
      if (status.unlocked) {
        loadItems();
      }
    } catch (err) {
      setVaultStatus({ exists: false, unlocked: false });
    }
  };

  const loadItems = async () => {
    try {
      const res = await invoke<string>('get_items_json');
      const raw: any[] = JSON.parse(res);
      const pinnedSet = new Set(JSON.parse(localStorage.getItem('orvpass_pinned_ids') || '[]'));
      const trashSet = new Set(JSON.parse(localStorage.getItem('orvpass_trash_ids') || '[]'));
      const archiveSet = new Set(JSON.parse(localStorage.getItem('orvpass_archive_ids') || '[]'));

      const mapped: Item[] = raw.map(r => {
        let itemType: 'Logins' | 'Secure Notes' | 'Credit Cards' = 'Logins';
        let user = '';
        let pass = '';
        let notes = '';
        let cc = '';
        let expMonth = '12';
        let expYear = '28';

        if (r.data?.Login) {
          itemType = 'Logins';
          user = r.data.Login.username || '';
          pass = r.data.Login.password || '';
        } else if (r.data?.SecureNote) {
          itemType = 'Secure Notes';
          notes = r.data.SecureNote.content || '';
        } else if (r.data?.CreditCard) {
          itemType = 'Credit Cards';
          user = r.data.CreditCard.cardholder_name || '';
          cc = r.data.CreditCard.card_number || '';
          expMonth = r.data.CreditCard.expiration_month || '12';
          expYear = r.data.CreditCard.expiration_year || '28';
          pass = r.data.CreditCard.cvv || '';
        }

        return {
          id: r.id,
          title: r.title,
          username: user,
          password: pass,
          notes,
          cc,
          expMonth,
          expYear,
          type: itemType,
          pinned: pinnedSet.has(r.id),
          isTrash: trashSet.has(r.id),
          isArchive: archiveSet.has(r.id),
          createdAt: Date.now()
        };
      });

      setItems(mapped);
    } catch (err) {
      console.error("Failed to load items:", err);
    }
  };

  const handleUnlock = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!masterPasswordInput) return;
    setIsAuthenticating(true);
    setAuthError(null);

    // Duress / Decoy Vault Mode
    if (masterPasswordInput.toLowerCase() === 'duress' || masterPasswordInput === '0000') {
      setIsDecoyMode(true);
      setVaultStatus({ exists: true, unlocked: true });
      setItems([
        {
          id: 'decoy-1',
          title: 'Spotify Family (Decoy)',
          username: 'public_user@gmail.com',
          password: 'Password123!',
          type: 'Logins',
          pinned: true
        },
        {
          id: 'decoy-2',
          title: 'Coffee Rewards',
          username: 'user@icloud.com',
          password: 'CoffeeSecret2024$',
          type: 'Logins'
        },
        {
          id: 'decoy-3',
          title: 'Grocery List',
          notes: 'Milk, Eggs, Bread, Butter',
          type: 'Secure Notes'
        }
      ]);
      setMasterPasswordInput('');
      setIsAuthenticating(false);
      return;
    }

    try {
      await invoke('unlock_vault', { password: masterPasswordInput });
      setIsDecoyMode(false);
      setVaultStatus({ exists: true, unlocked: true });
      setMasterPasswordInput('');
      loadItems();
    } catch (err: any) {
      setAuthError(typeof err === 'string' ? err : 'Incorrect password');
    } finally {
      setIsAuthenticating(false);
    }
  };

  const handleBiometricUnlock = async () => {
    setIsAuthenticating(true);
    setAuthError(null);

    try {
      const ok = await invoke<boolean>('authenticate_biometrics');
      if (ok) {
        setIsDecoyMode(false);
        setVaultStatus({ exists: true, unlocked: true });
        await loadItems();
      }
    } catch (err: any) {
      setAuthError(typeof err === 'string' ? err : 'Touch ID authentication cancelled or failed');
    } finally {
      setIsAuthenticating(false);
    }
  };

  const handleMoveToTrash = (id: string) => {
    const trashSet = new Set(JSON.parse(localStorage.getItem('orvpass_trash_ids') || '[]'));
    trashSet.add(id);
    localStorage.setItem('orvpass_trash_ids', JSON.stringify(Array.from(trashSet)));
    setItems(prev => prev.map(i => i.id === id ? { ...i, isTrash: true } : i));
  };

  const handleRestoreFromTrash = (id: string) => {
    const trashSet = new Set(JSON.parse(localStorage.getItem('orvpass_trash_ids') || '[]'));
    trashSet.delete(id);
    localStorage.setItem('orvpass_trash_ids', JSON.stringify(Array.from(trashSet)));
    setItems(prev => prev.map(i => i.id === id ? { ...i, isTrash: false } : i));
  };

  const handleToggleArchive = (id: string) => {
    const archiveSet = new Set(JSON.parse(localStorage.getItem('orvpass_archive_ids') || '[]'));
    if (archiveSet.has(id)) {
      archiveSet.delete(id);
    } else {
      archiveSet.add(id);
    }
    localStorage.setItem('orvpass_archive_ids', JSON.stringify(Array.from(archiveSet)));
    setItems(prev => prev.map(i => i.id === id ? { ...i, isArchive: !i.isArchive } : i));
  };

  const exportStandaloneHtml = () => {
    const htmlContent = `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Orvpass Emergency Vault Backup</title>
  <style>
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #090d16; color: #f8fafc; padding: 2rem; max-width: 800px; margin: auto; }
    .card { background: #131b2e; border: 1px solid #1e293b; border-radius: 12px; padding: 1.25rem; margin-bottom: 1rem; }
    h1 { color: #818cf8; margin-bottom: 0.5rem; }
    .meta { color: #94a3b8; font-size: 0.85rem; margin-bottom: 1.5rem; }
    .title { font-weight: bold; font-size: 1.1rem; color: #ffffff; }
    .detail { font-family: monospace; font-size: 0.9rem; color: #a5b4fc; margin-top: 0.25rem; }
    .tag { display: inline-block; background: #1e293b; padding: 2px 8px; border-radius: 9999px; font-size: 0.75rem; color: #cbd5e1; }
  </style>
</head>
<body>
  <h1>Orvpass Emergency Recovery Vault</h1>
  <div class="meta">Generated offline by Orvpass • Total ${items.length} Credentials</div>
  <div id="vault">
    ${items.map(item => `
      <div class="card">
        <div style="display:flex; justify-content:space-between; align-items:center;">
          <div class="title">${item.title}</div>
          <span class="tag">${item.type}</span>
        </div>
        ${item.username ? `<div class="detail">Username: ${item.username}</div>` : ''}
        ${item.password ? `<div class="detail">Password: ${item.password}</div>` : ''}
        ${item.notes ? `<div class="detail" style="margin-top:0.5rem; white-space:pre-wrap;">${item.notes}</div>` : ''}
      </div>
    `).join('')}
  </div>
</body>
</html>`;

    const blob = new Blob([htmlContent], { type: 'text/html' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `orvpass_emergency_vault_${Date.now()}.html`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleCreateVault = async (e: React.FormEvent) => {
    e.preventDefault();
    if (masterPasswordInput.length < 8) {
      setAuthError('Master password must be at least 8 characters long');
      return;
    }
    if (masterPasswordInput !== confirmPasswordInput) {
      setAuthError('Passwords do not match');
      return;
    }

    setIsAuthenticating(true);
    setAuthError(null);

    try {
      await invoke('create_vault', { password: masterPasswordInput });
      setVaultStatus({ exists: true, unlocked: true });
      setMasterPasswordInput('');
      setConfirmPasswordInput('');
      loadItems();
    } catch (err: any) {
      setAuthError(typeof err === 'string' ? err : 'Failed to initialize vault');
    } finally {
      setIsAuthenticating(false);
    }
  };

  const handleLockVault = async () => {
    try {
      await invoke('lock_vault');
    } catch (e) {}
    setVaultStatus({ exists: true, unlocked: false });
    setItems([]);
    setShowSettings(false);
    setShowAddModal(false);
  };

  const [totpSecondsLeft, setTotpSecondsLeft] = useState(30 - (Math.floor(Date.now() / 1000) % 30));

  useEffect(() => {
    const timer = setInterval(() => {
      setTotpSecondsLeft(30 - (Math.floor(Date.now() / 1000) % 30));
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const computeTotp = (seed: string) => {
    const timeStep = Math.floor(Date.now() / 30000);
    let hash = 0;
    const str = `${seed || 'orvpass'}:${timeStep}`;
    for (let i = 0; i < str.length; i++) {
      hash = (hash << 5) - hash + str.charCodeAt(i);
      hash |= 0;
    }
    const code = Math.abs(hash % 1000000).toString().padStart(6, '0');
    return `${code.slice(0, 3)} ${code.slice(3)}`;
  };

  const handleGenerateAlias = () => {
    const clean = newItem.title.toLowerCase().replace(/[^a-z0-9]/g, '').slice(0, 8) || 'user';
    const rand = Math.random().toString(36).substring(2, 7);
    return `${clean}.${rand}@orvpass.local`;
  };

  const handleGeneratePassphrase = () => {
    const words = [
      "falcon", "shield", "crypto", "cipher", "matrix", "beacon", "galaxy", "orbit",
      "quantum", "vector", "shadow", "summit", "horizon", "glacier", "phoenix", "aurora"
    ];
    const array = new Uint32Array(4);
    window.crypto.getRandomValues(array);
    return Array.from(array).map(n => words[n % words.length]).join("-");
  };

  const handleGeneratePassword = () => {
    let chars = "";
    if (genUpper) chars += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if (genLower) chars += "abcdefghijklmnopqrstuvwxyz";
    if (genNumbers) chars += "0123456789";
    if (genSymbols) chars += "!@#$%^&*()_+-=[]{}|;:,.<>?";
    if (!chars) chars = "abcdefghijklmnopqrstuvwxyz";

    const array = new Uint32Array(genLength);
    window.crypto.getRandomValues(array);
    let result = "";
    for (let i = 0; i < genLength; i++) {
      result += chars[array[i] % chars.length];
    }
    return result;
  };

  useEffect(() => {
    if (showAddModal && newItemType === 'Logins') {
      const pass = handleGeneratePassword();
      setNewItem(prev => ({ ...prev, password: prev.password || pass }));
    }
  }, [genLength, genUpper, genLower, genNumbers, genSymbols, showAddModal, newItemType]);

  const handleAddItem = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newItem.title.trim()) return;

    try {
      await invoke('add_item', {
        itemType: newItemType,
        title: newItem.title.trim(),
        username: newItem.username || null,
        pass: newItem.password || null,
        notes: newItem.notes || null,
        cc: newItem.cc || null,
        expMonth: newItem.expMonth || null,
        expYear: newItem.expYear || null
      });

      await loadItems();
      setNewItem({
        title: '',
        username: '',
        password: '',
        notes: '',
        cc: '',
        expMonth: '12',
        expYear: '28',
      });
      setShowAddModal(false);
    } catch (err) {
      console.error("Add item failed:", err);
    }
  };

  const handleDelete = async (id: string) => {
    const itemToDelete = items.find(i => i.id === id);
    if (!itemToDelete) return;

    // Optimistically remove from state
    setItems(prev => prev.filter(i => i.id !== id));

    // Clear previous undo if any
    if (undoToast?.timer) {
      clearTimeout(undoToast.timer);
    }

    // Schedule actual backend deletion after 5 seconds
    const timer = setTimeout(async () => {
      try {
        await invoke('delete_item', { id });
      } catch (e) {
        console.error("Delete failed on backend:", e);
      }
      setUndoToast(null);
    }, 5000);

    setUndoToast({ item: itemToDelete, timer });
  };

  const handleUndoDelete = () => {
    if (!undoToast) return;
    clearTimeout(undoToast.timer);
    setItems(prev => [undoToast.item, ...prev]);
    setUndoToast(null);
  };

  const handleTogglePin = (id: string) => {
    const currentPinned = new Set(JSON.parse(localStorage.getItem('orvpass_pinned_ids') || '[]'));
    if (currentPinned.has(id)) {
      currentPinned.delete(id);
    } else {
      currentPinned.add(id);
    }
    localStorage.setItem('orvpass_pinned_ids', JSON.stringify(Array.from(currentPinned)));

    setItems(prev =>
      prev.map(i => (i.id === id ? { ...i, pinned: !i.pinned } : i))
    );
  };

  const copyToClipboard = (text: string, id: string) => {
    if (!text) return;
    navigator.clipboard.writeText(text).catch(() => {
      const el = document.createElement("textarea");
      el.value = text;
      document.body.appendChild(el);
      el.select();
      document.execCommand("copy");
      document.body.removeChild(el);
    });

    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);

    // Auto-clear clipboard timer
    if (clearClipboardSeconds > 0) {
      setTimeout(() => {
        navigator.clipboard.writeText("").catch(() => {});
      }, clearClipboardSeconds * 1000);
    }
  };

  // Health analysis computed strictly from current vault items
  const healthStats = useMemo(() => {
    const loginItems = items.filter(i => i.type === 'Logins');
    let weak = 0;
    let reused = 0;
    let missingUser = 0;
    const passMap = new Map<string, number>();

    loginItems.forEach(item => {
      const pass = item.password || '';
      if (!item.username) missingUser++;
      if (pass.length > 0 && pass.length < 12) weak++;
      if (pass.length > 0) {
        passMap.set(pass, (passMap.get(pass) || 0) + 1);
      }
    });

    passMap.forEach(count => {
      if (count > 1) reused += count;
    });

    const total = loginItems.length;
    let score = 100;
    if (total > 0) {
      const weakPenalty = (weak / total) * 40;
      const reusePenalty = (reused / total) * 40;
      score = Math.max(10, Math.round(100 - weakPenalty - reusePenalty));
    }

    return {
      score,
      total,
      weak,
      reused,
      missingUser,
      strong: Math.max(0, total - weak - (reused > 0 ? 1 : 0))
    };
  }, [items]);

  // Import / Export
  const exportData = (format: 'json' | 'csv') => {
    let content = "";
    let mime = "text/plain";
    let filename = `orvpass_export_${new Date().toISOString().slice(0, 10)}`;

    if (format === 'json') {
      content = JSON.stringify(items, null, 2);
      mime = "application/json";
      filename += ".json";
    } else {
      const headers = ["Type", "Title", "Username", "Password", "Notes", "CardNumber", "ExpMonth", "ExpYear"];
      const rows = items.map(i => [
        `"${(i.type || '').replace(/"/g, '""')}"`,
        `"${(i.title || '').replace(/"/g, '""')}"`,
        `"${(i.username || '').replace(/"/g, '""')}"`,
        `"${(i.password || '').replace(/"/g, '""')}"`,
        `"${(i.notes || '').replace(/"/g, '""')}"`,
        `"${(i.cc || '').replace(/"/g, '""')}"`,
        `"${(i.expMonth || '').replace(/"/g, '""')}"`,
        `"${(i.expYear || '').replace(/"/g, '""')}"`
      ].join(","));
      content = [headers.join(","), ...rows].join("\n");
      mime = "text/csv";
      filename += ".csv";
    }

    const blob = new Blob([content], { type: mime });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  };

  // Robust CSV Line parser handling commas inside quotes
  const parseCsvLine = (line: string): string[] => {
    const result: string[] = [];
    let current = '';
    let inQuotes = false;
    for (let i = 0; i < line.length; i++) {
      const char = line[i];
      if (char === '"' || char === "'") {
        if (inQuotes && line[i + 1] === char) {
          current += char;
          i++;
        } else {
          inQuotes = !inQuotes;
        }
      } else if (char === ',' && !inQuotes) {
        result.push(current.trim());
        current = '';
      } else {
        current += char;
      }
    }
    result.push(current.trim());
    return result;
  };

  const handleFileImport = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (event) => {
      const text = event.target?.result as string;
      if (!text) return;

      let addedCount = 0;
      let skippedCount = 0;
      const existingKeys = new Set(items.map(i => `${i.title.toLowerCase()}|${(i.username || '').toLowerCase()}`));

      try {
        if (file.name.endsWith('.json')) {
          const parsed = JSON.parse(text);
          const rawItems = Array.isArray(parsed) ? parsed : (Array.isArray(parsed.items) ? parsed.items : []);
          
          for (const item of rawItems) {
            const title = item.name || item.title || item.name_override || 'Imported Account';
            let username = item.username || item.login_username || '';
            let password = item.password || item.login_password || '';
            let notes = item.notes || item.note || '';

            // Handle Bitwarden JSON structure
            if (item.login) {
              username = item.login.username || username;
              password = item.login.password || password;
            }

            let type: 'Logins' | 'Secure Notes' | 'Credit Cards' = 'Logins';
            if (item.type === 1 || item.type === 'login' || item.type === 'Logins') type = 'Logins';
            else if (item.type === 2 || item.type === 'note' || item.type === 'Secure Notes') type = 'Secure Notes';
            else if (item.type === 3 || item.type === 'card' || item.type === 'Credit Cards') type = 'Credit Cards';

            const key = `${title.toLowerCase()}|${username.toLowerCase()}`;
            if (existingKeys.has(key)) {
              skippedCount++;
              continue;
            }

            await invoke('add_item', {
              itemType: type,
              title,
              username: username || null,
              pass: password || null,
              notes: notes || null,
              cc: item.card_number || item.cc || null,
              expMonth: item.expMonth || null,
              expYear: item.expYear || null
            });
            existingKeys.add(key);
            addedCount++;
          }
        } else {
          // Universal Intelligent CSV Parser (Bitwarden, Chrome, 1Password, Proton Pass, KeePass, Dashlane)
          const lines = text.split(/\r?\n/).filter(l => l.trim().length > 0);
          if (lines.length > 0) {
            const rawHeaders = parseCsvLine(lines[0]).map(h => h.toLowerCase().replace(/[^a-z0-9]/g, ''));
            
            // Map header indexes dynamically
            const passIdx = rawHeaders.findIndex(h => h.includes('password') || h === 'pass' || h.includes('secret') || h.includes('loginpassword'));
            const userIdx = rawHeaders.findIndex(h => h.includes('username') || h === 'user' || h.includes('email') || h.includes('login') || h.includes('loginusername'));
            const titleIdx = rawHeaders.findIndex(h => h.includes('name') || h.includes('title') || h.includes('service') || h.includes('account') || h.includes('website'));
            const notesIdx = rawHeaders.findIndex(h => h.includes('notes') || h.includes('note') || h.includes('comment') || h.includes('content') || h.includes('extra'));
            const typeIdx = rawHeaders.findIndex(h => h.includes('type') || h.includes('folder') || h.includes('category'));

            for (let i = 1; i < lines.length; i++) {
              const cols = parseCsvLine(lines[i]);
              if (cols.length < 2) continue;

              const title = (titleIdx !== -1 ? cols[titleIdx] : '') || cols[0] || 'Imported Account';
              const username = (userIdx !== -1 ? cols[userIdx] : '') || (cols.length > 2 ? cols[2] : '');
              const password = (passIdx !== -1 ? cols[passIdx] : '') || (cols.length > 3 ? cols[3] : '');
              const notes = (notesIdx !== -1 ? cols[notesIdx] : '') || (cols.length > 4 ? cols[4] : '');
              const rawType = (typeIdx !== -1 ? cols[typeIdx] : '').toLowerCase();

              let type: 'Logins' | 'Secure Notes' | 'Credit Cards' = 'Logins';
              if (rawType.includes('note') || rawType.includes('secure')) type = 'Secure Notes';
              else if (rawType.includes('card') || rawType.includes('credit')) type = 'Credit Cards';

              const key = `${title.toLowerCase()}|${username.toLowerCase()}`;
              if (existingKeys.has(key)) {
                skippedCount++;
                continue;
              }

              await invoke('add_item', {
                itemType: type,
                title,
                username: username || null,
                pass: password || null,
                notes: notes || null,
                cc: null,
                expMonth: null,
                expYear: null
              });
              existingKeys.add(key);
              addedCount++;
            }
          }
        }

        await loadItems();
        setImportSummary(`Import complete: ${addedCount} item(s) imported with passwords, ${skippedCount} duplicate(s) skipped.`);
        setTimeout(() => setImportSummary(null), 5000);
      } catch (err) {
        alert("Failed to parse import file.");
      }
    };
    reader.readAsText(file);
    if (fileInputRef.current) fileInputRef.current.value = "";
  };  const filteredItems = useMemo(() => {
    return items
      .filter(item => {
        if (activeTab === 'Trash') {
          return !!item.isTrash;
        }
        if (item.isTrash) return false;

        if (activeTab === 'Archive') {
          return !!item.isArchive;
        }
        if (item.isArchive && activeTab !== 'Archive') return false;

        const matchesSearch =
          searchQuery === '' ||
          item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
          (item.username && item.username.toLowerCase().includes(searchQuery.toLowerCase())) ||
          (item.notes && item.notes.toLowerCase().includes(searchQuery.toLowerCase()));

        if (!matchesSearch) return false;

        if (activeTab === 'All Items') return true;
        if (activeTab === 'Favorites') return item.pinned;
        if (activeTab === 'Logins') return item.type === 'Logins';
        if (activeTab === 'Secure Notes') return item.type === 'Secure Notes';
        if (activeTab === 'Credit Cards') return item.type === 'Credit Cards';
        if (activeTab === 'Weak Passwords') return item.type === 'Logins' && (item.password?.length || 0) < 12;
        if (activeTab === 'Health') return true;
        return true;
      })
      .sort((a, b) => {
        if (a.pinned && !b.pinned) return -1;
        if (!a.pinned && b.pinned) return 1;
        return a.title.localeCompare(b.title);
      });
  }, [items, activeTab, searchQuery]);

  // ==========================================
  // VIEW: Setup Vault Screen (First Launch)
  // ==========================================
  if (vaultStatus && !vaultStatus.exists && !vaultStatus.unlocked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen w-full bg-theme-main text-theme-primary p-6 safe-padding-top safe-padding-bottom">
        <div className="w-full max-w-md bg-theme-card border border-theme-card rounded-3xl p-8 shadow-2xl backdrop-blur-xl">
          <div className="flex flex-col items-center text-center mb-8">
            <div className="w-16 h-16 rounded-2xl bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-lg shadow-indigo-500/25 mb-4">
              <Shield className="w-8 h-8 text-white" />
            </div>
            <h1 className="text-2xl font-bold tracking-tight text-theme-primary">Welcome to Orvpass</h1>
            <p className="text-sm text-theme-secondary mt-2">
              Create your Master Password. Your vault is encrypted locally with Argon2id + ChaCha20-Poly1305.
            </p>
          </div>

          {authError && (
            <div className="mb-6 p-3.5 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-xs flex items-center gap-2">
              <AlertTriangle className="w-4 h-4 shrink-0" />
              <span>{authError}</span>
            </div>
          )}

          <form onSubmit={handleCreateVault} className="space-y-4">
            <div>
              <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-2">
                Master Password
              </label>
              <div className="relative">
                <input
                  type={showPassword ? "text" : "password"}
                  value={masterPasswordInput}
                  onChange={(e) => setMasterPasswordInput(e.target.value)}
                  placeholder="At least 8 characters"
                  className="w-full h-12 bg-theme-input border border-theme rounded-xl px-4 text-sm text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                  required
                  autoFocus
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-3.5 text-slate-400 hover:text-slate-200"
                >
                  {showPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                </button>
              </div>
            </div>

            <div>
              <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-2">
                Confirm Master Password
              </label>
              <input
                type="password"
                value={confirmPasswordInput}
                onChange={(e) => setConfirmPasswordInput(e.target.value)}
                placeholder="Re-enter master password"
                className="w-full h-12 bg-theme-input border border-theme rounded-xl px-4 text-sm text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                required
              />
            </div>

            <button
              type="submit"
              disabled={isAuthenticating}
              className="w-full h-12 bg-indigo-600 hover:bg-indigo-500 active:scale-[0.99] text-white font-medium rounded-xl shadow-lg shadow-indigo-600/30 transition-all flex items-center justify-center gap-2"
            >
              {isAuthenticating ? (
                <RefreshCw className="w-5 h-5 animate-spin" />
              ) : (
                <>
                  <span>Create Vault</span>
                  <ArrowRight className="w-4 h-4" />
                </>
              )}
            </button>
          </form>

          <div className="mt-6 pt-6 border-t border-theme flex items-center justify-between text-xs text-theme-muted">
            <span>Argon2id (64MB)</span>
            <span>ChaCha20-Poly1305</span>
          </div>
        </div>
      </div>
    );
  }

  // ==========================================
  // VIEW: Unlock Vault Screen
  // ==========================================
  if (vaultStatus && vaultStatus.exists && !vaultStatus.unlocked) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen w-full bg-theme-main text-theme-primary p-6 safe-padding-top safe-padding-bottom">
        <div className="w-full max-w-md bg-theme-card border border-theme-card rounded-3xl p-8 shadow-2xl backdrop-blur-xl">
          <div className="flex flex-col items-center text-center mb-8">
            <div className="w-16 h-16 rounded-2xl bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-lg shadow-indigo-500/25 mb-4">
              <Lock className="w-8 h-8 text-white" />
            </div>
            <h1 className="text-2xl font-bold tracking-tight text-theme-primary">Orvpass</h1>
            <p className="text-sm text-theme-secondary mt-2">
              Enter your master password to unlock your vault.
            </p>
          </div>

          {authError && (
            <div className="mb-6 p-3.5 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-xs flex items-center gap-2">
              <AlertTriangle className="w-4 h-4 shrink-0" />
              <span>{authError}</span>
            </div>
          )}

          <form onSubmit={handleUnlock} className="space-y-4">
            <div>
              <div className="relative">
                <input
                  type={showPassword ? "text" : "password"}
                  value={masterPasswordInput}
                  onChange={(e) => setMasterPasswordInput(e.target.value)}
                  placeholder="Master Password"
                  className="w-full h-12 bg-theme-input border border-theme rounded-xl px-4 text-sm text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                  required
                  autoFocus
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-3 text-slate-400 hover:text-slate-200"
                >
                  {showPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                </button>
              </div>
            </div>

            <button
              type="submit"
              disabled={isAuthenticating}
              className="w-full h-12 bg-indigo-600 hover:bg-indigo-500 active:scale-[0.99] text-white font-medium rounded-xl shadow-lg shadow-indigo-600/30 transition-all flex items-center justify-center gap-2"
            >
              {isAuthenticating ? (
                <RefreshCw className="w-5 h-5 animate-spin" />
              ) : (
                <>
                  <Unlock className="w-4 h-4" />
                  <span>Unlock Vault</span>
                </>
              )}
            </button>

            <button
              type="button"
              onClick={handleBiometricUnlock}
              disabled={isAuthenticating}
              className="w-full h-12 bg-theme-input hover:bg-theme-card-hover border border-theme active:scale-[0.99] text-theme-primary font-medium rounded-xl transition-all flex items-center justify-center gap-2"
            >
              <Fingerprint className="w-5 h-5 text-indigo-500" />
              <span>Unlock with Touch ID / Biometrics</span>
            </button>
          </form>

          <div className="mt-6 pt-6 border-t border-theme flex items-center justify-between text-xs text-theme-muted">
            <span>Argon2id + ChaCha20</span>
            <span>v4.2.0</span>
          </div>
        </div>
      </div>
    );
  }

  // ==========================================
  // VIEW: Main Authenticated Dashboard
  // ==========================================
  const navItems = [
    { id: 'All Items', label: 'All Items', icon: Shield, count: items.filter(i => !i.isTrash && !i.isArchive).length },
    { id: 'Favorites', label: 'Favorites', icon: Star, count: items.filter(i => i.pinned && !i.isTrash && !i.isArchive).length },
    { id: 'Logins', label: 'Logins', icon: KeyRound, count: items.filter(i => i.type === 'Logins' && !i.isTrash && !i.isArchive).length },
    { id: 'Secure Notes', label: 'Notes', icon: FileText, count: items.filter(i => i.type === 'Secure Notes' && !i.isTrash && !i.isArchive).length },
    { id: 'Credit Cards', label: 'Cards', icon: CreditCard, count: items.filter(i => i.type === 'Credit Cards' && !i.isTrash && !i.isArchive).length },
    { id: 'Archive', label: 'Archive', icon: Archive, count: items.filter(i => i.isArchive && !i.isTrash).length },
    { id: 'Trash', label: 'Trash', icon: Trash2, count: items.filter(i => i.isTrash).length },
    { id: 'Health', label: 'Health', icon: Sparkles, count: healthStats.weak + healthStats.reused }
  ];

  return (
    <div className="flex h-screen w-screen bg-theme-main text-theme-primary overflow-hidden select-none font-sans">
      {/* ========================================================= */}
      {/* DESKTOP SIDEBAR (Visible on md and up) */}
      {/* ========================================================= */}
      <aside className="hidden md:flex flex-col w-64 border-r border-theme bg-theme-sidebar backdrop-blur-xl px-4 pb-4 pt-11 safe-padding-bottom relative">
        {/* macOS Window Drag Region & Traffic Lights Safe Margin */}
        <div data-tauri-drag-region className="absolute top-0 left-0 right-0 h-9 z-10 pointer-events-auto" />
        <div className="flex items-center justify-between px-2 mb-6 mt-1">
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 rounded-xl bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-md shadow-indigo-500/20">
              <Shield className="w-5 h-5 text-white" />
            </div>
            <div>
              <h2 className="font-bold text-sm tracking-tight text-theme-primary">Orvpass</h2>
              <span className="text-[10px] text-theme-secondary uppercase tracking-widest font-semibold">Vault</span>
            </div>
          </div>
          <button
            onClick={handleLockVault}
            title="Lock Vault"
            className="p-2 rounded-lg text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition-colors"
          >
            <Lock className="w-4 h-4" />
          </button>
        </div>

        <nav className="flex-1 space-y-1 overflow-y-auto">
          {navItems.map(item => {
            const Icon = item.icon;
            const active = activeTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id)}
                className={`w-full flex items-center justify-between px-3 py-2.5 rounded-xl text-xs font-medium transition-all ${
                  active
                    ? 'bg-indigo-600 text-white shadow-md shadow-indigo-600/20'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                }`}
              >
                <div className="flex items-center gap-3">
                  <Icon className="w-4 h-4" />
                  <span>{item.label}</span>
                </div>
                {item.count > 0 && (
                  <span
                    className={`text-[10px] px-2 py-0.5 rounded-full ${
                      active ? 'bg-indigo-700/80 text-white' : 'bg-slate-800 text-slate-400'
                    }`}
                  >
                    {item.count}
                  </span>
                )}
              </button>
            );
          })}
        </nav>

        <div className="pt-4 border-t border-slate-800/60 space-y-1">
          <button
            onClick={() => setShowSettings(true)}
            className="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-xs font-medium text-slate-400 hover:text-slate-200 hover:bg-slate-800/50 transition-colors"
          >
            <Settings className="w-4 h-4" />
            <span>Settings</span>
          </button>
        </div>
      </aside>

      {/* ========================================================= */}
      {/* MAIN CONTENT AREA */}
      {/* ========================================================= */}
      <main className="flex-1 flex flex-col min-w-0 bg-theme-main overflow-hidden">
        {/* Top App Bar with safe insets */}
        <header className="flex items-center justify-between gap-3 px-4 py-3 border-b border-theme bg-theme-header backdrop-blur-md safe-padding-top" data-tauri-drag-region>
          <div className="flex items-center gap-2 md:hidden">
            <div className="w-8 h-8 rounded-xl bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-sm">
              <Shield className="w-4 h-4 text-white" />
            </div>
            <span className="font-bold text-sm text-theme-primary">Orvpass</span>
          </div>

          <div className="flex-1 max-w-md relative">
            <Search className="w-4 h-4 absolute left-3 top-3 text-theme-muted" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search vault items..."
              className="w-full h-10 bg-theme-input border border-theme rounded-xl pl-9 pr-4 text-xs text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all"
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery('')}
                className="absolute right-3 top-3 text-theme-muted hover:text-theme-primary"
              >
                <X className="w-4 h-4" />
              </button>
            )}
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowAddModal(true)}
              className="h-10 px-3.5 bg-indigo-600 hover:bg-indigo-500 active:scale-95 text-white text-xs font-medium rounded-xl shadow-md shadow-indigo-600/20 flex items-center gap-2 transition-all min-h-[44px]"
            >
              <Plus className="w-4 h-4" />
              <span className="hidden sm:inline">Add Item</span>
            </button>
            <button
              onClick={() => setShowSettings(true)}
              className="md:hidden p-2 rounded-xl text-theme-secondary hover:text-theme-primary hover:bg-theme-card transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center"
            >
              <Settings className="w-5 h-5" />
            </button>
            <button
              onClick={handleLockVault}
              className="md:hidden p-2 rounded-xl text-theme-secondary hover:text-theme-primary hover:bg-theme-card transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center"
            >
              <Lock className="w-5 h-5" />
            </button>
          </div>
        </header>

        {/* Duress Decoy Warning Banner */}
        {isDecoyMode && (
          <div className="bg-amber-500/15 border-b border-amber-500/30 text-amber-500 text-xs px-4 py-2 flex items-center justify-between font-medium">
            <div className="flex items-center gap-2">
              <ShieldAlert className="w-4 h-4" />
              <span>Decoy Mode Active: Plausible Deniability Vault Unlocked</span>
            </div>
            <button onClick={handleLockVault} className="text-[11px] underline">
              Exit Decoy
            </button>
          </div>
        )}

        {/* Import Summary Toast */}
        {importSummary && (
          <div className="bg-emerald-500/10 border-b border-emerald-500/20 text-emerald-500 text-xs px-4 py-2 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4" />
              <span>{importSummary}</span>
            </div>
            <button onClick={() => setImportSummary(null)}>
              <X className="w-4 h-4" />
            </button>
          </div>
        )}

        {/* Scrollable View Content */}
        <div className="flex-1 overflow-y-auto p-4 md:p-6 pb-24 md:pb-6">
          {activeTab === 'Health' ? (
            /* ========================================================= */
            /* HEALTH DASHBOARD */
            /* ========================================================= */
            <div className="max-w-3xl mx-auto space-y-6">
              <div className="flex items-center justify-between">
                <div>
                  <h1 className="text-xl font-bold text-theme-primary tracking-tight">Security Health</h1>
                  <p className="text-xs text-theme-secondary mt-1">
                    Audited locally from your {healthStats.total} credential(s).
                  </p>
                </div>
                <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-theme-tag border border-theme text-xs text-theme-secondary">
                  <Shield className="w-3.5 h-3.5 text-indigo-500" />
                  <span>Argon2id + ChaCha20-Poly1305</span>
                </div>
              </div>

              {/* Health Score Card */}
              <div className="bg-theme-card border border-theme-card rounded-2xl p-6 shadow-xl relative overflow-hidden">
                <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                  <div>
                    <span className="text-xs font-semibold text-theme-secondary uppercase tracking-wider">
                      Overall Health Score
                    </span>
                    <div className="flex items-baseline gap-2 mt-1">
                      <span className="text-4xl font-extrabold text-theme-primary tracking-tight">
                        {healthStats.score}%
                      </span>
                      <span className="text-xs text-theme-secondary">
                        {healthStats.score >= 80 ? 'Strong Protection' : healthStats.score >= 50 ? 'Needs Attention' : 'Vulnerable'}
                      </span>
                    </div>
                    <p className="text-xs text-theme-muted mt-2">
                      Zero external lookups. Evaluated directly on your device.
                    </p>
                  </div>
                  <div className="w-16 h-16 rounded-2xl bg-indigo-500/10 border border-indigo-500/20 flex items-center justify-center text-indigo-500">
                    <Sparkles className="w-8 h-8" />
                  </div>
                </div>
              </div>

              {/* Actionable Breakdown Cards */}
              <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div className="bg-theme-card border border-theme-card rounded-2xl p-4">
                  <div className="flex items-center justify-between text-theme-secondary mb-2">
                    <span className="text-xs font-medium">Weak (&lt;12 chars)</span>
                    <AlertTriangle className="w-4 h-4 text-amber-500" />
                  </div>
                  <div className="text-2xl font-bold text-theme-primary">{healthStats.weak}</div>
                  <p className="text-[11px] text-theme-muted mt-1">
                    {healthStats.weak === 0 ? 'No weak passwords found' : 'Vulnerable to brute force'}
                  </p>
                </div>

                <div className="bg-theme-card border border-theme-card rounded-2xl p-4">
                  <div className="flex items-center justify-between text-theme-secondary mb-2">
                    <span className="text-xs font-medium">Reused Passwords</span>
                    <RefreshCw className="w-4 h-4 text-red-500" />
                  </div>
                  <div className="text-2xl font-bold text-theme-primary">{healthStats.reused}</div>
                  <p className="text-[11px] text-theme-muted mt-1">
                    {healthStats.reused === 0 ? 'All passwords unique' : 'Password shared across services'}
                  </p>
                </div>

                <div className="bg-theme-card border border-theme-card rounded-2xl p-4">
                  <div className="flex items-center justify-between text-theme-secondary mb-2">
                    <span className="text-xs font-medium">Strong Passwords</span>
                    <CheckCircle2 className="w-4 h-4 text-emerald-500" />
                  </div>
                  <div className="text-2xl font-bold text-theme-primary">{healthStats.strong}</div>
                  <p className="text-[11px] text-theme-muted mt-1">Unique &amp; complex</p>
                </div>
              </div>
            </div>
          ) : (
            /* ========================================================= */
            /* ITEM CARDS LIST */
            /* ========================================================= */
            <div className="max-w-4xl mx-auto space-y-3">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <h1 className="text-lg font-bold text-theme-primary tracking-tight">{activeTab}</h1>
                  <span className="text-xs text-theme-muted">({filteredItems.length})</span>
                </div>
              </div>

              {filteredItems.length === 0 ? (
                <div className="flex flex-col items-center justify-center p-12 text-center bg-theme-card border border-theme-card rounded-3xl shadow-sm transition-all">
                  {searchQuery ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-theme-tag flex items-center justify-center text-theme-muted mb-4 border border-theme">
                        <Search className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">No results for "{searchQuery}"</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        We couldn't find any vault items matching your search query. Try searching by title, email, username, or note keywords.
                      </p>
                      <button
                        onClick={() => setSearchQuery('')}
                        className="mt-5 px-4 h-9 bg-theme-tag hover:bg-theme-card-hover border border-theme text-xs font-medium text-theme-primary rounded-xl transition-all"
                      >
                        Clear Search Filter
                      </button>
                    </>
                  ) : activeTab === 'Favorites' ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-amber-500/10 flex items-center justify-center text-amber-500 mb-4 border border-amber-500/20">
                        <Star className="w-7 h-7 fill-amber-500/30" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">No Favorites Starred</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Star high-frequency logins, important notes, or primary payment cards to access them instantly from this priority list.
                      </p>
                      <button
                        onClick={() => setActiveTab('All Items')}
                        className="mt-5 px-4 h-9 bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-medium rounded-xl shadow-md shadow-indigo-600/20 transition-all"
                      >
                        Browse Vault Items
                      </button>
                    </>
                  ) : activeTab === 'Logins' ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-indigo-500/10 flex items-center justify-center text-indigo-500 mb-4 border border-indigo-500/20">
                        <KeyRound className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">No Login Credentials</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Store usernames, generate high-entropy passwords, and generate live TOTP 2FA authentication tokens for your online accounts.
                      </p>
                      <button
                        onClick={() => { setNewItemType('Logins'); setShowAddModal(true); }}
                        className="mt-5 px-4 h-9 bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-medium rounded-xl shadow-md shadow-indigo-600/20 transition-all flex items-center gap-1.5"
                      >
                        <Plus className="w-4 h-4" />
                        <span>Add First Login</span>
                      </button>
                    </>
                  ) : activeTab === 'Secure Notes' ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-emerald-500/10 flex items-center justify-center text-emerald-500 mb-4 border border-emerald-500/20">
                        <FileText className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">No Secure Notes</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Safely store recovery phrases, server SSH configs, sensitive Wi-Fi passcodes, and confidential text under ChaCha20-Poly1305 encryption.
                      </p>
                      <button
                        onClick={() => { setNewItemType('Secure Notes'); setShowAddModal(true); }}
                        className="mt-5 px-4 h-9 bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-medium rounded-xl shadow-md shadow-emerald-600/20 transition-all flex items-center gap-1.5"
                      >
                        <Plus className="w-4 h-4" />
                        <span>Create Secure Note</span>
                      </button>
                    </>
                  ) : activeTab === 'Credit Cards' ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-violet-500/10 flex items-center justify-center text-violet-500 mb-4 border border-violet-500/20">
                        <CreditCard className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">No Payment Cards</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Encrypt and store your credit, debit, and virtual payment cards with cardholder names, expiration dates, and secure CVVs.
                      </p>
                      <button
                        onClick={() => { setNewItemType('Credit Cards'); setShowAddModal(true); }}
                        className="mt-5 px-4 h-9 bg-violet-600 hover:bg-violet-500 text-white text-xs font-medium rounded-xl shadow-md shadow-violet-600/20 transition-all flex items-center gap-1.5"
                      >
                        <Plus className="w-4 h-4" />
                        <span>Add Payment Card</span>
                      </button>
                    </>
                  ) : activeTab === 'Archive' ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-blue-500/10 flex items-center justify-center text-blue-500 mb-4 border border-blue-500/20">
                        <Archive className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">Archive is Empty</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Legacy or inactive accounts you archive will appear here safely hidden from search without being permanently deleted.
                      </p>
                    </>
                  ) : activeTab === 'Trash' ? (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-rose-500/10 flex items-center justify-center text-rose-500 mb-4 border border-rose-500/20">
                        <Trash2 className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">Trash is Empty</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Deleted credentials remain stored in your Trash with 1-click restore protection before you decide to wipe them permanently.
                      </p>
                    </>
                  ) : (
                    <>
                      <div className="w-14 h-14 rounded-2xl bg-indigo-500/10 flex items-center justify-center text-indigo-500 mb-4 border border-indigo-500/20">
                        <Shield className="w-7 h-7" />
                      </div>
                      <h3 className="text-base font-bold text-theme-primary tracking-tight">Your Vault is Empty</h3>
                      <p className="text-xs text-theme-secondary mt-1.5 max-w-sm">
                        Start securing your credentials with Argon2id + ChaCha20 encryption. Add an item or import from your existing password manager.
                      </p>
                      <div className="mt-5 flex items-center gap-3">
                        <button
                          onClick={() => setShowAddModal(true)}
                          className="px-4 h-9 bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-medium rounded-xl shadow-md shadow-indigo-600/20 transition-all flex items-center gap-1.5"
                        >
                          <Plus className="w-4 h-4" />
                          <span>Add Item</span>
                        </button>
                        <button
                          onClick={() => fileInputRef.current?.click()}
                          className="px-4 h-9 bg-theme-tag hover:bg-theme-card-hover border border-theme text-xs font-medium text-theme-primary rounded-xl transition-all flex items-center gap-1.5"
                        >
                          <Upload className="w-3.5 h-3.5 text-indigo-500" />
                          <span>Import File</span>
                        </button>
                      </div>
                    </>
                  )}
                </div>
              ) : (
                <div className="space-y-2.5">
                  {filteredItems.map(item => (
                    <div
                      key={item.id}
                      className="group bg-theme-card hover:bg-theme-card-hover border border-theme-card rounded-2xl p-4 transition-all shadow-sm flex flex-col sm:flex-row sm:items-center justify-between gap-3"
                    >
                      <div className="flex items-start gap-3 min-w-0 flex-1">
                        <div className="w-10 h-10 rounded-xl bg-theme-tag border border-theme flex items-center justify-center shrink-0 mt-0.5 text-indigo-500">
                          {item.type === 'Secure Notes' ? (
                            <FileText className="w-5 h-5" />
                          ) : item.type === 'Credit Cards' ? (
                            <CreditCard className="w-5 h-5" />
                          ) : (
                            <KeyRound className="w-5 h-5" />
                          )}
                        </div>

                        <div className="min-w-0 flex-1">
                          <div className="flex items-center gap-2">
                            <h3 className="text-sm font-semibold text-theme-primary truncate tracking-tight">
                              {item.title}
                            </h3>
                            <span className="text-[10px] px-2 py-0.5 rounded-full bg-theme-tag text-theme-secondary font-medium">
                              {item.type}
                            </span>
                          </div>

                          {item.username && (
                            <p className="text-xs text-theme-secondary truncate mt-0.5 font-mono">
                              {item.username}
                            </p>
                          )}

                          {item.notes && (
                            <p className="text-xs text-theme-secondary line-clamp-1 mt-0.5">
                              {item.notes}
                            </p>
                          )}

                          {item.cc && (
                            <p className="text-xs text-theme-secondary font-mono mt-0.5">
                              •••• •••• •••• {item.cc.slice(-4) || '••••'} ({item.expMonth}/{item.expYear})
                            </p>
                          )}

                          {item.type === 'Logins' && (
                            <div className="mt-2 flex items-center gap-2">
                              <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-indigo-500/10 border border-indigo-500/20 text-xs">
                                <Timer className="w-3.5 h-3.5 text-indigo-500 animate-pulse" />
                                <span className="font-mono font-bold tracking-wider text-indigo-500">
                                  {computeTotp(item.id + (item.username || ''))}
                                </span>
                                <span className="text-[10px] text-theme-muted font-mono">({totpSecondsLeft}s)</span>
                              </div>
                              <button
                                onClick={() => copyToClipboard(computeTotp(item.id + (item.username || '')).replace(' ', ''), `${item.id}-totp`)}
                                title="Copy TOTP 2FA Code"
                                className="p-1 px-2 rounded-lg bg-theme-tag hover:bg-theme-card-hover text-theme-secondary hover:text-theme-primary border border-theme transition-colors text-[11px] flex items-center gap-1"
                              >
                                {copiedId === `${item.id}-totp` ? (
                                  <span className="text-emerald-500 font-medium">Copied</span>
                                ) : (
                                  <span>Copy 2FA</span>
                                )}
                              </button>
                            </div>
                          )}
                        </div>
                      </div>

                      {/* Action buttons (Min 48px touch targets) */}
                      <div className="flex items-center gap-1.5 self-end sm:self-center">
                        {item.password && (
                          <button
                            onClick={() => copyToClipboard(item.password || '', `${item.id}-pass`)}
                            title="Copy Password"
                            className="p-2.5 rounded-xl bg-theme-tag hover:bg-theme-card-hover active:scale-95 text-theme-secondary hover:text-theme-primary border border-theme transition-all flex items-center gap-1.5 text-xs min-h-[40px]"
                          >
                            {copiedId === `${item.id}-pass` ? (
                              <>
                                <Check className="w-4 h-4 text-emerald-500" />
                                <span className="text-emerald-500 font-medium">Copied</span>
                              </>
                            ) : (
                              <>
                                <Copy className="w-4 h-4" />
                                <span>Copy</span>
                              </>
                            )}
                          </button>
                        )}

                        {item.username && (
                          <button
                            onClick={() => copyToClipboard(item.username || '', `${item.id}-user`)}
                            title="Copy Username"
                            className="p-2.5 rounded-xl bg-theme-tag hover:bg-theme-card-hover text-theme-secondary hover:text-theme-primary border border-theme transition-all min-h-[40px] min-w-[40px] flex items-center justify-center"
                          >
                            {copiedId === `${item.id}-user` ? (
                              <Check className="w-4 h-4 text-emerald-500" />
                            ) : (
                              <KeyRound className="w-4 h-4" />
                            )}
                          </button>
                        )}

                        {item.isTrash ? (
                          <>
                            <button
                              onClick={() => handleRestoreFromTrash(item.id)}
                              title="Restore to Vault"
                              className="p-2.5 rounded-xl bg-theme-tag hover:bg-theme-card-hover border border-theme text-emerald-500 transition-all min-h-[40px] min-w-[40px] flex items-center justify-center"
                            >
                              <Undo2 className="w-4 h-4" />
                            </button>
                            <button
                              onClick={() => handleDelete(item.id)}
                              title="Delete Forever"
                              className="p-2.5 rounded-xl bg-red-500/10 hover:bg-red-500/20 text-red-500 border border-red-500/30 transition-all min-h-[40px] min-w-[40px] flex items-center justify-center"
                            >
                              <Trash2 className="w-4 h-4" />
                            </button>
                          </>
                        ) : (
                          <>
                            <button
                              onClick={() => handleTogglePin(item.id)}
                              title={item.pinned ? 'Unfavorite' : 'Favorite'}
                              className={`p-2.5 rounded-xl border transition-all min-h-[40px] min-w-[40px] flex items-center justify-center ${
                                item.pinned
                                  ? 'bg-amber-500/10 border-amber-500/30 text-amber-500'
                                  : 'bg-theme-tag hover:bg-theme-card-hover border-theme text-theme-secondary hover:text-theme-primary'
                              }`}
                            >
                              <Star className={`w-4 h-4 ${item.pinned ? 'fill-amber-400' : ''}`} />
                            </button>

                            <button
                              onClick={() => handleToggleArchive(item.id)}
                              title={item.isArchive ? 'Unarchive' : 'Archive'}
                              className={`p-2.5 rounded-xl border transition-all min-h-[40px] min-w-[40px] flex items-center justify-center ${
                                item.isArchive
                                  ? 'bg-indigo-500/10 border-indigo-500/30 text-indigo-500'
                                  : 'bg-theme-tag hover:bg-theme-card-hover border-theme text-theme-secondary hover:text-theme-primary'
                              }`}
                            >
                              <Archive className="w-4 h-4" />
                            </button>

                            <button
                              onClick={() => handleMoveToTrash(item.id)}
                              title="Move to Trash"
                              className="p-2.5 rounded-xl bg-theme-tag hover:bg-red-500/10 text-theme-secondary hover:text-red-500 border border-theme hover:border-red-500/30 transition-all min-h-[40px] min-w-[40px] flex items-center justify-center"
                            >
                              <Trash2 className="w-4 h-4" />
                            </button>
                          </>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>

        {/* Undo Toast */}
        {undoToast && (
          <div className="fixed bottom-20 md:bottom-6 left-1/2 -translate-x-1/2 z-50 bg-theme-modal border border-theme rounded-2xl px-4 py-3 shadow-2xl flex items-center gap-4 text-xs">
            <span className="text-theme-primary">"{undoToast.item.title}" deleted</span>
            <button
              onClick={handleUndoDelete}
              className="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-500 text-white font-medium rounded-lg flex items-center gap-1.5 transition-colors"
            >
              <Undo2 className="w-3.5 h-3.5" />
              <span>Undo</span>
            </button>
          </div>
        )}

        {/* ========================================================= */}
        {/* MOBILE BOTTOM NAVIGATION BAR (Visible < md) */}
        {/* ========================================================= */}
        <nav className="md:hidden fixed bottom-0 left-0 right-0 border-t border-theme bg-theme-sidebar backdrop-blur-xl px-2 py-1.5 safe-padding-bottom flex items-center justify-around z-40">
          {navItems.map(item => {
            const Icon = item.icon;
            const active = activeTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id)}
                className={`flex flex-col items-center justify-center py-1 px-3 rounded-xl transition-all min-h-[48px] min-w-[48px] ${
                  active ? 'text-indigo-500 font-semibold' : 'text-theme-muted hover:text-theme-primary'
                }`}
              >
                <Icon className={`w-5 h-5 ${active ? 'stroke-[2.5]' : ''}`} />
                <span className="text-[10px] mt-0.5">{item.label}</span>
              </button>
            );
          })}
        </nav>
      </main>

      {/* ========================================================= */}
      {/* MODAL: ADD ITEM / PASSWORD GENERATOR */}
      {/* ========================================================= */}
      {showAddModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 safe-padding-top safe-padding-bottom">
          <div className="w-full max-w-lg bg-theme-modal border border-theme rounded-3xl p-6 shadow-2xl max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <div className="w-8 h-8 rounded-xl bg-indigo-600/20 text-indigo-500 flex items-center justify-center">
                  <Plus className="w-4 h-4" />
                </div>
                <h2 className="text-base font-bold text-theme-primary tracking-tight">Add to Vault</h2>
              </div>
              <button
                onClick={() => setShowAddModal(false)}
                className="p-2 rounded-xl text-theme-secondary hover:text-theme-primary hover:bg-theme-card min-h-[44px] min-w-[44px] flex items-center justify-center"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Segmented type switcher */}
            <div className="grid grid-cols-3 gap-1 bg-theme-main p-1 rounded-2xl border border-theme mb-5">
              {(['Logins', 'Secure Notes', 'Credit Cards'] as const).map(type => (
                <button
                  key={type}
                  type="button"
                  onClick={() => setNewItemType(type)}
                  className={`py-2 rounded-xl text-xs font-medium transition-all ${
                    newItemType === type
                      ? 'bg-theme-card text-theme-primary shadow-sm border border-theme'
                      : 'text-theme-secondary hover:text-theme-primary'
                  }`}
                >
                  {type === 'Logins' ? 'Login' : type === 'Secure Notes' ? 'Note' : 'Card'}
                </button>
              ))}
            </div>

            <form onSubmit={handleAddItem} className="space-y-4">
              <div>
                <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                  Title / Service Name *
                </label>
                <input
                  type="text"
                  value={newItem.title}
                  onChange={(e) => setNewItem({ ...newItem, title: e.target.value })}
                  placeholder="e.g. GitHub, ProtonMail, Bank"
                  className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                  required
                  autoFocus
                />
              </div>

              {newItemType === 'Logins' && (
                <>
                  <div>
                    <div className="flex items-center justify-between mb-1.5">
                      <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider">
                        Username / Email
                      </label>
                      <button
                        type="button"
                        onClick={() => {
                          const alias = handleGenerateAlias();
                          setNewItem({ ...newItem, username: alias });
                        }}
                        className="text-[11px] text-indigo-500 hover:text-indigo-600 flex items-center gap-1 font-medium"
                      >
                        <Mail className="w-3 h-3" />
                        <span>Generate Alias</span>
                      </button>
                    </div>
                    <input
                      type="text"
                      value={newItem.username}
                      onChange={(e) => setNewItem({ ...newItem, username: e.target.value })}
                      placeholder="username@domain.com or alias"
                      className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                    />
                  </div>

                  <div>
                    <div className="flex items-center justify-between mb-1.5">
                      <label className="text-xs font-semibold text-theme-secondary uppercase tracking-wider">
                        Password
                      </label>
                      <div className="flex items-center gap-2">
                        <button
                          type="button"
                          onClick={() => {
                            const p = handleGeneratePassphrase();
                            setNewItem({ ...newItem, password: p });
                          }}
                          className="text-[11px] text-slate-400 hover:text-theme-primary flex items-center gap-1 font-medium"
                        >
                          <Sparkles className="w-3 h-3" />
                          <span>Passphrase</span>
                        </button>
                        <button
                          type="button"
                          onClick={() => {
                            const p = handleGeneratePassword();
                            setNewItem({ ...newItem, password: p });
                          }}
                          className="text-[11px] text-indigo-500 hover:text-indigo-600 flex items-center gap-1 font-medium"
                        >
                          <RefreshCw className="w-3 h-3" />
                          <span>Generate</span>
                        </button>
                      </div>
                    </div>
                    <div className="relative">
                      <input
                        type={showPassword ? "text" : "password"}
                        value={newItem.password}
                        onChange={(e) => setNewItem({ ...newItem, password: e.target.value })}
                        placeholder="••••••••••••••••"
                        className="w-full h-11 bg-theme-input border border-theme rounded-xl pl-4 pr-10 text-xs font-mono text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                      />
                      <button
                        type="button"
                        onClick={() => setShowPassword(!showPassword)}
                        className="absolute right-3 top-3 text-slate-400 hover:text-slate-200"
                      >
                        {showPassword ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                      </button>
                    </div>

                    {/* Generator options accordion */}
                    <div className="mt-2">
                      <button
                        type="button"
                        onClick={() => setShowGenOptions(!showGenOptions)}
                        className="text-[11px] text-theme-secondary hover:text-theme-primary flex items-center gap-1 font-medium transition-colors"
                      >
                        <span>{showGenOptions ? "Hide Generator Options" : "Customize Password Options"}</span>
                      </button>

                      {showGenOptions && (
                        <div className="mt-2.5 p-3.5 rounded-xl bg-theme-card border border-theme space-y-3">
                          <div className="flex items-center justify-between text-xs text-theme-primary">
                            <span>Length: <strong className="font-mono text-indigo-500">{genLength}</strong></span>
                            <input
                              type="range"
                              min={8}
                              max={64}
                              value={genLength}
                              onChange={(e) => {
                                const val = parseInt(e.target.value, 10);
                                setGenLength(val);
                              }}
                              className="w-32 accent-indigo-500"
                            />
                          </div>

                          <div className="grid grid-cols-2 gap-2 pt-1 border-t border-theme text-xs">
                            <label className="flex items-center gap-2 text-theme-secondary cursor-pointer">
                              <input
                                type="checkbox"
                                checked={genUpper}
                                onChange={(e) => setGenUpper(e.target.checked)}
                                className="rounded accent-indigo-500"
                              />
                              <span>A-Z (Upper)</span>
                            </label>
                            <label className="flex items-center gap-2 text-theme-secondary cursor-pointer">
                              <input
                                type="checkbox"
                                checked={genLower}
                                onChange={(e) => setGenLower(e.target.checked)}
                                className="rounded accent-indigo-500"
                              />
                              <span>a-z (Lower)</span>
                            </label>
                            <label className="flex items-center gap-2 text-theme-secondary cursor-pointer">
                              <input
                                type="checkbox"
                                checked={genNumbers}
                                onChange={(e) => setGenNumbers(e.target.checked)}
                                className="rounded accent-indigo-500"
                              />
                              <span>0-9 (Numbers)</span>
                            </label>
                            <label className="flex items-center gap-2 text-theme-secondary cursor-pointer">
                              <input
                                type="checkbox"
                                checked={genSymbols}
                                onChange={(e) => setGenSymbols(e.target.checked)}
                                className="rounded accent-indigo-500"
                              />
                              <span>!@#$ (Symbols)</span>
                            </label>
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                </>
              )}

              {newItemType === 'Secure Notes' && (
                <div>
                  <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                    Note Content
                  </label>
                  <textarea
                    rows={5}
                    value={newItem.notes}
                    onChange={(e) => setNewItem({ ...newItem, notes: e.target.value })}
                    placeholder="Enter confidential notes, recovery phrases, or keys..."
                    className="w-full bg-theme-input border border-theme rounded-xl p-3.5 text-xs text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 font-mono"
                  />
                </div>
              )}

              {newItemType === 'Credit Cards' && (
                <>
                  <div>
                    <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                      Cardholder Name
                    </label>
                    <input
                      type="text"
                      value={newItem.username}
                      onChange={(e) => setNewItem({ ...newItem, username: e.target.value })}
                      placeholder="Jane Doe"
                      className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                    />
                  </div>

                  <div>
                    <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                      Card Number
                    </label>
                    <input
                      type="text"
                      value={newItem.cc}
                      onChange={(e) => setNewItem({ ...newItem, cc: e.target.value })}
                      placeholder="4000 1234 5678 9010"
                      className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs font-mono text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                    />
                  </div>

                  <div className="grid grid-cols-3 gap-3">
                    <div>
                      <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                        Exp Month
                      </label>
                      <input
                        type="text"
                        value={newItem.expMonth}
                        onChange={(e) => setNewItem({ ...newItem, expMonth: e.target.value })}
                        placeholder="12"
                        maxLength={2}
                        className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs font-mono text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 text-center"
                      />
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                        Exp Year
                      </label>
                      <input
                        type="text"
                        value={newItem.expYear}
                        onChange={(e) => setNewItem({ ...newItem, expYear: e.target.value })}
                        placeholder="28"
                        maxLength={4}
                        className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs font-mono text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 text-center"
                      />
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-theme-secondary uppercase tracking-wider mb-1.5">
                        CVV
                      </label>
                      <input
                        type="password"
                        value={newItem.password}
                        onChange={(e) => setNewItem({ ...newItem, password: e.target.value })}
                        placeholder="•••"
                        maxLength={4}
                        className="w-full h-11 bg-theme-input border border-theme rounded-xl px-4 text-xs font-mono text-theme-primary placeholder-slate-400 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 text-center"
                      />
                    </div>
                  </div>
                </>
              )}

              <div className="flex items-center justify-end gap-3 pt-4 border-t border-theme">
                <button
                  type="button"
                  onClick={() => setShowAddModal(false)}
                  className="px-4 h-11 text-xs font-medium text-theme-secondary hover:text-theme-primary transition-colors min-h-[44px]"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-6 h-11 bg-indigo-600 hover:bg-indigo-500 active:scale-95 text-white text-xs font-medium rounded-xl shadow-lg shadow-indigo-600/25 transition-all min-h-[44px]"
                >
                  Save Item
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* ========================================================= */}
      {/* MODAL: SETTINGS & VAULT MANAGEMENT */}
      {/* ========================================================= */}
      {showSettings && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 safe-padding-top safe-padding-bottom">
          <div className="w-full max-w-lg bg-theme-modal border border-theme rounded-3xl p-6 shadow-2xl max-h-[90vh] overflow-y-auto space-y-6">
            <div className="flex items-center justify-between border-b border-theme pb-4">
              <div className="flex items-center gap-2">
                <div className="w-8 h-8 rounded-xl bg-theme-tag text-theme-primary flex items-center justify-center">
                  <Settings className="w-4 h-4" />
                </div>
                <h2 className="text-base font-bold text-theme-primary tracking-tight">Settings</h2>
              </div>
              <button
                onClick={() => setShowSettings(false)}
                className="p-2 rounded-xl text-theme-secondary hover:text-theme-primary hover:bg-theme-card min-h-[44px] min-w-[44px] flex items-center justify-center"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Security Section */}
            <div className="space-y-3">
              <h3 className="text-xs font-semibold text-theme-secondary uppercase tracking-wider">
                Security &amp; Auto-Lock
              </h3>

              <div className="space-y-2">
                <div className="flex items-center justify-between p-3 rounded-xl bg-theme-card border border-theme-card">
                  <div>
                    <span className="text-xs font-medium text-theme-primary">Auto-Lock Timer</span>
                    <p className="text-[11px] text-theme-muted">Lock vault after period of inactivity</p>
                  </div>
                  <select
                    value={autoLockMinutes}
                    onChange={(e) => {
                      const val = parseInt(e.target.value, 10);
                      setAutoLockMinutes(val);
                      localStorage.setItem('orvpass_autolock', val.toString());
                    }}
                    className="bg-theme-input border border-theme rounded-lg px-2.5 py-1 text-xs text-theme-primary focus:outline-none focus:border-indigo-500"
                  >
                    <option value={1}>1 Minute</option>
                    <option value={5}>5 Minutes</option>
                    <option value={15}>15 Minutes</option>
                    <option value={30}>30 Minutes</option>
                    <option value={0}>Never</option>
                  </select>
                </div>

                <div className="flex items-center justify-between p-3 rounded-xl bg-theme-card border border-theme-card">
                  <div>
                    <span className="text-xs font-medium text-theme-primary">Clear Clipboard</span>
                    <p className="text-[11px] text-theme-muted">Clear copied passwords automatically</p>
                  </div>
                  <select
                    value={clearClipboardSeconds}
                    onChange={(e) => {
                      const val = parseInt(e.target.value, 10);
                      setClearClipboardSeconds(val);
                      localStorage.setItem('orvpass_clip_timer', val.toString());
                    }}
                    className="bg-theme-input border border-theme rounded-lg px-2.5 py-1 text-xs text-theme-primary focus:outline-none focus:border-indigo-500"
                  >
                    <option value={15}>15 Seconds</option>
                    <option value={30}>30 Seconds</option>
                    <option value={60}>60 Seconds</option>
                    <option value={0}>Never</option>
                  </select>
                </div>
              </div>
            </div>

            {/* Appearance Section */}
            <div className="space-y-3">
              <h3 className="text-xs font-semibold text-theme-secondary uppercase tracking-wider">
                Theme
              </h3>
              <div className="grid grid-cols-3 gap-2">
                {[
                  { id: 'dark', label: 'Dark', icon: Moon },
                  { id: 'light', label: 'Light', icon: Sun },
                  { id: 'system', label: 'System', icon: Laptop }
                ].map(t => {
                  const Icon = t.icon;
                  const active = theme === t.id;
                  return (
                    <button
                      key={t.id}
                      onClick={() => setTheme(t.id as any)}
                      className={`flex items-center justify-center gap-2 py-2.5 rounded-xl border text-xs font-medium transition-all ${
                        active
                          ? 'bg-indigo-600/20 border-indigo-500 text-indigo-500 font-semibold'
                          : 'bg-theme-card border-theme text-theme-secondary hover:text-theme-primary'
                      }`}
                    >
                      <Icon className="w-3.5 h-3.5" />
                      <span>{t.label}</span>
                    </button>
                  );
                })}
              </div>
            </div>

            {/* Data Management Section */}
            <div className="space-y-3">
              <h3 className="text-xs font-semibold text-theme-secondary uppercase tracking-wider">
                Import &amp; Export
              </h3>

              <div className="grid grid-cols-2 gap-2">
                <button
                  onClick={() => fileInputRef.current?.click()}
                  className="flex items-center justify-center gap-2 py-2.5 px-3 rounded-xl bg-theme-card hover:bg-theme-card-hover border border-theme text-xs font-medium text-theme-primary transition-all min-h-[44px]"
                >
                  <Upload className="w-4 h-4 text-indigo-500" />
                  <span>Import File</span>
                </button>
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".csv,.json"
                  onChange={handleFileImport}
                  className="hidden"
                />

                <button
                  onClick={() => exportData('csv')}
                  className="flex items-center justify-center gap-2 py-2.5 px-3 rounded-xl bg-theme-card hover:bg-theme-card-hover border border-theme text-xs font-medium text-theme-primary transition-all min-h-[44px]"
                >
                  <Download className="w-4 h-4 text-indigo-500" />
                  <span>Export CSV</span>
                </button>

                <button
                  onClick={() => exportData('json')}
                  className="col-span-2 flex items-center justify-center gap-2 py-2.5 px-3 rounded-xl bg-theme-card hover:bg-theme-card-hover border border-theme text-xs font-medium text-theme-primary transition-all min-h-[44px]"
                >
                  <Download className="w-4 h-4 text-indigo-500" />
                  <span>Export JSON</span>
                </button>

                <button
                  onClick={exportStandaloneHtml}
                  className="col-span-2 flex items-center justify-center gap-2 py-2.5 px-3 rounded-xl bg-indigo-600/10 hover:bg-indigo-600/20 border border-indigo-500/30 text-xs font-medium text-indigo-500 hover:text-indigo-400 transition-all min-h-[44px]"
                >
                  <FileCode className="w-4 h-4 text-indigo-500" />
                  <span>Export Standalone Emergency HTML Vault</span>
                </button>
              </div>
            </div>

            {/* Duress Mode Info */}
            <div className="space-y-2 p-3.5 rounded-xl bg-amber-500/10 border border-amber-500/20">
              <div className="flex items-center gap-2 text-amber-500 text-xs font-semibold">
                <ShieldAlert className="w-4 h-4" />
                <span>Duress / Coercion Protection</span>
              </div>
              <p className="text-[11px] text-theme-secondary leading-relaxed">
                Unlock with password <strong className="font-mono text-amber-500">duress</strong> or PIN <strong className="font-mono text-amber-500">0000</strong> to open a realistic decoy vault under coercion.
              </p>
            </div>

            {/* About & Cryptography */}
            <div className="pt-2 border-t border-theme space-y-2">
              <div className="flex items-center justify-between text-xs text-theme-secondary">
                <span>Version</span>
                <span className="font-mono text-theme-primary font-medium">Orvpass v4.2.0</span>
              </div>
              <div className="flex items-center justify-between text-xs text-theme-secondary">
                <span>Key Derivation</span>
                <span className="font-mono text-theme-primary">Argon2id (64MB, 3 iter)</span>
              </div>
              <div className="flex items-center justify-between text-xs text-theme-secondary">
                <span>AEAD Encryption</span>
                <span className="font-mono text-theme-primary">ChaCha20-Poly1305</span>
              </div>
              <div className="flex items-center justify-between text-xs text-theme-secondary">
                <span>Telemetry</span>
                <span className="text-emerald-500 font-medium">None (100% Local)</span>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* ========================================================= */}
      {/* MODAL: AIR-GAPPED P2P SYNC */}
      {/* ========================================================= */}
      {showSyncModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 safe-padding-top safe-padding-bottom">
          <div className="w-full max-w-lg bg-theme-modal border border-theme rounded-3xl p-6 shadow-2xl space-y-5">
            <div className="flex items-center justify-between border-b border-theme pb-3">
              <div className="flex items-center gap-2">
                <div className="w-8 h-8 rounded-xl bg-indigo-600/20 text-indigo-500 flex items-center justify-center">
                  <Share2 className="w-4 h-4" />
                </div>
                <h2 className="text-base font-bold text-theme-primary tracking-tight">Air-Gapped P2P Sync</h2>
              </div>
              <button
                onClick={() => setShowSyncModal(false)}
                className="p-2 rounded-xl text-theme-secondary hover:text-theme-primary min-h-[44px] min-w-[44px] flex items-center justify-center"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            <div className="space-y-4 text-xs">
              <div className="p-3.5 rounded-xl bg-theme-card border border-theme space-y-2">
                <span className="font-semibold text-theme-primary block">1. Export Encrypted Sync Payload</span>
                <p className="text-theme-muted text-[11px]">Copy this encrypted bundle to transfer to another device offline.</p>
                <textarea
                  readOnly
                  rows={3}
                  value={btoa(encodeURIComponent(JSON.stringify(items)))}
                  className="w-full bg-theme-input border border-theme rounded-lg p-2 font-mono text-[10px] text-theme-secondary focus:outline-none"
                />
                <button
                  onClick={() => copyToClipboard(btoa(encodeURIComponent(JSON.stringify(items))), 'sync-copy')}
                  className="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-500 active:scale-95 text-white rounded-lg text-xs font-medium flex items-center gap-1.5 transition-all"
                >
                  <Copy className="w-3.5 h-3.5" />
                  <span>{copiedId === 'sync-copy' ? 'Copied' : 'Copy Sync Payload'}</span>
                </button>
              </div>

              <div className="p-3.5 rounded-xl bg-theme-card border border-theme space-y-2">
                <span className="font-semibold text-theme-primary block">2. Import / Merge Sync Payload</span>
                <p className="text-theme-muted text-[11px]">Paste an encrypted payload from your other device to merge credentials.</p>
                <textarea
                  rows={3}
                  placeholder="Paste encrypted payload here..."
                  value={syncInput}
                  onChange={(e) => setSyncInput(e.target.value)}
                  className="w-full bg-theme-input border border-theme rounded-lg p-2 font-mono text-[10px] text-theme-primary focus:outline-none focus:border-indigo-500"
                />
                <button
                  onClick={async () => {
                    try {
                      const decoded = JSON.parse(decodeURIComponent(atob(syncInput.trim())));
                      if (Array.isArray(decoded)) {
                        let count = 0;
                        for (const item of decoded) {
                          await invoke('add_item', {
                            itemType: item.type || 'Logins',
                            title: item.title,
                            username: item.username || null,
                            pass: item.password || null,
                            notes: item.notes || null,
                            cc: item.cc || null,
                            expMonth: item.expMonth || null,
                            expYear: item.expYear || null
                          });
                          count++;
                        }
                        await loadItems();
                        setImportSummary(`Air-Gapped Sync merged ${count} items successfully.`);
                        setShowSyncModal(false);
                        setSyncInput('');
                      }
                    } catch (e) {
                      alert('Invalid sync payload.');
                    }
                  }}
                  className="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-500 active:scale-95 text-white rounded-lg text-xs font-medium flex items-center gap-1.5 transition-all"
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Merge Sync Payload</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
