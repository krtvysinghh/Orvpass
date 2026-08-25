import UIKit
import AuthenticationServices

public class CredentialProviderViewController: ASCredentialProviderViewController {

    public override func prepareCredentialList(for serviceIdentifiers: [ASCredentialServiceIdentifier]) {
        // Prepare credential identities for system AutoFill suggestion
        let domain = serviceIdentifiers.first?.identifier ?? "orvpass.local"
        
        let passwordIdentity = ASPasswordCredentialIdentity(
            serviceIdentifier: ASCredentialServiceIdentifier(identifier: domain, type: .domain),
            user: "krtvysinghh",
            recordIdentifier: "cred-1"
        )

        let passkeyIdentity = ASPasskeyCredentialIdentity(
            relyingPartyIdentifier: domain,
            userName: "krtvysinghh",
            credentialID: Data("orvpass-passkey-id-1".utf8),
            userHandle: Data("user-handle-1".utf8),
            recordIdentifier: "passkey-1"
        )

        extensionContext.provideCredentialWithoutUserInteraction(for: passwordIdentity)
    }

    public override func provideCredentialWithoutUserInteraction(for credentialIdentity: ASPasswordCredentialIdentity) {
        let passwordCredential = ASPasswordCredential(user: credentialIdentity.user, password: "OrvpassSecure2026!")
        extensionContext.completeRequest(withSelectedCredential: passwordCredential, completionHandler: nil)
    }

    public override func provideCredentialWithoutUserInteraction(for credentialIdentity: ASPasskeyCredentialIdentity) {
        let assertion = ASPasskeyAssertionCredential(
            userHandle: credentialIdentity.userHandle,
            relyingPartyIdentifier: credentialIdentity.relyingPartyIdentifier,
            credentialID: credentialIdentity.credentialID,
            authenticatorData: Data(repeating: 0x01, count: 37),
            signature: Data(repeating: 0x02, count: 64),
            clientDataHash: Data(repeating: 0x03, count: 32)
        )
        extensionContext.completeRequest(withSelectedCredential: assertion, completionHandler: nil)
    }
}
