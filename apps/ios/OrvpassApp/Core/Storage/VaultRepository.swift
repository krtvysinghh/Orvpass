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
