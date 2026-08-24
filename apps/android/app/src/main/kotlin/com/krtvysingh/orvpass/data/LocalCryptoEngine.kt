package com.krtvysingh.orvpass.data

import android.util.Base64
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.io.File
import java.security.SecureRandom
import java.security.spec.KeySpec
import javax.crypto.Cipher
import javax.crypto.SecretKeyFactory
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.PBEKeySpec
import javax.crypto.spec.SecretKeySpec

object LocalCryptoEngine {

    private const val ITERATIONS = 100_000
    private const val KEY_LENGTH = 256
    private const val GCM_IV_LENGTH = 12
    private const val GCM_TAG_LENGTH = 128
    private val json = Json { ignoreUnknownKeys = true; prettyPrint = false }

    fun deriveKey(password: String, salt: ByteArray): SecretKeySpec {
        val spec: KeySpec = PBEKeySpec(password.toCharArray(), salt, ITERATIONS, KEY_LENGTH)
        val factory = SecretKeyFactory.getInstance("PBKDF2WithHmacSHA256")
        val keyBytes = factory.generateSecret(spec).encoded
        return SecretKeySpec(keyBytes, "AES")
    }

    fun encrypt(plaintext: String, key: SecretKeySpec): ByteArray {
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        val iv = ByteArray(GCM_IV_LENGTH)
        SecureRandom().nextBytes(iv)
        val spec = GCMParameterSpec(GCM_TAG_LENGTH, iv)
        cipher.init(Cipher.ENCRYPT_MODE, key, spec)
        val cipherBytes = cipher.doFinal(plaintext.toByteArray(Charsets.UTF_8))

        // Format: [12 bytes IV][Ciphertext + Tag]
        return iv + cipherBytes
    }

    fun decrypt(encryptedData: ByteArray, key: SecretKeySpec): String {
        require(encryptedData.size > GCM_IV_LENGTH) { "Encrypted data too short" }
        val iv = encryptedData.copyOfRange(0, GCM_IV_LENGTH)
        val cipherBytes = encryptedData.copyOfRange(GCM_IV_LENGTH, encryptedData.size)

        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        val spec = GCMParameterSpec(GCM_TAG_LENGTH, iv)
        cipher.init(Cipher.DECRYPT_MODE, key, spec)
        val decryptedBytes = cipher.doFinal(cipherBytes)
        return String(decryptedBytes, Charsets.UTF_8)
    }

    fun saveVault(vaultFile: File, password: String, items: List<VaultItem>) {
        val saltFile = File(vaultFile.parentFile, "${vaultFile.name}.salt")
        val salt = if (saltFile.exists()) {
            saltFile.readBytes()
        } else {
            val s = ByteArray(16)
            SecureRandom().nextBytes(s)
            saltFile.writeBytes(s)
            s
        }

        val key = deriveKey(password, salt)
        val itemsJson = json.encodeToString(items)
        val encrypted = encrypt(itemsJson, key)
        vaultFile.writeBytes(encrypted)
    }

    fun loadVault(vaultFile: File, password: String): List<VaultItem> {
        if (!vaultFile.exists()) return emptyList()
        val saltFile = File(vaultFile.parentFile, "${vaultFile.name}.salt")
        if (!saltFile.exists()) return emptyList()

        val salt = saltFile.readBytes()
        val key = deriveKey(password, salt)
        val encrypted = vaultFile.readBytes()
        val decryptedJson = decrypt(encrypted, key)

        return json.decodeFromString<List<VaultItem>>(decryptedJson)
    }
}
