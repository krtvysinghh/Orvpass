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

class VaultRepository(private val context: Context) {

    private val json = Json { ignoreUnknownKeys = true }
    private val vaultPath: String by lazy {
        File(context.filesDir, "orvpass_vault.enc").absolutePath
    }

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
        try {
            val statusJson = OrvpassNativeBridge.checkVaultStatus(vaultPath)
            val obj = json.parseToJsonElement(statusJson).jsonObject
            val exists = obj["exists"]?.jsonPrimitive?.content?.toBoolean() ?: false
            val unlocked = obj["unlocked"]?.jsonPrimitive?.content?.toBoolean() ?: false
            _vaultStatus.value = VaultStatus(exists = exists, unlocked = unlocked)
            if (unlocked) {
                loadItems()
            }
        } catch (t: Throwable) {
            t.printStackTrace()
            val exists = File(vaultPath).exists()
            _vaultStatus.value = VaultStatus(exists = exists, unlocked = false)
        }
    }

    suspend fun createVault(password: String): Boolean = withContext(Dispatchers.IO) {
        try {
            val ok = OrvpassNativeBridge.createVault(vaultPath, password)
            if (ok) {
                _vaultStatus.value = VaultStatus(exists = true, unlocked = true)
                loadItems()
            }
            ok
        } catch (t: Throwable) {
            t.printStackTrace()
            false
        }
    }

    suspend fun unlockVault(password: String): Boolean = withContext(Dispatchers.IO) {
        try {
            val ok = OrvpassNativeBridge.unlockVault(vaultPath, password)
            if (ok) {
                _vaultStatus.value = VaultStatus(exists = true, unlocked = true)
                loadItems()
            }
            ok
        } catch (t: Throwable) {
            t.printStackTrace()
            false
        }
    }

    suspend fun lockVault() = withContext(Dispatchers.IO) {
        try {
            OrvpassNativeBridge.lockVault()
        } catch (t: Throwable) {
            t.printStackTrace()
        }
        val exists = File(vaultPath).exists()
        _vaultStatus.value = VaultStatus(exists = exists, unlocked = false)
        _items.value = emptyList()
    }

    suspend fun loadItems() = withContext(Dispatchers.IO) {
        try {
            val jsonStr = OrvpassNativeBridge.getItemsJson()
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
        } catch (t: Throwable) {
            t.printStackTrace()
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
            }
            ok
        } catch (t: Throwable) {
            t.printStackTrace()
            false
        }
    }

    suspend fun deleteItem(id: String): Boolean = withContext(Dispatchers.IO) {
        try {
            val ok = OrvpassNativeBridge.deleteItem(id)
            if (ok) {
                loadItems()
            }
            ok
        } catch (t: Throwable) {
            t.printStackTrace()
            false
        }
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
        return try {
            OrvpassNativeBridge.generatePassword(length)
        } catch (t: Throwable) {
            // Fallback entropy generator
            val charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+"
            (1..length).map { charset.random() }.joinToString("")
        }
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
}
