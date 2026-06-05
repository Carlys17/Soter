# Wave 5: Frontend Issues (Testnet) for Soter

Wave 5 focuses on deploying the frontend for **Testnet**, wiring environment configuration correctly, and polishing the Testnet demo UX.

Complexity score scale:
- **100**: Beginner - Focused UI wiring and environment validation.
- **150**: Intermediate - Data flow and wallet/network UX.
- **200**: Advanced - Cross-cutting deployment or performance work.

---

### Issue 1: Deploy Frontend to Vercel (Testnet Environment Only)
**Labels:** Frontend, deployment, vercel, testnet, help wanted
**Complexity Score: 150**

#### Description
Deploy `app/frontend` to Vercel with **testnet configuration** so reviewers can test end-to-end flows safely.

#### Requirements
- Configure Vercel project with root directory `app/frontend`.
- Add environment variables for testnet (backend URL, Soroban network, contract ID).
- Ensure preview deployments work for PRs.
- Add a minimal smoke checklist for the deployed app.

---

### Issue 2: Environment + Network Indicator (Testnet)
**Labels:** Frontend, testnet, ux, good first issue
**Complexity Score: 100**

#### Description
Make it obvious to users that they are using **Testnet** and which contract is configured.

#### Requirements
- Show environment (preview/production) and network (testnet) in the UI.
- Display the configured contract ID in a copyable, truncated format.
- Ensure the indicator is visible on mobile breakpoints too.

---

### Issue 3: Wallet Network Mismatch Guard (Freighter)
**Labels:** Frontend, wallet, testnet, security
**Complexity Score: 150**

#### Description
Prevent users from attempting actions when the wallet is not on the configured Testnet network.

#### Requirements
- Detect wallet network mismatch.
- Block on-chain actions with clear remediation steps.
- Add tests for mismatch states and recoveries.

---

### Issue 4: Transaction Progress UI for Testnet Submissions
**Labels:** Frontend, Soroban, testnet, ux
**Complexity Score: 150**

#### Description
Provide consistent progress and failure UX when submitting on-chain transactions during Testnet.

#### Requirements
- Show pending/succeeded/failed states with tx hash and explorer link.
- Persist recent transaction activity across refresh.
- Handle common failures (timeout, rejected signature, RPC failure) with actionable messaging.

---

### Issue 5: Testnet Faucet Helper UX
**Labels:** Frontend, testnet, ux, good first issue
**Complexity Score: 100**

#### Description
Make it easy for reviewers and recipients to obtain testnet funds during demos.

#### Requirements
- Add a small “Get Testnet XLM” helper panel with links to official faucet tools.
- Gate it so it only shows in testnet environments.
- Ensure it is accessible and non-intrusive.

---

### Issue 6: Standardize Explorer Links (Contract + Tx) in UI
**Labels:** Frontend, testnet, ux
**Complexity Score: 100**

#### Description
Ensure all explorer links in the UI are consistent and correct for Testnet.

#### Requirements
- Add a single helper for building explorer URLs.
- Use it for contract links, transaction links, and address links.
- Add tests for correct formatting.

---

### Issue 7: Runtime Config Validation and Friendly Error Page
**Labels:** Frontend, testnet, reliability
**Complexity Score: 150**

#### Description
Fail fast when environment variables are missing or misconfigured, especially on Vercel.

#### Requirements
- Validate required env vars at startup (backend URL, network, contract ID).
- Show a friendly “misconfigured deployment” page for missing config.
- Avoid exposing secrets in the UI.

---

### Issue 8: Testnet Demo Route: Guided “Happy Path” Checklist
**Labels:** Frontend, testnet, devex, help wanted
**Complexity Score: 150**

#### Description
Add a small guided checklist route for reviewers to run through the Testnet demo flow reliably.

#### Requirements
- Create a route that lists the exact steps to verify the demo (connect wallet, view campaign, submit claim, verify receipt).
- Include links to relevant pages and show current system health status.
- Keep the route hidden behind a simple feature flag or dev-only setting.
