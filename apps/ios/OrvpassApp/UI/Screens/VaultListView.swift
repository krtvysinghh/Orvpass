import SwiftUI

public struct VaultListView: View {
    @ObservedObject var repository = VaultRepository.shared

    @State private var searchQuery: String = ""
    @State private var selectedTab: String = "All Items"
    @State private var showAddSheet: Bool = false
    @State private var showHealth: Bool = false
    @State private var showSettings: Bool = false
    @State private var copiedItemId: String? = nil

    private let categories = ["All Items", "Favorites", "Logins", "Passkeys", "Secure Notes", "Credit Cards"]
    private let vaults = ["Personal", "Work", "Family"]
    private let generator = UIImpactFeedbackGenerator(style: .light)

    public init() {}

    private var filteredItems: [VaultItem] {
        repository.items.filter { item in
            let matchSearch = searchQuery.isEmpty ||
                item.title.localizedCaseInsensitiveContains(searchQuery) ||
                item.username.localizedCaseInsensitiveContains(searchQuery) ||
                item.notes.localizedCaseInsensitiveContains(searchQuery)

            guard matchSearch else { return false }

            switch selectedTab {
            case "Favorites": return item.isPinned
            case "Logins": return item.type == "Logins"
            case "Passkeys": return item.type == "Passkeys" || item.notes.contains("FIDO2")
            case "Secure Notes": return item.type == "Secure Notes"
            case "Credit Cards": return item.type == "Credit Cards"
            default: return true
            }
        }.sorted { ($0.isPinned ? 1 : 0) > ($1.isPinned ? 1 : 0) }
    }

    public var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                // Multi-Vault Switcher Bar
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 8) {
                        ForEach(vaults, id: \.self) { v in
                            Button(action: {
                                generator.impactOccurred()
                                repository.selectedVault = v
                            }) {
                                Text(v)
                                    .font(.system(size: 12, weight: repository.selectedVault == v ? .bold : .medium))
                                    .padding(.horizontal, 14)
                                    .padding(.vertical, 6)
                                    .background(repository.selectedVault == v ? Color.indigo.opacity(0.2) : Color.gray.opacity(0.12))
                                    .foregroundColor(repository.selectedVault == v ? .indigo : .primary)
                                    .clipShape(Capsule())
                            }
                        }
                    }
                    .padding(.horizontal, 16)
                    .padding(.vertical, 6)
                }

                // Category Filter Pills
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 8) {
                        ForEach(categories, id: \.self) { cat in
                            Button(action: {
                                generator.impactOccurred()
                                selectedTab = cat
                            }) {
                                Text(cat)
                                    .font(.system(size: 13, weight: selectedTab == cat ? .semibold : .regular))
                                    .padding(.horizontal, 12)
                                    .padding(.vertical, 6)
                                    .background(selectedTab == cat ? Color.indigo : Color.clear)
                                    .foregroundColor(selectedTab == cat ? .white : .secondary)
                                    .clipShape(RoundedRectangle(cornerRadius: 10))
                            }
                        }
                    }
                    .padding(.horizontal, 16)
                    .padding(.vertical, 4)
                }

                // Vault Items List
                if filteredItems.isEmpty {
                    VStack(spacing: 12) {
                        Spacer()
                        Image(systemName: "lock.slash")
                            .font(.system(size: 40))
                            .foregroundColor(.gray)
                        Text("No Items in \(selectedTab)")
                            .font(.headline)
                            .foregroundColor(.secondary)
                        Button("Add First Item") {
                            showAddSheet = true
                        }
                        .buttonStyle(.borderedProminent)
                        .tint(.indigo)
                        Spacer()
                    }
                } else {
                    List {
                        ForEach(filteredItems) { item in
                            VaultItemRow(item: item, copiedId: $copiedItemId)
                                .swipeActions(edge: .trailing, allowsFullSwipe: true) {
                                    Button(role: .destructive) {
                                        repository.deleteItem(id: item.id)
                                    } label: {
                                        Label("Delete", systemImage: "trash")
                                    }

                                    Button {
                                        repository.togglePin(id: item.id)
                                    } label: {
                                        Label(item.isPinned ? "Unstar" : "Favorite", systemImage: item.isPinned ? "star.slash" : "star.fill")
                                    }
                                    .tint(.yellow)
                                }
                        }
                    }
                    .listStyle(.insetGrouped)
                }
            }
            .searchable(text: $searchQuery, prompt: "Search passwords, passkeys...")
            .navigationTitle("Vault")
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button(action: { repository.lockVault() }) {
                        Image(systemName: "lock.fill")
                            .foregroundColor(.red)
                    }
                }

                ToolbarItemGroup(placement: .navigationBarTrailing) {
                    NavigationLink(destination: HealthDashboardView()) {
                        Image(systemName: "heart.text.square.fill")
                            .foregroundColor(.green)
                    }

                    NavigationLink(destination: SettingsView()) {
                        Image(systemName: "gearshape.fill")
                            .foregroundColor(.gray)
                    }

                    Button(action: { showAddSheet = true }) {
                        Image(systemName: "plus.circle.fill")
                            .font(.title3)
                            .foregroundColor(.indigo)
                    }
                }
            }
            .sheet(isPresented: $showAddSheet) {
                AddItemView()
            }
        }
    }
}

struct VaultItemRow: View {
    let item: VaultItem
    @Binding var copiedId: String?
    private let generator = UIImpactFeedbackGenerator(style: .light)

    var body: some View {
        HStack(spacing: 14) {
            ZStack {
                RoundedRectangle(cornerRadius: 12)
                    .fill(item.type == "Passkeys" ? Color.purple.opacity(0.15) : (item.type == "Secure Notes" ? Color.green.opacity(0.15) : Color.indigo.opacity(0.15)))
                    .frame(width: 42, height: 42)

                Image(systemName: item.type == "Passkeys" ? "key.fill" : (item.type == "Secure Notes" ? "note.text" : (item.type == "Credit Cards" ? "creditcard.fill" : "person.badge.key.fill")))
                    .foregroundColor(item.type == "Passkeys" ? .purple : (item.type == "Secure Notes" ? .green : .indigo))
            }

            VStack(alignment: .leading, spacing: 3) {
                HStack {
                    Text(item.title)
                        .font(.system(size: 15, weight: .semibold))
                    if item.isPinned {
                        Image(systemName: "star.fill")
                            .font(.system(size: 11))
                            .foregroundColor(.yellow)
                    }
                }

                Text(item.username.isEmpty ? item.type : item.username)
                    .font(.system(size: 12, design: .monospaced))
                    .foregroundColor(.secondary)
            }

            Spacer()

            if !item.password.isEmpty {
                Button(action: {
                    UIPasteboard.general.string = item.password
                    generator.impactOccurred()
                    copiedId = item.id
                    DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
                        if copiedId == item.id { copiedId = nil }
                    }
                }) {
                    Image(systemName: copiedId == item.id ? "checkmark.circle.fill" : "doc.on.doc")
                        .foregroundColor(copiedId == item.id ? .green : .indigo)
                        .padding(8)
                }
                .buttonStyle(.plain)
            }
        }
        .padding(.vertical, 2)
    }
}
