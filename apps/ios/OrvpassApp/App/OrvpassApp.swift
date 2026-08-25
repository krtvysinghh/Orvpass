import SwiftUI

@main
public struct OrvpassApp: App {
    @StateObject private var repository = VaultRepository.shared

    public init() {}

    public var body: some Scene {
        WindowGroup {
            Group {
                if repository.isUnlocked {
                    VaultListView()
                        .transition(.opacity)
                } else {
                    UnlockVaultView()
                        .transition(.opacity)
                }
            }
            .animation(.easeInOut(duration: 0.25), value: repository.isUnlocked)
            .preferredColorScheme(.dark)
        }
    }
}
