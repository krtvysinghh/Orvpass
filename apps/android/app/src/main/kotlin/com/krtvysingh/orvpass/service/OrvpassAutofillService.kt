package com.krtvysingh.orvpass.service

import android.app.assist.AssistStructure
import android.os.CancellationSignal
import android.service.autofill.*
import android.view.autofill.AutofillId
import android.view.autofill.AutofillValue
import android.widget.RemoteViews
import com.krtvysingh.orvpass.R
import com.krtvysingh.orvpass.data.VaultItem
import com.krtvysingh.orvpass.data.VaultRepository

class OrvpassAutofillService : AutofillService() {

    override fun onFillRequest(
        request: FillRequest,
        cancellationSignal: CancellationSignal,
        callback: FillCallback
    ) {
        val structure = request.fillContexts.lastOrNull()?.structure ?: run {
            callback.onSuccess(null)
            return
        }

        val usernameFields = mutableListOf<AutofillId>()
        val passwordFields = mutableListOf<AutofillId>()

        parseStructure(structure, usernameFields, passwordFields)

        if (usernameFields.isEmpty() && passwordFields.isEmpty()) {
            callback.onSuccess(null)
            return
        }

        val responseBuilder = FillResponse.Builder()
        val items = VaultRepository.getItems(this)

        if (items.isEmpty()) {
            callback.onSuccess(null)
            return
        }

        for (item in items.take(5)) {
            val presentation = RemoteViews(packageName, android.R.layout.simple_list_item_2).apply {
                setTextViewText(android.R.id.text1, item.title)
                setTextViewText(android.R.id.text2, item.username.ifEmpty { "Orvpass Secure Credentials" })
            }

            val datasetBuilder = Dataset.Builder(presentation)

            if (usernameFields.isNotEmpty() && item.username.isNotEmpty()) {
                datasetBuilder.setValue(usernameFields.first(), AutofillValue.forText(item.username))
            }
            if (passwordFields.isNotEmpty() && item.password.isNotEmpty()) {
                datasetBuilder.setValue(passwordFields.first(), AutofillValue.forText(item.password))
            }

            responseBuilder.addDataset(datasetBuilder.build())
        }

        callback.onSuccess(responseBuilder.build())
    }

    override fun onSaveRequest(request: SaveRequest, callback: SaveCallback) {
        callback.onSuccess()
    }

    private fun parseStructure(
        structure: AssistStructure,
        usernameFields: MutableList<AutofillId>,
        passwordFields: MutableList<AutofillId>
    ) {
        val nodes = (0 until structure.windowNodeCount).map { structure.getWindowNodeAt(it).rootViewNode }
        val queue = ArrayDeque(nodes)

        while (queue.isNotEmpty()) {
            val node = queue.removeFirst()

            val hints = node.autofillHints?.toList() ?: emptyList()
            val idEntry = node.idEntry?.lowercase() ?: ""
            val hintText = node.hint?.toString()?.lowercase() ?: ""
            val inputType = node.inputType

            val isPassword = hints.contains(android.view.View.AUTOFILL_HINT_PASSWORD) ||
                    idEntry.contains("password") ||
                    hintText.contains("password") ||
                    (inputType and android.text.InputType.TYPE_TEXT_VARIATION_PASSWORD) != 0

            val isUsername = hints.contains(android.view.View.AUTOFILL_HINT_USERNAME) ||
                    hints.contains(android.view.View.AUTOFILL_HINT_EMAIL_ADDRESS) ||
                    idEntry.contains("user") || idEntry.contains("email") || idEntry.contains("login") ||
                    hintText.contains("user") || hintText.contains("email")

            node.autofillId?.let { id ->
                if (isPassword) {
                    passwordFields.add(id)
                } else if (isUsername) {
                    usernameFields.add(id)
                }
            }

            for (i in 0 until node.childCount) {
                node.getChildAt(i)?.let { queue.add(it) }
            }
        }
    }
}
