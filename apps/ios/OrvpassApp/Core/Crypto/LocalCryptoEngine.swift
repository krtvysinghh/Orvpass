import Foundation
import Security
import CryptoKit

public struct LocalCryptoEngine {
    
    private static let dicewareWords = [
        "falcon", "shield", "crypto", "cipher", "matrix", "beacon", "galaxy", "orbit",
        "quantum", "vector", "shadow", "summit", "horizon", "glacier", "phoenix", "aurora",
        "nebula", "zenith", "vortex", "starlight", "timber", "cascade", "dynamo", "solace",
        "granite", "pinnacle", "bastion", "sentinel", "citadel", "velocity", "meridian", "solstice",
        "eclipse", "astral", "chrono", "pulsar", "quasar", "titan", "hydra", "radiant",
        "blizzard", "canyon", "tempest", "monarch", "vanguard", "tundra", "evergreen", "valiant"
    ]

    public static func generateDicewarePassphrase(wordCount: Int = 4, separator: String = "-") -> String {
        var selectedWords: [String] = []
        for _ in 0..<wordCount {
            var randomBytes = UInt32(0)
            _ = SecRandomCopyBytes(kSecRandomDefault, MemoryLayout<UInt32>.size, &randomBytes)
            let index = Int(randomBytes) % dicewareWords.count
            selectedWords.append(dicewareWords[index])
        }
        var randomNum = UInt32(0)
        _ = SecRandomCopyBytes(kSecRandomDefault, MemoryLayout<UInt32>.size, &randomNum)
        let num = (Int(randomNum) % 90) + 10
        return selectedWords.joined(separator: separator) + "\(separator)\(num)"
    }

    public static func generatePassword(
        length: Int = 18,
        upper: Bool = true,
        lower: Bool = true,
        numbers: Bool = true,
        symbols: Bool = true
    ) -> String {
        var charset = ""
        if upper { charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ" }
        if lower { charset += "abcdefghijklmnopqrstuvwxyz" }
        if numbers { charset += "0123456789" }
        if symbols { charset += "!@#$%^&*()_+-=[]{}|;:,.<>?" }
        if charset.isEmpty { charset = "abcdefghijklmnopqrstuvwxyz0123456789" }

        let characters = Array(charset)
        var result = ""
        for _ in 0..<length {
            var randomBytes = UInt32(0)
            _ = SecRandomCopyBytes(kSecRandomDefault, MemoryLayout<UInt32>.size, &randomBytes)
            let index = Int(randomBytes) % characters.count
            result.append(characters[index])
        }
        return result
    }

    public static func evaluateHealth(items: [VaultItem]) -> HealthStats {
        let passwords = items.compactMap { $0.password.isEmpty ? nil : $0.password }
        if passwords.isEmpty {
            return HealthStats(total: items.count, strong: 0, weak: 0, reused: 0, score: 100)
        }

        var weakCount = 0
        var reusedCount = 0
        var passCounts: [String: Int] = [:]

        for pass in passwords {
            passCounts[pass, default: 0] += 1
            if pass.count < 12 {
                weakCount += 1
            }
        }

        for (_, count) in passCounts where count > 1 {
            reusedCount += count
        }

        let strongCount = max(0, passwords.count - weakCount - (reusedCount / 2))
        let deductions = (weakCount * 15) + (reusedCount * 10)
        let score = max(0, min(100, 100 - deductions))

        return HealthStats(
            total: items.count,
            strong: strongCount,
            weak: weakCount,
            reused: reusedCount,
            score: score
        )
    }
}
