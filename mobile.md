# Wave 5: Mobile Issues (Testnet) for Soter

Wave 5 focuses on getting the mobile app ready for **Testnet** demos with reliable wallet/network UX and clear on-chain proof links.

Complexity score scale:
- **100**: Beginner - Screen wiring, config, and UI polish.
- **150**: Intermediate - Network/wallet flows and integration.
- **200**: Advanced - Offline-first queues and robust sync.

---

### Issue 1: Testnet Environment Configuration (API + Contract ID)
**Labels:** Mobile, testnet, integration, help wanted
**Complexity Score: 150**

#### Description
Ensure the mobile app reads and displays the correct Testnet configuration (backend URL + contract ID).

#### Requirements
- Add env config for backend base URL, `SOROBAN_NETWORK=testnet`, and contract ID.
- Add a visible indicator in Settings or Diagnostics showing current network and contract ID.
- Ensure misconfiguration fails gracefully with actionable messaging.

---

### Issue 2: Wallet Network Mismatch Guard (Mobile)
**Labels:** Mobile, wallet, testnet, security
**Complexity Score: 150**

#### Description
Prevent on-chain actions when the wallet is not on Testnet.

#### Requirements
- Detect wallet network mismatch.
- Block actions that require signatures when mismatched and provide remediation.
- Add tests for mismatch and recovery flows.

---

### Issue 3: Testnet Explorer Links for Claim Receipt
**Labels:** Mobile, Soroban, testnet, ux, good first issue
**Complexity Score: 100**

#### Description
Make it easy to verify claims by linking to testnet explorer for tx hash and addresses.

#### Requirements
- Add a utility that generates correct explorer URLs for testnet.
- Include tx hash and contract link on claim receipt screen.
- Add copy-to-clipboard actions for tx hash and contract ID.

---

### Issue 4: End-to-End “Scan → Details → Claim” Testnet Demo Flow
**Labels:** Mobile, testnet, integration, help wanted
**Complexity Score: 150**

#### Description
Polish the main field workflow so it works reliably on Testnet and is demo-ready.

#### Requirements
- Ensure QR scanning routes correctly into package details.
- Ensure claim action shows progress, final status, and receipt.
- Add at least one integration test for the navigation and claim flow.

---

### Issue 5: Background Retry for Pending Claim Submissions
**Labels:** Mobile, testnet, reliability
**Complexity Score: 200**

#### Description
Network instability should not break claims. Pending submissions should retry safely when connectivity returns.

#### Requirements
- Add a local queue for pending submissions with idempotency handling.
- Retry automatically on reconnect and allow manual retry.
- Show user-facing status for queued vs submitted vs failed.

---

### Issue 6: Diagnostics Screen for Testnet Support
**Labels:** Mobile, testnet, devex, good first issue
**Complexity Score: 100**

#### Description
Support quick debugging during Testnet pilots by exposing safe diagnostics.

#### Requirements
- Show app version, environment, API reachability, network state, and configured contract ID.
- Add copy-to-clipboard for non-sensitive diagnostics.
- Ensure no secrets/tokens are displayed.

---

### Issue 7: Push Notification Deep Links for Testnet Events
**Labels:** Mobile, testnet, ux, integration
**Complexity Score: 150**

#### Description
When a claim or verification updates, notifications should deep link to the correct screen.

#### Requirements
- Add deep link handling for claim receipts and package details.
- Handle cold start vs background taps reliably.
- Add tests for deep link routing.

---

### Issue 8: Testnet Faucet Helper in Mobile Settings
**Labels:** Mobile, testnet, ux, good first issue
**Complexity Score: 100**

#### Description
Help users obtain Testnet XLM quickly during demos.

#### Requirements
- Add a “Get Testnet XLM” section that links to official faucet tools.
- Show only when network is testnet.
- Keep copy concise and accessible.
