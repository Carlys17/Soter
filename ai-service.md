# Wave 5: AI Service Issues (Testnet/Staging) for Soter

Wave 5 focuses on stabilizing the AI service for integration testing in a **Testnet-aligned environment** (staging behavior, secure callbacks, and predictable performance).

Complexity score scale:
- **100**: Beginner - Small endpoint, test, or config improvements.
- **150**: Intermediate - Cross-module flows and reliability enhancements.
- **200**: Advanced - Security, privacy guarantees, or deployment architecture.

---

### Issue 1: Testnet/Staging Environment Profile for AI Service
**Labels:** ai-service, python, testnet, devops, help wanted
**Complexity Score: 150**

#### Description
Add a dedicated environment profile for “testnet/staging” so the AI service behaves consistently during end-to-end testing.

#### Requirements
- Add explicit `APP_ENV=staging` behavior for safe defaults (rate limits, logging level, providers).
- Ensure config validation fails fast if required env vars are missing.
- Add tests verifying environment-specific config is applied.

---

### Issue 2: Secure HMAC Signature Validation for Backend Webhooks
**Labels:** ai-service, python, security, testnet, help wanted
**Complexity Score: 150**

#### Description
Validate inbound webhook/callback requests (if any) and sign outbound callbacks to the backend to prevent spoofing.

#### Requirements
- Add HMAC signing for outbound callbacks to backend.
- Validate signatures on inbound endpoints used for callbacks/commands (if present).
- Add replay protection using timestamps and nonce/ID where applicable.

---

### Issue 3: Outbound Callback Delivery Retries with Backoff
**Labels:** ai-service, python, reliability, testnet
**Complexity Score: 150**

#### Description
When the backend is temporarily unavailable, callbacks should retry safely without losing results.

#### Requirements
- Implement retry with exponential backoff for callback deliveries.
- Persist delivery attempts with status and last error.
- Add tests for retry, max-attempt exhaustion, and recovery.

---

### Issue 4: Artifact Upload Integrity Checks (Hash + Size)
**Labels:** ai-service, python, security, data-processing
**Complexity Score: 150**

#### Description
Ensure uploaded artifacts are not corrupted and are within policy constraints before processing.

#### Requirements
- Enforce max sizes and allowed MIME types for multipart uploads.
- Compute and store content hash for artifacts (used for dedup/integrity).
- Add tests for invalid MIME, oversize, and hash mismatch cases.

---

### Issue 5: Deterministic “Test Provider” for Staging/Testnet
**Labels:** ai-service, python, testnet, testing
**Complexity Score: 150**

#### Description
Provide deterministic AI outputs for predictable integration testing.

#### Requirements
- Add a “test provider” mode that returns stable, fixture-driven results for key endpoints.
- Ensure it can be enabled in CI and staging without code changes.
- Add tests proving output stability across runs.

---

### Issue 6: Rate Limits per Endpoint (Cost-Aware)
**Labels:** ai-service, python, testnet, reliability
**Complexity Score: 150**

#### Description
Apply stricter limits on high-cost endpoints (OCR, verification) while keeping health/docs unthrottled.

#### Requirements
- Configure route-specific limits.
- Ensure limits are compatible with multi-instance deployments (avoid purely in-memory state).
- Add tests for limit exhaustion and reset behavior.

---

### Issue 7: Structured Logging with Guaranteed Redaction
**Labels:** ai-service, python, security, observability
**Complexity Score: 200**

#### Description
Move to structured logs while preventing PII from ever being logged.

#### Requirements
- Emit JSON logs with request ID, route, latency, outcome, and provider metadata (non-sensitive).
- Apply redaction to request/response payload logging paths.
- Add tests that assert PII-like strings never appear in captured logs.

---

### Issue 8: Add `/health/dependencies` Probe
**Labels:** ai-service, python, observability, testnet
**Complexity Score: 100**

#### Description
Expose a lightweight dependency probe endpoint for staging and CI.

#### Requirements
- Include checks for Redis connectivity, provider configuration readiness, and filesystem/temp access.
- Keep response shape stable and minimal.
- Ensure the endpoint is safe (no secrets, no PII).

---

### Issue 9: Timeout Budgeting and Circuit Breakers for Providers
**Labels:** ai-service, python, reliability, testnet
**Complexity Score: 200**

#### Description
Prevent long-running provider calls from causing request pileups during demos.

#### Requirements
- Enforce request-level timeouts per provider call.
- Add circuit breaker behavior on repeated failures with automatic recovery.
- Add tests for timeouts and breaker open/close transitions.

---

### Issue 10: OCR Accuracy Regression Harness (Golden Inputs)
**Labels:** ai-service, python, data-processing, testing
**Complexity Score: 200**

#### Description
Prevent OCR regressions by testing against a small golden dataset.

#### Requirements
- Define golden inputs and expected extracted fields/bounds.
- Produce a summary report with pass/fail and error categories.
- Add CI job that runs the harness on relevant changes.

---

### Issue 11: PII Scrubbing Regression Harness (Golden Inputs)
**Labels:** ai-service, python, security, testing
**Complexity Score: 150**

#### Description
Lock in privacy expectations using a golden dataset for scrubbing behavior.

#### Requirements
- Add curated fixtures for emails, phones, IDs, addresses, names, and edge cases.
- Assert correct redaction and avoid false positives where safe text is preserved.
- Provide a summary diff output when regressions occur.

---

### Issue 12: Signed Artifact Access Tokens (Short-Lived)
**Labels:** ai-service, python, security, testnet
**Complexity Score: 150**

#### Description
Avoid permanent public URLs for sensitive artifacts by issuing short-lived access tokens.

#### Requirements
- Create token-based access for artifact downloads with expiry.
- Enforce role/ownership checks by integrating with backend authorization signals.
- Log access attempts without leaking sensitive content.

---

### Issue 13: Docker Image Publish for Staging (GitHub Actions)
**Labels:** ai-service, devops, ci, testnet, help wanted
**Complexity Score: 150**

#### Description
Publish a container image to be used by staging/testnet deployments.

#### Requirements
- Build a versioned image on tagged releases or main branch merges.
- Run tests before publishing.
- Ensure secrets are never printed and images are reproducible.

---

### Issue 14: End-to-End Contract-Aware Verification Metadata
**Labels:** ai-service, Backend, Soroban, testnet, integration
**Complexity Score: 150**

#### Description
Ensure AI verification results include metadata that the backend can anchor to on-chain events during Testnet demos.

#### Requirements
- Include stable identifiers (campaign reference, claim ID, package ID) in result payloads where applicable.
- Validate these identifiers and reject malformed inputs early.
- Add tests that verify payload shape and metadata propagation.
