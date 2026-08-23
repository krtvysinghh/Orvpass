
package com.orvpass

object OrvpassBridge {

    init {
        System.loadLibrary("orvpass")
    }

    external fun version(): String
    external fun unlock(password: String): Boolean
    external fun lock()
}
