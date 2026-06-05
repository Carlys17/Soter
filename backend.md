# Wave 5: Backend Issues (Testnet) for Soter

Wave 5 focuses on **Testnet-ready integrations**: contract configuration, ledger indexing, observability, and deployment hygiene for the NestJS backend.

Complexity score scale:
- **100**: Beginner - Focused endpoint/DTO changes and safe refactors.
- **150**: Intermediate - Multi-module flows and integration work.
- **200**: Advanced - Cross-cutting reliability, data lifecycle, and indexer correctness.

---

### Issue 1: Testnet Contract Config + Network Guardrails
**Labels:** Backend, Soroban, testnet, integration, help wanted
**Complexity Score: 150**

#### Description
Wire the backend to the deployed **Testnet** `AidEscrow` contract ID and prevent cross-network mismatches.

#### Requirements
- Add env vars for `SOROBAN_NETWORK=testnet` and `AID_ESCROW_CONTRACT_ID`.
- Validate config at startup (missing/invalid contract ID should fail fast).
- Add clear errors when wallet/network does not match configured testnet environment.

---

### Issue 2: Persist Deployment Metadata in DB
**Labels:** Backend, testnet, deployment, data-processing
**Complexity Score: 150**

#### Description
Store contract deployment metadata so the backend can report the currently configured contract and provenance.

#### Requirements
- Add a table/model for deployment metadata (network, contract ID, wasm hash, deployed at, commit SHA).
- Add read-only API endpoint for internal/admin visibility.
- Add tests to ensure metadata is tenant-safe where relevant.

---

### Issue 3: Ledger Backfill Service for Testnet Contract Events
**Labels:** Backend, Soroban, testnet, data-processing, help wanted
**Complexity Score: 200**

#### Description
Backfill historical contract events from the deployed Testnet contract to populate analytics and timelines.

#### Requirements
- Implement a backfill job that scans from a configurable starting ledger/sequence.
- Persist progress checkpoints and support resume.
- Ensure idempotent writes and no duplicate events.

---

### Issue 4: Ledger Reconciliation and Drift Detection
**Labels:** Backend, Soroban, testnet, observability, reliability
**Complexity Score: 200**

#### Description
Detect and reconcile divergence between on-chain state and backend cached state during Testnet testing.

#### Requirements
- Implement periodic reconciliation of package statuses and locked totals.
- Record drift incidents with details and timestamps.
- Provide an admin endpoint to trigger reconciliation on-demand.

---

### Issue 5: Generate Stellar Explorer Links for Testnet (Contract + Tx)
**Labels:** Backend, testnet, integration, good first issue
**Complexity Score: 100**

#### Description
Standardize explorer URLs returned by the backend so clients can link to transactions and contract addresses on Testnet.

#### Requirements
- Provide utilities to generate correct explorer links for the configured network.
- Return explorer links in relevant API responses (where tx hash/contract ID is present).
- Add unit tests for formatting and network switching.

---

### Issue 6: On-Chain Health Probe Endpoint (Read-Only Contract Ping)
**Labels:** Backend, Soroban, testnet, observability
**Complexity Score: 150**

#### Description
Expose an internal health probe that confirms the backend can talk to Soroban RPC and read contract state.

#### Requirements
- Add a health endpoint that performs at least one read-only contract call.
- Return latency and a minimal status result without leaking secrets.
- Add tests and ensure the probe is protected (not public).

---

### Issue 7: End-to-End Integration Harness (Backend ↔ Contract on Testnet)
**Labels:** Backend, Soroban, testnet, testing, ci
**Complexity Score: 200**

#### Description
Add an opt-in harness to validate backend integration against the real Testnet contract.

#### Requirements
- Add a test mode that runs a minimal create/claim flow using testnet secrets.
- Run only via manual workflow dispatch or protected branches.
- Ensure retries/timeouts are deterministic and logs include correlation IDs.

---

### Issue 8: Idempotency Keys for On-Chain Mutations
**Labels:** Backend, Soroban, testnet, reliability
**Complexity Score: 150**

#### Description
Make on-chain mutations safe to retry so clients can handle network flakiness during Testnet.

#### Requirements
- Add idempotency-key support for endpoints that submit Soroban transactions.
- Persist request fingerprints and prevent double-submission.
- Add tests for duplicate requests and mismatched request bodies.

---

### Issue 9: Verification Inbox API for Clients (Testnet Demo)
**Labels:** Backend, testnet, api, help wanted
**Complexity Score: 150**

#### Description
Expose a stable API for clients to fetch verification statuses used during Testnet demos.

#### Requirements
- Provide list and detail endpoints for verification inbox items.
- Support filters for status and date.
- Ensure org/role enforcement is correct.

---

### Issue 10: Evidence Upload Sessions (Chunking) for Field Networks
**Labels:** Backend, testnet, reliability, data-processing
**Complexity Score: 200**

#### Description
Support chunked/resumable evidence uploads to reduce failures in poor connectivity environments during Testnet pilots.

#### Requirements
- Add upload session create/chunk/finalize endpoints.
- Enforce ownership, size limits, and content-type restrictions.
- Add tests for resume and duplicate chunk handling.

---

### Issue 11: Content Deduplication and Source Fingerprinting
**Labels:** Backend, data-processing, testnet, help wanted
**Complexity Score: 150**

#### Description
Prevent duplicate evidence and repeated submissions from re-triggering expensive processing and skewing analytics.

#### Requirements
- Generate stable fingerprints for uploaded evidence and key text payloads.
- De-duplicate repeated submissions within org scope while preserving auditability.
- Add tests for exact duplicates, near duplicates, and tenant isolation.

---

### Issue 12: Multilingual Translation and Normalization Pipeline
**Labels:** Backend, data-processing, testnet, help wanted
**Complexity Score: 200**

#### Description
Support intake across languages by translating and normalizing extracted text into a consistent analysis format.

#### Requirements
- Detect language and normalize extracted text into a canonical representation.
- Store translated and original text separately with provenance metadata.
- Add tests for deterministic normalization rules and error handling.

---

### Issue 13: Entity Linking to Projects, Assets, and Ecosystem Registry
**Labels:** Backend, data-processing, testnet, help wanted
**Complexity Score: 200**

#### Description
Link extracted entities (orgs, locations, assets, projects) to canonical registry records for better analytics and reporting.

#### Requirements
- Define a minimal registry model with stable IDs.
- Implement linking service with confidence scores and auditability.
- Add query APIs for linked entities by campaign/claim/verification.

---

### Issue 14: Webhook Delivery Receipts for AI Callbacks
**Labels:** Backend, ai-service, testnet, reliability
**Complexity Score: 150**

#### Description
Track AI callback deliveries so retries and “exactly-once” semantics are easier to reason about during Testnet.

#### Requirements
- Persist callback delivery attempts with status, timestamps, and response codes.
- Prevent stale callbacks from overwriting newer verification results.
- Add tests for duplicates and out-of-order deliveries.

---

### Issue 15: Strict CORS Policy for Vercel Preview + Testnet
**Labels:** Backend, Frontend, testnet, security
**Complexity Score: 100**

#### Description
Allow Vercel preview deployments to call the backend safely without widening CORS too much.

#### Requirements
- Support an allowlist-based CORS config for preview + production frontends.
- Keep sensitive/admin endpoints protected regardless of CORS.
- Add tests for expected allow/deny behavior.

---

### Issue 16: Testnet Demo Seed Endpoint (Sandbox-Only)
**Labels:** Backend, testnet, devex, good first issue
**Complexity Score: 100**

#### Description
Provide a safe way to generate demo data for reviewers while keeping it disabled outside sandbox contexts.

#### Requirements
- Add sandbox-only seed endpoints or scripts to create demo orgs/campaigns/claims.
- Ensure the feature is disabled by default and requires explicit enablement.
- Add tests verifying non-sandbox environments reject seed operations.

---

### Issue 17: Transaction Status Polling Endpoint for Clients
**Labels:** Backend, Soroban, testnet, api
**Complexity Score: 150**

#### Description
Expose a stable endpoint for frontend/mobile to poll transaction status and show progress during Testnet.

#### Requirements
- Provide tx status resolution from Soroban RPC by tx hash.
- Return normalized states: pending, succeeded, failed, unknown.
- Add tests for each status mapping and timeout behavior.

---

### Issue 18: Observability Dashboard for Testnet Operations
**Labels:** Backend, observability, testnet, help wanted
**Complexity Score: 150**

#### Description
Make Testnet reliability visible by exposing key metrics and failure modes.

#### Requirements
- Add metrics for contract call latency, tx submission failures, and callback failures.
- Ensure request correlation IDs propagate into logs and job processing.
- Add documentation of “what to look at” when Testnet issues occur.
