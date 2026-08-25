import Foundation

public struct VaultItem: Identifiable, Codable, Hashable {
    public let id: String
    public var type: String
    public var title: String
    public var username: String
    public var password: String
    public var notes: String
    public var isPinned: Bool
    public var isTrash: Bool
    public var isArchived: Bool
    public var vaultCategory: String // Personal, Work, Family
    public var updatedAt: Date

    public init(
        id: String = UUID().uuidString,
        type: String = "Logins",
        title: String,
        username: String = "",
        password: String = "",
        notes: String = "",
        isPinned: Bool = false,
        isTrash: Bool = false,
        isArchived: Bool = false,
        vaultCategory: String = "Personal",
        updatedAt: Date = Date()
    ) {
        self.id = id
        self.type = type
        self.title = title
        self.username = username
        self.password = password
        self.notes = notes
        self.isPinned = isPinned
        self.isTrash = isTrash
        self.isArchived = isArchived
        self.vaultCategory = vaultCategory
        self.updatedAt = updatedAt
    }
}

public struct HealthStats: Codable {
    public var total: Int
    public var strong: Int
    public var weak: Int
    public var reused: Int
    public var score: Int

    public init(total: Int = 0, strong: Int = 0, weak: Int = 0, reused: Int = 0, score: Int = 100) {
        self.total = total
        self.strong = strong
        self.weak = weak
        self.reused = reused
        self.score = score
    }
}
