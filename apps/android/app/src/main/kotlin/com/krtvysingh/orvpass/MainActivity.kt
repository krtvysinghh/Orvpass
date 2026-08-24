package com.krtvysingh.orvpass

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import com.krtvysingh.orvpass.data.VaultRepository

class MainActivity : FragmentActivity() {

    private lateinit var repository: VaultRepository

    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        repository = VaultRepository(applicationContext)

        setContent {
            OrvpassApp(
                repository = repository,
                onBiometricPromptRequest = { onSuccess ->
                    showBiometricPrompt(onSuccess)
                }
            )
        }
    }

    private fun showBiometricPrompt(onSuccess: () -> Unit) {
        try {
            val biometricManager = BiometricManager.from(this)
            val authenticators = BiometricManager.Authenticators.BIOMETRIC_STRONG or 
                                 BiometricManager.Authenticators.BIOMETRIC_WEAK or 
                                 BiometricManager.Authenticators.DEVICE_CREDENTIAL

            val canAuth = biometricManager.canAuthenticate(authenticators)
            if (canAuth != BiometricManager.BIOMETRIC_SUCCESS) {
                return
            }

            val executor = ContextCompat.getMainExecutor(this)
            val biometricPrompt = BiometricPrompt(
                this,
                executor,
                object : BiometricPrompt.AuthenticationCallback() {
                    override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                        super.onAuthenticationSucceeded(result)
                        onSuccess()
                    }
                    override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                        super.onAuthenticationError(errorCode, errString)
                    }
                }
            )

            // NOTE: In AndroidX Biometric, when DEVICE_CREDENTIAL is used, setNegativeButtonText MUST NOT be called.
            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle("Unlock Orvpass")
                .setSubtitle("Authenticate with biometrics or screen lock")
                .setAllowedAuthenticators(authenticators)
                .build()

            biometricPrompt.authenticate(promptInfo)
        } catch (t: Throwable) {
            t.printStackTrace()
        }
    }
}
