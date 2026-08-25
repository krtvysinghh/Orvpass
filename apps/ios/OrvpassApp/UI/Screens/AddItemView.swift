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
                        TextField(selectedType == "Passkeys" ? "User Handle / Email" : "Username / Email", text: $username)
                            .textContentType(.username)
                            .autocapitalization(.none)
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

                Section("Notes & Keys") {
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
                    .fontWeight(.bold)
                }
            }
        }
        .onAppear {
            if password.isEmpty && selectedType == "Logins" {
                generatePassword()
            }
        }
    }

    private func generatePassword() {
        if isPassphraseMode {
            password = LocalCryptoEngine.generateDicewarePassphrase()
        } else {
            password = LocalCryptoEngine.generatePassword(length: Int(genLength))
        }
    }

    private func saveItem() {
        guard !title.trimmingCharacters(in: .whitespaces).isEmpty else { return }
        let newItem = VaultItem(
            type: selectedType,
            title: title.trimmingCharacters(in: .whitespaces),
            username: username,
            password: password,
            notes: notes,
            vaultCategory: repository.selectedVault
        )
        repository.addItem(newItem)
        dismiss()
    }
}
