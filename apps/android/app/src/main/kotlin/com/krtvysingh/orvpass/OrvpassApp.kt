package com.krtvysingh.orvpass

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import com.krtvysingh.orvpass.data.VaultRepository
import com.krtvysingh.orvpass.ui.screens.*
import com.krtvysingh.orvpass.ui.theme.OrvpassTheme
import kotlinx.coroutines.launch

@Composable
fun OrvpassApp(
    repository: VaultRepository,
    onBiometricPromptRequest: (() -> Unit) -> Unit
) {
    val scope = rememberCoroutineScope()
    val vaultStatus by repository.vaultStatus.collectAsState()
    val items by repository.items.collectAsState()
    val searchQuery by repository.searchQuery.collectAsState()
    val selectedTab by repository.selectedTab.collectAsState()

    var authError by remember { mutableStateOf<String?>(null) }
    var showAddItemSheet by remember { mutableStateOf(false) }
    var currentScreen by remember { mutableStateOf<String>("vault") } // "vault", "settings", "health"
    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(Unit) {
        repository.checkStatus()
    }

    OrvpassTheme {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            when {
                !vaultStatus.exists && !vaultStatus.unlocked -> {
                    SetupVaultScreen(
                        onCreateVault = { password ->
                            scope.launch {
                                authError = null
                                val ok = repository.createVault(password)
                                if (!ok) authError = "Failed to create vault"
                            }
                        },
                        error = authError
                    )
                }
                vaultStatus.exists && !vaultStatus.unlocked -> {
                    UnlockVaultScreen(
                        onUnlockVault = { password ->
                            scope.launch {
                                authError = null
                                val ok = repository.unlockVault(password)
                                if (!ok) authError = "Incorrect master password"
                            }
                        },
                        onBiometricUnlock = {
                            onBiometricPromptRequest {
                                scope.launch {
                                    authError = null
                                    repository.unlockWithBiometric()
                                }
                            }
                        },
                        error = authError
                    )
                }
                else -> {
                    when (currentScreen) {
                        "settings" -> {
                            SettingsScreen(
                                onBack = { currentScreen = "vault" },
                                onHealthClick = { currentScreen = "health" }
                            )
                        }
                        "health" -> {
                            HealthScreen(
                                healthStats = repository.calculateHealth(),
                                onBack = { currentScreen = "vault" }
                            )
                        }
                        else -> {
                            VaultListScreen(
                                items = items,
                                selectedTab = selectedTab,
                                searchQuery = searchQuery,
                                onTabSelected = { repository.setSelectedTab(it) },
                                onSearchQueryChanged = { repository.setSearchQuery(it) },
                                onTogglePin = { repository.togglePin(it) },
                                onDeleteItem = { id -> scope.launch { repository.deleteItem(id) } },
                                onAddItemClick = { showAddItemSheet = true },
                                onSettingsClick = { currentScreen = "settings" },
                                onLockClick = { scope.launch { repository.lockVault() } },
                                snackbarHostState = snackbarHostState
                            )

                            if (showAddItemSheet) {
                                AddItemSheet(
                                    onDismiss = { showAddItemSheet = false },
                                    onSaveItem = { type, title, username, pass, notes, cc, expMonth, expYear ->
                                        scope.launch {
                                            repository.addItem(type, title, username, pass, notes, cc, expMonth, expYear)
                                        }
                                    },
                                    onGeneratePassword = { length ->
                                        repository.generatePassword(length)
                                    }
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
