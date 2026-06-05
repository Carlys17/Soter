# Wave 5: Smart Contract Issues (Testnet) for Soter

Wave 5 focuses on deploying and validating Soroban contracts on **Stellar Testnet** and making contract interactions production-ready for integration testing.

Complexity score scale:
- **100**: Beginner - Small tests, scripts, or parameter validations.
- **150**: Intermediate - New flow, indexing, or integration hardening.
- **200**: Advanced - Security-sensitive changes, deployment strategy, or performance work.

---

### Issue 1: Deploy `AidEscrow` Contract to Stellar Testnet
**Labels:** Soroban, Contract, testnet, deployment, help wanted
**Complexity Score: 200**

#### Description
Deploy the `AidEscrow` Soroban contract to Stellar **Testnet** and record all artifacts needed to reproduce and verify the deployment.

#### Requirements
- Build and optimize WASM from `app/onchain/contracts/aid_escrow`.
- Deploy to **Testnet only** and produce a deployment record (contract ID, wasm hash, deployer).
- Initialize contract with intended admin/config values.
- Verify core flows on testnet: initialize, create package, claim, revoke/cancel (if enabled), view reads.

---

### Issue 2: Publish Testnet Contract Registry (IDs + Config Snapshot)
**Labels:** Soroban, Contract, testnet, devex, documentation
**Complexity Score: 100**

#### Description
Create a single source of truth for Testnet contract addresses and init configuration used by the app.

#### Requirements
- Record contract ID(s), deployer address, wasm hash, and init args.
- Include a timestamp and git commit SHA for the deployed build.
- Ensure registry format is machine-readable (JSON or env-style mapping) and easy for contributors to update.

---

### Issue 3: Add Testnet Invoke Scripts for Common Actions
**Labels:** Soroban, Contract, testnet, devex, good first issue
**Complexity Score: 150**

#### Description
Provide repeatable scripts to invoke key contract functions on Testnet for debugging and demos.

#### Requirements
- Add scripts for initialize, create package, claim, and view queries.
- Make scripts parameterized (contract ID, recipient, token, amount, expiry).
- Ensure scripts print tx hash and the invoked arguments clearly.

---

### Issue 4: Testnet Smoke Suite Against Real Network
**Labels:** Soroban, Contract, testnet, testing, ci
**Complexity Score: 200**

#### Description
Add a smoke suite that runs against a real Testnet contract to ensure deployments remain valid.

#### Requirements
- Implement a minimal test run that executes read-only calls and at least one safe state transition.
- Gate the suite behind an opt-in CI secret and run it only on protected branches or manual dispatch.
- Ensure retries and network timeouts are handled deterministically.

---

### Issue 5: Testnet Redeploy & Versioning Strategy
**Labels:** Soroban, Contract, testnet, versioning, ops
**Complexity Score: 150**

#### Description
Define and implement a safe approach for redeploying and migrating contract state during Testnet iteration.

#### Requirements
- Document how to handle breaking changes vs non-breaking changes on testnet.
- Ensure contract version is queryable on-chain and logged in deployment artifacts.
- Add a checklist to prevent “orphaning” integrations when contract IDs change.

---

### Issue 6: Event Schema Audit for Indexers (Testnet)
**Labels:** Soroban, Contract, testnet, observability, data-processing
**Complexity Score: 150**

#### Description
Finalize event topics/payload shapes so backend indexers can rely on them during Testnet.

#### Requirements
- Enumerate emitted events and ensure they include stable identifiers (package ID, campaign reference if used).
- Validate event payloads don’t leak sensitive metadata.
- Add snapshot tests covering event topics/payload consistency.

---

### Issue 7: Gas Profiling for High-Volume Distributions on Testnet
**Labels:** Soroban, Contract, testnet, performance
**Complexity Score: 150**

#### Description
Measure resource usage for realistic distribution operations to avoid surprising limits during demos.

#### Requirements
- Benchmark create + claim flows across multiple packages.
- Record recommended “safe batch sizes” or throughput guidance based on results.
- Identify top contributors to resource use and propose optimizations if needed.

---

### Issue 8: Token Address Validation and Standard Interface Compliance
**Labels:** Soroban, Contract, testnet, security
**Complexity Score: 150**

#### Description
Harden token interactions so invalid or unexpected token contracts cannot break accounting on testnet.

#### Requirements
- Validate token addresses and enforce expected interface behavior.
- Add tests for invalid token addresses and reverted token transfers.
- Ensure failures emit clear contract errors mapped cleanly by the backend.

---

### Issue 9: Admin Key Policy for Testnet Deployments
**Labels:** Soroban, Contract, testnet, security, ops
**Complexity Score: 100**

#### Description
Define which accounts control admin actions on testnet and how keys are stored/rotated for contributors.

#### Requirements
- Separate deployer key and operator/admin key(s) where feasible.
- Define a minimum policy for key storage during testnet (no keys in repo, clear env naming).
- Add a short checklist for rotating a compromised testnet key.

---

### Issue 10: Merkle Allowlist Testnet Validation and Edge Cases
**Labels:** Soroban, Contract, testnet, data-processing, testing
**Complexity Score: 200**

#### Description
Validate the Merkle allowlist claim flow on Testnet with real proofs and adversarial cases.

#### Requirements
- Provide a way to generate and verify proofs for a sample allowlist.
- Test invalid proof paths, wrong recipient, wrong leaf, and mismatched root cases.
- Confirm error outputs remain stable for backend mapping.

---

### Issue 11: Claim Window and Expiry Boundary Validation on Testnet
**Labels:** Soroban, Contract, testnet, testing
**Complexity Score: 150**

#### Description
Ensure timing-sensitive rules behave correctly on Testnet ledgers (claim start time, expiry boundaries).

#### Requirements
- Validate claim behavior at exact boundary times (start, expiry, late claim).
- Confirm package status auto-expiry logic is correct (if implemented).
- Record expected behavior for the backend and frontend to display correctly.

---

### Issue 12: Testnet Deployment “Go/No-Go” Checklist and Runbook
**Labels:** Soroban, Contract, testnet, ops, documentation
**Complexity Score: 100**

#### Description
Create a runbook so anyone can deploy the contract to testnet consistently and verify success.

#### Requirements
- Include build steps, deploy steps, initialization steps, and verification steps.
- Include troubleshooting for common Soroban RPC issues.
- Include a minimal “post-deploy health check” procedure and expected outputs.
