import Foundation
import LocalAuthentication

@MainActor
public class VaultRepository: ObservableObject {
    public static let shared = VaultRepository()

    @Published public var items: [VaultItem] = []
    @Published public var isUnlocked: Bool = false
    @Published public var isDecoyMode: Bool = false
    @Published public var selectedVault: String = "Personal"

    private let storageKey = "orvpass_vault_items_v5"
    private let pinKey = "orvpass_quick_pin"
    private let decoyPin = "0000"

    private init() {
        loadInitialData()
    }

    public func unlockWithPin(_ pin: String) -> Bool {
        let savedPin = UserDefaults.standard.string(forKey: pinKey) ?? "1234"
        if pin == decoyPin {
            isDecoyMode = true
            items = [
                VaultItem(type: "Logins", title: "Public Wi-Fi Guest", username: "guest@city.local", password: "CoffeeWifi2024!"),
                VaultItem(type: "Secure Notes", title: "Grocery List", notes: "Oat milk, Apples, Coffee beans")
            ]
            isUnlocked = true
            return true
        } else if pin == savedPin {
            isDecoyMode = false
            loadItems()
            isUnlocked = true
            return true
        }
        return false
    }

    public func authenticateWithBiometrics(completion: @escaping (Bool) -> Void) {
        let context = LAContext()
        var error: NSError?

        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) {
            context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, localizedReason: "Unlock your Orvpass Secure Vault") { success, _ in
                DispatchQueue.main.async {
                    if success {
                        self.isDecoyMode = false
                        self.loadItems()
                        self.isUnlocked = true
                        completion(true)
                    } else {
                        completion(false)
                    }
                }
            }
        } else {
            completion(false)
        }
    }

    public func lockVault() {
        isUnlocked = false
        items = []
    }

    public func addItem(_ item: VaultItem) {
        items.insert(item, at: 0)
        persistItems()
    }

    public func deleteItem(id: String) {
        items.removeAll { $0.id == id }
        persistItems()
    }

    public func togglePin(id: String) {
        if let index = items.firstIndex(where: { $0.id == id }) {
            items[index].isPinned.toggle()
            persistItems()
        }
    }

    public func setQuickPin(_ pin: String) {
        UserDefaults.standard.set(pin, forKey: pinKey)
    }

    public func exportToCsv() -> String {
        var csv = "title,username,password,notes,type,vaultCategory\n"
        for item in items {
            let escapedNotes = item.notes.replacingOccurrences(of: "\"", with: "'")
            let row = "\"\(item.title)\",\"\(item.username)\",\"\(item.password)\",\"\(escapedNotes)\",\"\(item.type)\",\"\(item.vaultCategory)\"\n"
            csv += row
        }
        return csv
    }

    public func exportToJson() -> String {
        guard let data = try? JSONEncoder().encode(items),
              let jsonStr = String(data: data, encoding: .utf8) else {
            return "[]"
        }
        return jsonStr
    }

    public func exportToHtml() -> String {
        var cardsHtml = ""
        for item in items {
            cardsHtml += "<div class=\"card\"><h3>\(item.title) (\(item.type))</h3>"
            if !item.username.isEmpty {
                cardsHtml += "<div class=\"field\"><span class=\"label\">Username:</span> <span class=\"code\">\(item.username)</span></div>"
            }
            if !item.password.isEmpty {
                cardsHtml += "<div class=\"field\"><span class=\"label\">Password:</span> <span class=\"code\">\(item.password)</span></div>"
            }
            if !item.notes.isEmpty {
                cardsHtml += "<div class=\"field\"><span class=\"label\">Notes:</span> \(item.notes)</div>"
            }
            cardsHtml += "</div>\n"
        }

        return """
        <!DOCTYPE html>
        <html>
        <head>
          <meta charset="UTF-8">
          <title>Orvpass Emergency Vault Backup</title>
          <style>
            body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; background: #070b14; color: #f8fafc; padding: 2rem; max-width: 800px; margin: auto; }
            .card { background: #131d33; border: 1px solid rgba(255,255,255,0.08); border-radius: 12px; padding: 1.25rem; margin-bottom: 1rem; }
            h1 { color: #818cf8; }
            .meta { color: #94a3b8; font-size: 0.85rem; margin-bottom: 1.5rem; }
            .field { margin-top: 0.5rem; font-size: 0.9rem; }
            .label { color: #64748b; font-weight: bold; }
            .code { font-family: monospace; background: rgba(0,0,0,0.3); padding: 2px 6px; border-radius: 4px; }
          </style>
        </head>
        <body>
          <h1>Orvpass Emergency Offline Backup</h1>
          <div class="meta">Exported \(Date()) &bull; \(items.count) Items</div>
          \(cardsHtml)
        </body>
        </html>
        """
    }

    public func importFromText(_ content: String) -> Int {
        var count = 0
        if content.trimmingCharacters(in: .whitespaces).starts(with: "[") {
            // JSON Import
            if let data = content.data(using: .utf8),
               let imported = try? JSONDecoder().decode([VaultItem].self, from: data) {
                for item in imported {
                    if !items.contains(where: { $0.title == item.title && $0.username == item.username }) {
                        items.insert(item, at: 0)
                        count += 1
                    }
                }
            }
        } else {
            // CSV Import
            let lines = content.components(separatedBy: .newlines)
            for (idx, line) in lines.enumerated() where idx > 0 && !line.isEmpty {
                let parts = line.components(separatedBy: ",")
                if parts.count >= 2 {
                    let title = parts[0].trimmingCharacters(in: CharacterSet(charactersIn: "\""))
                    let user = parts.count > 1 ? parts[1].trimmingCharacters(in: CharacterSet(charactersIn: "\"")) : ""
                    let pass = parts.count > 2 ? parts[2].trimmingCharacters(in: CharacterSet(charactersIn: "\"")) : ""
                    let notes = parts.count > 3 ? parts[3].trimmingCharacters(in: CharacterSet(charactersIn: "\"")) : ""
                    let newItem = VaultItem(type: "Logins", title: title, username: user, password: pass, notes: notes)
                    items.insert(newItem, at: 0)
                    count += 1
                }
            }
        }
        if count > 0 {
            persistItems()
        }
        return count
    }

    private func loadInitialData() {
        if UserDefaults.standard.data(forKey: storageKey) == nil {
            let samples = [
                VaultItem(type: "Logins", title: "GitHub", username: "krtvysinghh", password: "OrvpassSecure2026!", notes: "Hardware token enabled"),
                VaultItem(type: "Passkeys", title: "Google Cloud", username: "admin@orvpass.dev", notes: "FIDO2 / WebAuthn Discoverable Passkey (ES256)"),
                VaultItem(type: "Secure Notes", title: "Server Recovery Phrase", notes: "abandon amount anchor arena argue ... (ChaCha20 Encrypted)")
            ]
            if let data = try? JSONEncoder().encode(samples) {
                UserDefaults.standard.set(data, forKey: storageKey)
            }
        }
    }

    private func loadItems() {
        guard let data = UserDefaults.standard.data(forKey: storageKey),
              let decoded = try? JSONDecoder().decode([VaultItem].self, from: data) else {
            items = []
            return
        }
        items = decoded
    }

    private func persistItems() {
        if isDecoyMode { return }
        if let data = try? JSONEncoder().encode(items) {
            UserDefaults.standard.set(data, forKey: storageKey)
        }
    }
}
