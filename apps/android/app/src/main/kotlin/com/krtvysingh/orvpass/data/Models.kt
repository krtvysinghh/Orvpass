package com.krtvysingh.orvpass.data

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

@Serializable
enum class ItemType {
    Login,
    SecureNote,
    Custom,
    Totp,
    CreditCard
}

@Serializable
data class VaultItem(
    val id: String,
    val title: String,
    val username: String = "",
    val password: String = "",
    val notes: String = "",
    val cc: String = "",
    val expMonth: String = "12",
    val expYear: String = "28",
    val type: String = "Logins",
    val isPinned: Boolean = false
)

@Serializable
data class VaultStatus(
    val exists: Boolean,
    val unlocked: Boolean
)

@Serializable
data class HealthStats(
    val score: Int,
    val total: Int,
    val weak: Int,
    val reused: Int,
    val strong: Int
)
