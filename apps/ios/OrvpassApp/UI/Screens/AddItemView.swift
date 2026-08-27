import SwiftUI

public struct AddItemView: View {
    @Environment(\.dismiss) private var dismiss
    @ObservedObject var repository = VaultRepository.shared

    @State private var selectedType: String = "Logins"
    @State private var title: String = ""
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var notes: String = ""
    @State private var isPassphraseMode: Bool = false
    @State private var showPassword: Bool = false
    @State private var genLength: Double = 18

    private let categories = ["Logins", "Passkeys", "Secure Notes", "Credit Cards"]

    public init() {}

    public var body: some View {
        NavigationView {
            Form {
                Section("Templates & Presets") {
                    ScrollView(.horizontal, showsIndicators: false) {
                        HStack(spacing: 8) {
                            Button("🔑 SSH Key") {
                                selectedType = "Secure Notes"
                                title = "SSH Server Key"
                                notes = "Host: 192.168.1.1\nPort: 22\nUser: root\nKey: ssh-ed25519 AAAAC3NzaC1lZDI1NTE5..."
                            }
                            .buttonStyle(.bordered)
                            .font(.caption)

                            Button("📶 Wi-Fi") {
                                selectedType = "Secure Notes"
                                title = "Home Wi-Fi Network"
                                notes = "SSID: HomeNetwork\nSecurity: WPA3-Personal\nPassword: "
                            }
                            .buttonStyle(.bordered)
                            .font(.caption)

                            Button("📜 License") {
                                selectedType = "Secure Notes"
                                title = "Software License"
                                notes = "Product: App Pro\nLicense Key: XXXX-XXXX-XXXX-XXXX\nSeat: 1"
                            }
                            .buttonStyle(.bordered)
                            .font(.caption)
                        }
                        .padding(.vertical, 2)
                    }
                }

                Section {
                    Picker("Category", selection: $selectedType) {
                        ForEach(categories, id: \.self) { cat in
                            Text(cat).tag(cat)
                        }
                    }
                    .pickerStyle(.segmented)
                    .onChange(of: selectedType) { newType in
                        if newType == "Logins" && password.isEmpty {
                            generatePassword()
                        } else if newType == "Passkeys" && notes.isEmpty {
                            notes = "FIDO2 / WebAuthn Discoverable Passkey (ES256)"
                        }
                    }
                }

                Section("Details") {
                    TextField("Title / Service *", text: $title)

                    if selectedType == "Logins" || selectedType == "Passkeys" {
                        HStack {
                            TextField(selectedType == "Passkeys" ? "User Handle / Email" : "Username / Email", text: $username)
                                .textContentType(.username)
                                .autocapitalization(.none)

                            Button("Alias") {
                                let randomWord = ["shadow", "swift", "nexus", "prism", "cyber"].randomElement() ?? "alias"
                                let num = Int.random(in: 100...999)
                                username = "\(randomWord)\(num)@duck.com"
                            }
                            .font(.caption2)
                            .foregroundColor(.indigo)
                        }
                    }

                    if selectedType == "Logins" {
                        HStack {
                            if showPassword {
                                TextField("Password", text: $password)
                            } else {
                                SecureField("Password", text: $password)
                            }

                            Button(action: { showPassword.toggle() }) {
                                Image(systemName: showPassword ? "eye.slash" : "eye")
                                    .foregroundColor(.gray)
                            }

                            Button(action: generatePassword) {
                                Image(systemName: "arrow.clockwise")
                                    .foregroundColor(.indigo)
                            }
                        }

                        Toggle("Diceware Passphrase Mode", isOn: $isPassphraseMode)
                            .onChange(of: isPassphraseMode) { _ in
                                generatePassword()
                            }

                        if !isPassphraseMode {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Length: \(Int(genLength))")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Slider(value: $genLength, in: 8...64, step: 1) { _ in
                                    generatePassword()
                                }
                            }
                        }
                    }
                }

                Section("Notes & Custom Fields") {
                    TextEditor(text: $notes)
                        .frame(minHeight: 100)
                }
            }
            .navigationTitle("Add to Vault")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") {
                        saveItem()
                    }
                    .disabled(title.trimmingCharacters(in: .whitespaces).isEmpty)
                }
            }
            .onAppear {
                if selectedType == "Logins" && password.isEmpty {
                    generatePassword()
                }
            }
        }
    }

    private func generatePassword() {
        if isPassphraseMode {
            let words = ["correct", "horse", "battery", "staple", "silver", "crypto", "shield", "beacon", "galaxy", "orbit", "quantum", "falcon", "liquid", "zenith", "timber", "solace"]
            let chosen = (0..<4).compactMap { _ in words.randomElement() }
            let num = Int.random(in: 10...99)
            password = chosen.joined(separator: "-") + "-\(num)"
        } else {
            let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-="
            let length = Int(genLength)
            password = String((0..<length).compactMap { _ in chars.randomElement() })
        }
    }

    private func saveItem() {
        let newItem = VaultItem(
            type: selectedType,
            title: title.trimmingCharacters(in: .whitespaces),
            username: username.trimmingCharacters(in: .whitespaces),
            password: password,
            notes: notes,
            vaultCategory: repository.selectedVault
        )
        repository.addItem(newItem)
        dismiss()
    }
}
