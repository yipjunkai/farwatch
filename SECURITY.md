# Security Policy

## Supported versions

farwatch is solo-maintained: only the **latest released minor version** receives
security fixes. Older minors are not patched — upgrade to the most recent
[release][rel] before reporting.

[rel]: https://github.com/yipjunkai/farwatch/releases/latest

## Reporting a vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Use GitHub's private security advisory mechanism:

1. Go to <https://github.com/yipjunkai/farwatch/security/advisories>
2. Click "Report a vulnerability"
3. Fill in the form with as much detail as you can share

Acknowledgement is targeted within 72 hours. There is no separate email contact at this stage; the GitHub advisory channel is the only supported route.

## What to report

- **Cryptographic or protocol weaknesses** — flaws in the X25519 handshake, HKDF-SHA256 key derivation, AES-256-GCM sealing, the monotonic-nonce replay/reorder protection, or the pairing-fingerprint MITM check that would let the relay or a network attacker read, forge, or replay session traffic
- **Zero-knowledge relay violations** — any way for the relay to recover plaintext, keys, or more than the documented metadata (session id, peer role, message size/timing), or to tamper with traffic without detection
- **Memory-safety or denial-of-service in the Rust code** — the workspace sets `unsafe_code = "forbid"`, so the likeliest surface is crafted wire input triggering a panic-as-crash, infinite loop, or unbounded allocation in the relay or the protocol decoders
- **Supply-chain concerns** — dependency vulnerabilities not yet flagged by Dependabot or `cargo audit`

## What is not in scope

- **Compromised endpoints** — a malicious or infected host/client machine already holds the decrypted session (see the threat model in the [README](README.md#-security))
- **Traffic analysis** — message timing and sizes are visible to the relay by design and are documented as known-exposed metadata
- **Relay denial-of-service** — a malicious relay dropping or delaying messages is a documented non-goal (it still cannot read or forge content)
- Missing tool integrations, feature requests, or general bugs (open a regular issue)
