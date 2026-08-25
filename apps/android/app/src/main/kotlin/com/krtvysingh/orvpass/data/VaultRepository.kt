package com.krtvysingh.orvpass.data

import android.content.Context
import com.krtvysingh.orvpass.bridge.OrvpassNativeBridge
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import java.io.File
import java.security.SecureRandom
import java.util.UUID

class VaultRepository(private val context: Context) {

    private val json = Json { ignoreUnknownKeys = true }
    private val vaultFile: File by lazy {
        File(context.filesDir, "orvpass_vault.enc")
    }

    private var activePassword: String? = null

    private val _vaultStatus = MutableStateFlow(VaultStatus(exists = false, unlocked = false))
    val vaultStatus: StateFlow<VaultStatus> = _vaultStatus.asStateFlow()

    private val _items = MutableStateFlow<List<VaultItem>>(emptyList())
    val items: StateFlow<List<VaultItem>> = _items.asStateFlow()

    private val _searchQuery = MutableStateFlow("")
    val searchQuery: StateFlow<String> = _searchQuery.asStateFlow()

    private val _selectedTab = MutableStateFlow("All Items")
    val selectedTab: StateFlow<String> = _selectedTab.asStateFlow()

    private val _pinnedIds = MutableStateFlow<Set<String>>(emptySet())

    suspend fun checkStatus() = withContext(Dispatchers.IO) {
        val exists = vaultFile.exists() && vaultFile.length() > 0
        try {
            val statusJson = OrvpassNativeBridge.checkVaultStatus(vaultFile.absolutePath)
            val obj = json.parseToJsonElement(statusJson).jsonObject
            val nativeExists = obj["exists"]?.jsonPrimitive?.content?.toBoolean() ?: exists
            val nativeUnlocked = obj["unlocked"]?.jsonPrimitive?.content?.toBoolean() ?: false
            _vaultStatus.value = VaultStatus(exists = nativeExists || exists, unlocked = nativeUnlocked)
            if (nativeUnlocked) {
                loadItems()
            }
        } catch (t: Throwable) {
            _vaultStatus.value = VaultStatus(exists = exists, unlocked = activePassword != null)
        }
    }

    suspend fun createVault(password: String): Boolean = withContext(Dispatchers.IO) {
        activePassword = password
        var success = false

        // 1. Try native JNI
        try {
            val ok = OrvpassNativeBridge.createVault(vaultFile.absolutePath, password)
            if (ok) {
                success = true
            }
        } catch (t: Throwable) {
            // JNI fallback
        }

        // 2. Pure-Kotlin AES-GCM engine fallback
        if (!success) {
            try {
                LocalCryptoEngine.saveVault(vaultFile, password, emptyList())
                success = true
            } catch (t: Throwable) {
                t.printStackTrace()
            }
        }

        if (success) {
            _vaultStatus.value = VaultStatus(exists = true, unlocked = true)
            loadItems()
        }
        success
    }

    suspend fun unlockVault(password: String): Boolean = withContext(Dispatchers.IO) {
        var success = false

        // 1. Try native JNI
        try {
            val ok = OrvpassNativeBridge.unlockVault(vaultFile.absolutePath, password)
            if (ok) {
                activePassword = password
                success = true
            }
        } catch (t: Throwable) {
            // JNI fallback
        }

        // 2. Pure-Kotlin AES-GCM engine fallback
        if (!success) {
            try {
                val loaded = LocalCryptoEngine.loadVault(vaultFile, password)
                activePassword = password
                _items.value = loaded
                success = true
            } catch (t: Throwable) {
                // Incorrect password or invalid crypto tag
            }
        }

        if (success) {
            _vaultStatus.value = VaultStatus(exists = true, unlocked = true)
            loadItems()
        }
        success
    }

    suspend fun unlockWithBiometric(): Boolean = withContext(Dispatchers.IO) {
        _vaultStatus.value = VaultStatus(exists = true, unlocked = true)
        loadItems()
        true
    }

    suspend fun lockVault() = withContext(Dispatchers.IO) {
        try {
            OrvpassNativeBridge.lockVault()
        } catch (t: Throwable) {
            t.printStackTrace()
        }
        activePassword = null
        val exists = vaultFile.exists() && vaultFile.length() > 0
        _vaultStatus.value = VaultStatus(exists = exists, unlocked = false)
        _items.value = emptyList()
    }

    suspend fun loadItems() = withContext(Dispatchers.IO) {
        var loadedFromNative = false
        try {
            val jsonStr = OrvpassNativeBridge.getItemsJson()
            if (jsonStr.isNotEmpty() && jsonStr != "[]") {
                val array = json.parseToJsonElement(jsonStr).jsonArray
                val parsed = array.map { element ->
                    val obj = element.jsonObject
                    val id = obj["id"]?.jsonPrimitive?.content ?: ""
                    val title = obj["title"]?.jsonPrimitive?.content ?: "Untitled"
                    val dataObj = obj["data"]?.jsonObject

                    var type = "Logins"
                    var user = ""
                    var pass = ""
                    var notes = ""
                    var cc = ""
                    var expM = "12"
                    var expY = "28"

                    if (dataObj != null) {
                        if (dataObj.containsKey("Login")) {
                            type = "Logins"
                            val login = dataObj["Login"]?.jsonObject
                            user = login?.get("username")?.jsonPrimitive?.content ?: ""
                            pass = login?.get("password")?.jsonPrimitive?.content ?: ""
                        } else if (dataObj.containsKey("SecureNote")) {
                            type = "Secure Notes"
                            val note = dataObj["SecureNote"]?.jsonObject
                            notes = note?.get("content")?.jsonPrimitive?.content ?: ""
                        } else if (dataObj.containsKey("CreditCard")) {
                            type = "Credit Cards"
                            val card = dataObj["CreditCard"]?.jsonObject
                            user = card?.get("cardholder_name")?.jsonPrimitive?.content ?: ""
                            cc = card?.get("card_number")?.jsonPrimitive?.content ?: ""
                            expM = card?.get("expiration_month")?.jsonPrimitive?.content ?: "12"
                            expY = card?.get("expiration_year")?.jsonPrimitive?.content ?: "28"
                            pass = card?.get("cvv")?.jsonPrimitive?.content ?: ""
                        }
                    }

                    VaultItem(
                        id = id,
                        title = title,
                        username = user,
                        password = pass,
                        notes = notes,
                        cc = cc,
                        expMonth = expM,
                        expYear = expY,
                        type = type,
                        isPinned = _pinnedIds.value.contains(id)
                    )
                }
                _items.value = parsed
                loadedFromNative = true
            }
        } catch (t: Throwable) {
            // Native fallback
        }

        if (!loadedFromNative && activePassword != null && vaultFile.exists()) {
            try {
                val fallbackItems = LocalCryptoEngine.loadVault(vaultFile, activePassword!!)
                _items.value = fallbackItems
            } catch (t: Throwable) {
                t.printStackTrace()
            }
        }
    }

    suspend fun addItem(
        type: String,
        title: String,
        username: String,
        pass: String,
        notes: String,
        cc: String,
        expMonth: String,
        expYear: String
    ): Boolean = withContext(Dispatchers.IO) {
        var added = false
        try {
            val ok = OrvpassNativeBridge.addItem(
                type = type,
                title = title,
                username = username,
                password = pass,
                notes = notes,
                cc = cc,
                expMonth = expMonth,
                expYear = expYear
            )
            if (ok) {
                loadItems()
                added = true
            }
        } catch (t: Throwable) {
            // Native fallback
        }

        if (!added) {
            val newItem = VaultItem(
                id = UUID.randomUUID().toString(),
                title = title,
                username = username,
                password = pass,
                notes = notes,
                cc = cc,
                expMonth = expMonth,
                expYear = expYear,
                type = type,
                isPinned = false
            )
            val updated = _items.value + newItem
            _items.value = updated
            if (activePassword != null) {
                try {
                    LocalCryptoEngine.saveVault(vaultFile, activePassword!!, updated)
                    added = true
                } catch (t: Throwable) {
                    t.printStackTrace()
                }
            } else {
                added = true
            }
        }
        added
    }

    suspend fun deleteItem(id: String): Boolean = withContext(Dispatchers.IO) {
        var deleted = false
        try {
            val ok = OrvpassNativeBridge.deleteItem(id)
            if (ok) {
                loadItems()
                deleted = true
            }
        } catch (t: Throwable) {
            // Native fallback
        }

        if (!deleted) {
            val updated = _items.value.filter { it.id != id }
            _items.value = updated
            if (activePassword != null) {
                try {
                    LocalCryptoEngine.saveVault(vaultFile, activePassword!!, updated)
                    deleted = true
                } catch (t: Throwable) {
                    t.printStackTrace()
                }
            } else {
                deleted = true
            }
        }
        deleted
    }

    fun togglePin(id: String) {
        val set = _pinnedIds.value.toMutableSet()
        if (set.contains(id)) set.remove(id) else set.add(id)
        _pinnedIds.value = set
        _items.value = _items.value.map {
            if (it.id == id) it.copy(isPinned = set.contains(id)) else it
        }
    }

    fun setSearchQuery(query: String) {
        _searchQuery.value = query
    }

    fun setSelectedTab(tab: String) {
        _selectedTab.value = tab
    }

    fun generatePassword(length: Int): String {
        try {
            val native = OrvpassNativeBridge.generatePassword(length)
            if (native.isNotEmpty()) return native
        } catch (t: Throwable) {
            // Fallback
        }

        val charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-="
        val secureRandom = SecureRandom()
        val len = if (length > 0) length else 18
        return (1..len)
            .map { charset[secureRandom.nextInt(charset.length)] }
            .joinToString("")
    }

    fun calculateHealth(): HealthStats {
        val loginItems = _items.value.filter { it.type == "Logins" }
        var weak = 0
        var reused = 0
        val passCounts = mutableMapOf<String, Int>()

        loginItems.forEach { item ->
            if (item.password.isNotEmpty() && item.password.length < 12) weak++
            if (item.password.isNotEmpty()) {
                passCounts[item.password] = (passCounts[item.password] ?: 0) + 1
            }
        }

        passCounts.values.forEach { if (it > 1) reused += it }

        val total = loginItems.size
        var score = 100
        if (total > 0) {
            val penalty = ((weak.toFloat() / total) * 40) + ((reused.toFloat() / total) * 40)
            score = (100 - penalty).toInt().coerceIn(10, 100)
        }

        return HealthStats(
            score = score,
            total = total,
            weak = weak,
            reused = reused,
            strong = (total - weak - if (reused > 0) 1 else 0).coerceAtLeast(0)
        )
    }

    fun exportCsv(): String {
        val sb = StringBuilder("title,username,password,notes,type\n")
        _items.value.forEach { item ->
            val escapedNotes = item.notes.replace("\"", "'")
            sb.append("\"${item.title}\",\"${item.username}\",\"${item.password}\",\"$escapedNotes\",\"${item.type}\"\n")
        }
        return sb.toString()
    }

    fun exportJson(): String {
        return try {
            json.encodeToString(kotlinx.serialization.builtins.ListSerializer(VaultItem.serializer()), _items.value)
        } catch (t: Throwable) {
            "[]"
        }
    }

    fun exportHtml(): String {
        val cards = StringBuilder()
        _items.value.forEach { item ->
            cards.append("<div style='background:#131d33;padding:1rem;border-radius:12px;margin-bottom:1rem;'>")
            cards.append("<h3>${item.title} (${item.type})</h3>")
            if (item.username.isNotEmpty()) cards.append("<p><b>Username:</b> <code>${item.username}</code></p>")
            if (item.password.isNotEmpty()) cards.append("<p><b>Password:</b> <code>${item.password}</code></p>")
            if (item.notes.isNotEmpty()) cards.append("<p><b>Notes:</b> ${item.notes}</p>")
            cards.append("</div>")
        }
        return "<!DOCTYPE html><html><head><title>Orvpass Emergency Backup</title><style>body{font-family:sans-serif;background:#070b14;color:#f8fafc;padding:2rem;}</style></head><body><h1>Orvpass Vault Backup</h1>$cards</body></html>"
    }

    suspend fun importFromText(text: String): Int = withContext(Dispatchers.IO) {
        var count = 0
        val trimmed = text.trim()
        val importedList = mutableListOf<VaultItem>()

        if (trimmed.startsWith("[")) {
            try {
                val parsed = json.decodeFromString(kotlinx.serialization.builtins.ListSerializer(VaultItem.serializer()), trimmed)
                importedList.addAll(parsed)
            } catch (t: Throwable) {}
        } else {
            val lines = trimmed.split("\n")
            lines.drop(1).forEach { line ->
                if (line.isNotBlank()) {
                    val parts = line.split(",").map { it.trim().removeSurrounding("\"") }
                    if (parts.size >= 2) {
                        val newItem = VaultItem(
                            id = UUID.randomUUID().toString(),
                            title = parts[0],
                            username = if (parts.size > 1) parts[1] else "",
                            password = if (parts.size > 2) parts[2] else "",
                            notes = if (parts.size > 3) parts[3] else "",
                            type = if (parts.size > 4) parts[4] else "Logins"
                        )
                        importedList.add(newItem)
                    }
                }
            }
        }

        if (importedList.isNotEmpty()) {
            val current = _items.value.toMutableList()
            importedList.forEach { newItem ->
                if (current.none { it.title == newItem.title && it.username == newItem.username }) {
                    current.add(0, newItem)
                    count++
                }
            }
            _items.value = current
            if (activePassword != null) {
                try {
                    LocalCryptoEngine.saveVault(vaultFile, activePassword!!, current)
                } catch (t: Throwable) {}
            }
        }
        count
    }
}
