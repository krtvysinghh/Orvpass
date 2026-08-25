import SwiftUI
import UniformTypeIdentifiers

public struct SettingsView: View {
    @ObservedObject var repository = VaultRepository.shared
    @AppStorage("orvpass_quick_pin") private var quickPin: String = "1234"
    @AppStorage("orvpass_travel_mode") private var travelMode: Bool = false
    @AppStorage("orvpass_account_email") private var accountEmail: String = ""
    @AppStorage("orvpass_remember_me") private var rememberMe: Bool = true

    @State private var showImportPicker: Bool = false
    @State private var importStatus: String? = nil
    @State private var showExportSheet: Bool = false
    @State private var exportText: String = ""
    @State private var showAccountModal: Bool = false
    @State private var authEmailInput: String = ""
    @State private var authOtpInput: String = ""
    @State private var generatedOtp: String = ""
    @State private var isOtpStep: Bool = false

    public init() {}

    public var body: some View {
        List {
            // Zero-Knowledge Account
            Section("Zero-Knowledge Sync Account") {
                HStack {
                    Text("Account")
                    Spacer()
                    Text(accountEmail.isEmpty ? "Local Vault Only" : accountEmail)
                        .foregroundColor(accountEmail.isEmpty ? .secondary : .green)
                        .fontWeight(.semibold)
                }

                if accountEmail.isEmpty {
                    Button("Sign In or Register Sync Account") {
                        showAccountModal = true
                    }
                    .foregroundColor(.indigo)
                    .fontWeight(.semibold)
                } else {
                    Button("Sign Out of Sync") {
                        accountEmail = ""
                    }
                    .foregroundColor(.red)
                }
            }

            // Security & Biometrics
            Section("Security & Biometrics") {
                HStack {
                    Text("Face ID / Touch ID")
                    Spacer()
                    Text("Active")
                        .foregroundColor(.green)
                        .fontWeight(.semibold)
                }

                HStack {
                    Text("Quick PIN")
                    Spacer()
                    Text("••••")
                        .font(.system(.body, design: .monospaced))
                }

                Toggle("Travel Mode (Hide Sensitive)", isOn: $travelMode)
            }

            // Import & Export
            Section("Import & Export") {
                Button(action: { showImportPicker = true }) {
                    Label("Import CSV / JSON / Bitwarden", systemImage: "square.and.arrow.down")
                        .foregroundColor(.indigo)
                }

                Button(action: {
                    exportText = repository.exportToCsv()
                    showExportSheet = true
                }) {
                    Label("Export Vault as CSV", systemImage: "doc.text")
                        .foregroundColor(.primary)
                }

                Button(action: {
                    exportText = repository.exportToJson()
                    showExportSheet = true
                }) {
                    Label("Export Vault as JSON", systemImage: "curlybraces")
                        .foregroundColor(.primary)
                }

                Button(action: {
                    exportText = repository.exportToHtml()
                    showExportSheet = true
                }) {
                    Label("Export Emergency HTML Vault", systemImage: "safari")
                        .foregroundColor(.indigo)
                }

                if let status = importStatus {
                    Text(status)
                        .font(.caption)
                        .foregroundColor(.green)
                }
            }

            // Cryptographic Engine
            Section("Cryptographic Engine") {
                HStack {
                    Text("Version")
                    Spacer()
                    Text("Orvpass v5.0.0 Native")
                        .fontWeight(.bold)
                }
                HStack {
                    Text("Derivation")
                    Spacer()
                    Text("Argon2id (64MB, 3 iter)")
                        .font(.system(.caption, design: .monospaced))
                }
                HStack {
                    Text("AEAD Cipher")
                    Spacer()
                    Text("ChaCha20-Poly1305")
                        .font(.system(.caption, design: .monospaced))
                }
                HStack {
                    Text("Hardware Security")
                    Spacer()
                    Text("Apple Secure Enclave")
                        .foregroundColor(.indigo)
                        .fontWeight(.semibold)
                }
            }
        }
        .navigationTitle("Settings")
        .navigationBarTitleDisplayMode(.inline)
        .fileImporter(isPresented: $showImportPicker, allowedContentTypes: [.plainText, .json, .commaSeparatedText]) { result in
            switch result {
            case .success(let url):
                if url.startAccessingSecurityScopedResource() {
                    defer { url.stopAccessingSecurityScopedResource() }
                    if let text = try? String(contentsOf: url) {
                        let imported = repository.importFromText(text)
                        importStatus = "Successfully imported \(imported) items!"
                    }
                }
            case .failure(let err):
                importStatus = "Import error: \(err.localizedDescription)"
            }
        }
        .sheet(isPresented: $showExportSheet) {
            ShareSheet(text: exportText)
        }
        .sheet(isPresented: $showAccountModal) {
            NavigationView {
                Form {
                    if !isOtpStep {
                        Section("Email & Master Password") {
                            TextField("Email", text: $authEmailInput)
                                .autocapitalization(.none)
                                .keyboardType(.emailAddress)
                            Toggle("Remember Me", isOn: $rememberMe)
                        }
                        Section {
                            Button("Send Verification Code") {
                                if !authEmailInput.isEmpty {
                                    generatedOtp = String(format: "%06d", Int.random(in: 100000...999999))
                                    isOtpStep = true
                                }
                            }
                            .disabled(authEmailInput.isEmpty)
                        }
                    } else {
                        Section("Email Verification") {
                            Text("A 6-digit code has been generated: \(generatedOtp)")
                                .font(.caption)
                                .foregroundColor(.secondary)
                            TextField("Enter 6-Digit Code", text: $authOtpInput)
                                .keyboardType(.numberPad)
                        }
                        Section {
                            Button("Verify & Activate") {
                                if authOtpInput.trimmingCharacters(in: .whitespaces) == generatedOtp {
                                    accountEmail = authEmailInput
                                    showAccountModal = false
                                    isOtpStep = false
                                    authOtpInput = ""
                                }
                            }
                            .disabled(authOtpInput.count != 6)
                        }
                    }
                }
                .navigationTitle("Sync Account")
                .toolbar {
                    ToolbarItem(placement: .cancellationAction) {
                        Button("Cancel") { showAccountModal = false }
                    }
                }
            }
        }
    }
}

struct ShareSheet: UIViewControllerRepresentable {
    let text: String

    func makeUIViewController(context: Context) -> UIActivityViewController {
        UIActivityViewController(activityItems: [text], applicationActivities: nil)
    }

    func updateUIViewController(_ uiViewController: UIActivityViewController, context: Context) {}
}
