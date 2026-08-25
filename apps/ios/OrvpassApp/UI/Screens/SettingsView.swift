import SwiftUI

public struct SettingsView: View {
    @ObservedObject var repository = VaultRepository.shared
    @AppStorage("orvpass_quick_pin") private var quickPin: String = "1234"
    @AppStorage("orvpass_travel_mode") private var travelMode: Bool = false
    @AppStorage("orvpass_account_email") private var accountEmail: String = ""

    public init() {}

    public var body: some View {
        List {
            Section("Zero-Knowledge Sync Account") {
                HStack {
                    Text("Status")
                    Spacer()
                    Text(accountEmail.isEmpty ? "Local Vault Only" : "Active Relay")
                        .foregroundColor(accountEmail.isEmpty ? .secondary : .green)
                        .fontWeight(.semibold)
                }

                HStack {
                    Text("Sync Relay URL")
                    Spacer()
                    Text("https://sync.orvpass.local/v1")
                        .font(.system(.caption, design: .monospaced))
                        .foregroundColor(.secondary)
                }
            }

            Section("Security & Biometrics") {
                HStack {
                    Text("Face ID / Touch ID")
                    Spacer()
                    Text("Enabled")
                        .foregroundColor(.green)
                        .fontWeight(.semibold)
                }

                HStack {
                    Text("Quick PIN (4 Digits)")
                    Spacer()
                    Text("••••")
                        .font(.system(.body, design: .monospaced))
                }

                Toggle("Travel Mode (Hide Sensitive)", isOn: $travelMode)
            }

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
    }
}
