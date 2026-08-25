package com.krtvysingh.orvpass.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.krtvysingh.orvpass.ui.theme.IndigoPrimary

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AddItemSheet(
    onDismiss: () -> Unit,
    onSaveItem: (type: String, title: String, username: String, pass: String, notes: String, cc: String, expMonth: String, expYear: String) -> Unit,
    onGeneratePassword: (Int) -> String
) {
    var selectedType by remember { mutableStateOf("Logins") }
    var title by remember { mutableStateOf("") }
    var username by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var notes by remember { mutableStateOf("") }
    var ccNumber by remember { mutableStateOf("") }
    var expMonth by remember { mutableStateOf("12") }
    var expYear by remember { mutableStateOf("28") }

    var showPassword by remember { mutableStateOf(false) }
    var showGenOptions by remember { mutableStateOf(false) }
    var genLength by remember { mutableStateOf(18) }

    var isPassphraseMode by remember { mutableStateOf(false) }

    fun generateDiceware(): String {
        val words = listOf(
            "falcon", "shield", "crypto", "cipher", "matrix", "beacon", "galaxy", "orbit",
            "quantum", "vector", "shadow", "summit", "horizon", "glacier", "phoenix", "aurora",
            "nebula", "zenith", "vortex", "starlight", "timber", "cascade", "dynamo", "solace",
            "granite", "pinnacle", "bastion", "sentinel", "citadel", "velocity", "meridian", "solstice"
        )
        val selected = (1..4).map { words.random() }.joinToString("-")
        val num = (10..99).random()
        return "$selected-$num"
    }

    LaunchedEffect(selectedType) {
        if (selectedType == "Logins" && password.isEmpty()) {
            password = if (isPassphraseMode) generateDiceware() else onGeneratePassword(genLength)
        } else if (selectedType == "Passkeys" && notes.isEmpty()) {
            notes = "FIDO2 / WebAuthn Discoverable Passkey (ES256)"
        }
    }

    ModalBottomSheet(
        onDismissRequest = onDismiss,
        shape = RoundedCornerShape(topStart = 28.dp, topEnd = 28.dp)
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 24.dp)
                .padding(bottom = 32.dp)
                .verticalScroll(rememberScrollState())
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = "Add to Vault",
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold
                )
                IconButton(onClick = onDismiss) {
                    Icon(Icons.Default.Close, contentDescription = "Close")
                }
            }

            Spacer(modifier = Modifier.height(16.dp))

            // Segmented category chips
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                listOf("Logins", "Passkeys", "Secure Notes", "Credit Cards").forEach { type ->
                    val isSelected = selectedType == type
                    FilterChip(
                        selected = isSelected,
                        onClick = { selectedType = type },
                        label = { Text(if (type == "Logins") "Login" else if (type == "Passkeys") "Passkey" else if (type == "Secure Notes") "Note" else "Card", fontSize = 12.sp) },
                        modifier = Modifier.weight(1f),
                        shape = RoundedCornerShape(12.dp)
                    )
                }
            }

            Spacer(modifier = Modifier.height(16.dp))

            OutlinedTextField(
                value = title,
                onValueChange = { title = it },
                label = { Text("Title / Service *") },
                placeholder = { Text("e.g. GitHub, Google, Proton") },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
                singleLine = true
            )

            Spacer(modifier = Modifier.height(12.dp))

            if (selectedType == "Logins") {
                OutlinedTextField(
                    value = username,
                    onValueChange = { username = it },
                    label = { Text("Username / Email") },
                    placeholder = { Text("user@domain.com") },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp),
                    singleLine = true
                )

                Spacer(modifier = Modifier.height(12.dp))

                OutlinedTextField(
                    value = password,
                    onValueChange = { password = it },
                    label = { Text("Password") },
                    visualTransformation = if (showPassword) VisualTransformation.None else PasswordVisualTransformation(),
                    trailingIcon = {
                        Row {
                            IconButton(onClick = { password = if (isPassphraseMode) generateDiceware() else onGeneratePassword(genLength) }) {
                                Icon(Icons.Default.Refresh, contentDescription = "Generate", tint = IndigoPrimary)
                            }
                            IconButton(onClick = { showPassword = !showPassword }) {
                                Icon(
                                    imageVector = if (showPassword) Icons.Default.VisibilityOff else Icons.Default.Visibility,
                                    contentDescription = null
                                )
                            }
                        }
                    },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp),
                    singleLine = true
                )

                TextButton(
                    onClick = { showGenOptions = !showGenOptions },
                    modifier = Modifier.padding(top = 4.dp)
                ) {
                    Text(if (showGenOptions) "Hide Generator Options" else "Customize Password / Diceware Passphrase", fontSize = 12.sp)
                }

                if (showGenOptions) {
                    Surface(
                        shape = RoundedCornerShape(16.dp),
                        color = MaterialTheme.colorScheme.surfaceVariant,
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                horizontalArrangement = Arrangement.SpaceBetween,
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Text("Diceware Passphrase Mode", style = MaterialTheme.typography.bodySmall, fontWeight = FontWeight.SemiBold)
                                Switch(
                                    checked = isPassphraseMode,
                                    onCheckedChange = {
                                        isPassphraseMode = it
                                        password = if (it) generateDiceware() else onGeneratePassword(genLength)
                                    }
                                )
                            }

                            if (!isPassphraseMode) {
                                Text("Length: $genLength", style = MaterialTheme.typography.bodySmall, fontWeight = FontWeight.SemiBold)
                                Slider(
                                    value = genLength.toFloat(),
                                    onValueChange = {
                                        genLength = it.toInt()
                                        password = onGeneratePassword(genLength)
                                    },
                                    valueRange = 8f..64f,
                                    steps = 55
                                )
                            }
                        }
                    }
                }
            } else if (selectedType == "Passkeys") {
                OutlinedTextField(
                    value = username,
                    onValueChange = { username = it },
                    label = { Text("User Identifier / Email") },
                    placeholder = { Text("user@example.com") },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp),
                    singleLine = true
                )

                Spacer(modifier = Modifier.height(12.dp))

                OutlinedTextField(
                    value = notes,
                    onValueChange = { notes = it },
                    label = { Text("Passkey Credential Details") },
                    placeholder = { Text("FIDO2 WebAuthn ES256 Key") },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp)
                )
            } else if (selectedType == "Secure Notes") {
                OutlinedTextField(
                    value = notes,
                    onValueChange = { notes = it },
                    label = { Text("Secure Note Content") },
                    placeholder = { Text("Confidential keys, recovery phrases, notes...") },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(160.dp),
                    shape = RoundedCornerShape(16.dp)
                )
            } else if (selectedType == "Credit Cards") {
                OutlinedTextField(
                    value = username,
                    onValueChange = { username = it },
                    label = { Text("Cardholder Name") },
                    placeholder = { Text("Jane Doe") },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp),
                    singleLine = true
                )

                Spacer(modifier = Modifier.height(12.dp))

                OutlinedTextField(
                    value = ccNumber,
                    onValueChange = { ccNumber = it },
                    label = { Text("Card Number") },
                    placeholder = { Text("4000 1234 5678 9010") },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp),
                    singleLine = true
                )

                Spacer(modifier = Modifier.height(12.dp))

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    OutlinedTextField(
                        value = expMonth,
                        onValueChange = { expMonth = it },
                        label = { Text("Month") },
                        modifier = Modifier.weight(1f),
                        shape = RoundedCornerShape(16.dp),
                        singleLine = true
                    )
                    OutlinedTextField(
                        value = expYear,
                        onValueChange = { expYear = it },
                        label = { Text("Year") },
                        modifier = Modifier.weight(1f),
                        shape = RoundedCornerShape(16.dp),
                        singleLine = true
                    )
                    OutlinedTextField(
                        value = password,
                        onValueChange = { password = it },
                        label = { Text("CVV") },
                        visualTransformation = PasswordVisualTransformation(),
                        modifier = Modifier.weight(1f),
                        shape = RoundedCornerShape(16.dp),
                        singleLine = true
                    )
                }
            }

            Spacer(modifier = Modifier.height(24.dp))

            Button(
                onClick = {
                    if (title.isNotBlank()) {
                        onSaveItem(selectedType, title.trim(), username, password, notes, ccNumber, expMonth, expYear)
                        onDismiss()
                    }
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(52.dp),
                shape = RoundedCornerShape(16.dp),
                colors = ButtonDefaults.buttonColors(containerColor = IndigoPrimary)
            ) {
                Text("Save Item", fontWeight = FontWeight.Bold, fontSize = 15.sp)
            }
        }
    }
}
