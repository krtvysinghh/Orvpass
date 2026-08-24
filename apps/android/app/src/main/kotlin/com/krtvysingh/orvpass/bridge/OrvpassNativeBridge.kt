package com.krtvysingh.orvpass.bridge

object OrvpassNativeBridge {
    init {
        try {
            System.loadLibrary("orvpass_android")
        } catch (e: UnsatisfiedLinkError) {
            e.printStackTrace()
        }
    }

    external fun checkVaultStatus(path: String): String
    external fun createVault(path: String, password: String): Boolean
    external fun unlockVault(path: String, password: String): Boolean
    external fun lockVault(): Boolean
    external fun getItemsJson(): String
    external fun addItem(
        type: String,
        title: String,
        username: String,
        password: String,
        notes: String,
        cc: String,
        expMonth: String,
        expYear: String
    ): Boolean
    external fun deleteItem(id: String): Boolean
    external fun generatePassword(length: Int): String
}
