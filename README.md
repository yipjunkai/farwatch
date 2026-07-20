# farwatch

[![Release](https://img.shields.io/github/v/release/yipjunkai/farwatch)](https://github.com/yipjunkai/farwatch/releases/latest)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/yipjunkai/farwatch/badge)](https://scorecard.dev/viewer/?uri=github.com/yipjunkai/farwatch)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#-license)

**Access your AI coding assistant from any device.** End-to-end encrypted terminal mirroring with structured agent events — for Claude Code, Aider, Copilot, Gemini, and any terminal-based AI tool. Rust core. Zero-knowledge relay. Self-hostable.

```bash
farwatch start claude   # launches Claude Code in a PTY, prints a QR code
```

Scan the QR from your phone and you get a live, encrypted session — both a raw terminal mirror and a structured view with native UI for tool calls, thinking indicators, and prompts. The relay in the middle only ever sees ciphertext.

> **ℹ️ Status** — This is a personal project, kept public because the crypto is meant to be auditable. It is not actively developed as a product, so issues and PRs may not get timely responses. **The hosted relay is currently offline** (it may return) — [self-host a relay](#-self-hosting) to run farwatch today; that path needs no account. Backstory: [Why farwatch exists](#-why-farwatch-exists).

## 🧭 How it works

```
Your Machine                         Cloud                          Your Phone
┌──────────────────┐           ┌──────────────┐           ┌──────────────────┐
│ Claude Code      │           │ Relay Server │           │ Structured View  │
│ (real TUI)       │◄── PTY ──│ (zero-       │── E2E ───│ (native cards)   │
│                  │           │  knowledge)  │ encrypted │                  │
│ Desktop: Enter   │           │              │           │ Terminal View    │
│ to take over     │           │              │           │ (raw PTY mirror) │
└──────────────────┘           └──────────────┘           └──────────────────┘
```

**Desktop** gets the real Claude Code TUI via takeover mode. **Phone** gets structured events (thinking, tool calls, text) as native UI cards, plus a terminal view toggle. Both can send input simultaneously. The relay is a dumb pipe — it forwards opaque encrypted bytes and never sees your data.

## 📦 Install

```bash
# Homebrew (macOS / Linux)
brew install yipjunkai/farwatch/farwatch

# Shell script
curl -fsSL https://raw.githubusercontent.com/yipjunkai/farwatch/main/install.sh | sh

# From source
cargo install --git https://github.com/yipjunkai/farwatch -p cli

# Docker (relay server only)
docker pull ghcr.io/yipjunkai/farwatch:latest
```

## 🚀 Quick start

> The default `farwatch start` connects to the hosted relay, which is **currently offline**. [Self-host a relay](#-self-hosting) (no account needed) and point the CLI at it with `FARWATCH_URL`.

```bash
# Start a session (auto-detects your AI tool)
farwatch start

# Or specify a tool with extra args
farwatch start claude
farwatch start aider --model sonnet
```

Scan the QR code from your phone to connect. On the host:

- **Enter** — take over the terminal (you type directly into Claude Code)
- **Double-tap Esc** — return to the dashboard
- **q** — quit the session

### Manual attach (from another terminal)

```bash
farwatch attach --pairing-uri "farwatch://pair?..."
```

## 📱 What your phone sees

For tools that support structured output (currently Claude Code), the phone shows two views.

**Structured view** (default for Claude Code):

- Thinking blocks (purple cards with reasoning)
- Tool call cards (tool name, arguments)
- Tool results (output, truncated to 32KB)
- Text responses
- Turn markers and busy indicator
- Prompt bar with mic/send button (Telegram-style)

**Terminal view** (toggle via AppBar button):

- Raw PTY output mirror — exactly what the desktop sees
- Bottom sheet with terminal actions (Ctrl+C, arrows, paste, etc.)

Both views update in real-time as Claude Code works. The structured events come from tailing Claude Code's `.jsonl` session log (`~/.claude/projects/`), not from parsing terminal output.

The mobile client lives in a separate repo: [**farwatch-mobile**](https://github.com/yipjunkai/farwatch-mobile) (Flutter, iOS + Android).

## 🤖 Supported AI tools

Auto-detected in order of priority:

| Tool                               | Structured events | Notes                                          |
| ---------------------------------- | :---------------: | ---------------------------------------------- |
| **Claude Code** (Anthropic)        |        Yes        | JSONL session log tailing for native mobile UI |
| **OpenCode** (open source)         |     PTY only      |                                                |
| **GitHub Copilot CLI** (Microsoft) |     PTY only      |                                                |
| **Gemini CLI** (Google)            |     PTY only      |                                                |
| **Aider** (open source)            |     PTY only      |                                                |
| Any command on PATH                |     PTY only      | `farwatch start my-tool`                       |

Tools without structured support work via PTY mirroring — the phone shows a terminal emulator.

## 🏗️ Architecture

### Dual-channel design

For Claude Code, the host sends two parallel streams through the same encrypted channel:

```
Claude Code (PTY)
├── Raw PTY bytes ──→ SecureMessage::PtyOutput ──→ Phone terminal view
│
└── ~/.claude/projects/<hash>/<session>.jsonl
    └── JSONL watcher (notify/kqueue) ──→ SecureMessage::AgentEvent ──→ Phone structured view
```

The JSONL watcher tails Claude Code's session log file using filesystem notifications. It parses assistant messages, tool calls, tool results, thinking blocks, and turn completion (`stop_reason: "end_turn"`). These are emitted as `AgentEvent` variants through the same E2E encrypted channel.

Phone input flows back via `AgentCommand::Prompt` which gets injected into the PTY as keystrokes (`text + \r`). Tool approvals send `y\r`, denials send `n\r`.

### Takeover mode

Press Enter on the host dashboard to take over the PTY directly:

1. TUI dashboard suspends, terminal switches to raw mode
2. PTY output displays on your screen (Ctrl+L redraw on entry)
3. Your keyboard input goes directly to the PTY
4. Terminal resizes are forwarded to the PTY
5. PTY output simultaneously flows to the phone via relay
6. Double-tap Esc returns to the dashboard

Both desktop and phone can send input at any time. There's no locking — the PTY processes input from both sources.

### Protocol layers

All WebSocket messages are MessagePack-encoded across three layers:

**Relay-level** (`RelayMessage`): Register, Registered, Route, PeerStatus, Ping/Pong, Error

**E2E-level** (`PeerFrame` inside Route payload): Handshake, HandshakeConfirm, Secure (AES-GCM sealed), KeepAlive

**Application-level** (`SecureMessage` inside Secure):

- Terminal: `PtyInput`, `PtyOutput`, `Resize`
- Agent: `AgentEvent`, `AgentCommand`
- Session: `Heartbeat`, `VersionNotice`, `Notification`, `SessionEnded`, `ReadOnly`
- Voice: `VoiceCommand`

## 📁 Project structure

```text
farwatch/
├── crates/
│   ├── cli/         # User-facing binary (farwatch): PTY, JSONL watcher, TUI, takeover
│   ├── relay/       # Zero-knowledge relay server
│   └── protocol/    # Shared types, crypto (X25519 + AES-256-GCM), pairing primitives
├── Dockerfile       # Relay image for self-hosters
├── install.sh       # Shell installer (downloads a release binary)
└── .github/         # Release + relay-image publish workflows
```

| Crate      | Description                                                                        |
| ---------- | --------------------------------------------------------------------------------- |
| `cli`      | User-facing binary (`farwatch`), PTY management, JSONL watcher, TUI, takeover mode |
| `relay`    | Zero-knowledge relay server                                                       |
| `protocol` | Shared protocol types, crypto (X25519 + AES-256-GCM), pairing primitives          |

The Flutter mobile app (iOS + Android) is the primary mobile client — see [farwatch-mobile](https://github.com/yipjunkai/farwatch-mobile).

## 🔒 Security

Farwatch is designed so that **no one except you and your connected device can read your terminal data** — not us, not the relay operator, not anyone on the network.

### Threat model

The relay server is assumed to be **honest-but-curious**: it faithfully forwards messages but may attempt to read them. All terminal data is encrypted end-to-end before it reaches the relay, so a compromised or malicious relay learns nothing beyond metadata (session ID, peer role, message timing and size).

### End-to-end encryption

Every session establishes a unique encrypted channel between the host (your dev machine) and the client (your phone/tablet/other terminal):

1. **Key exchange**: Each side generates an ephemeral X25519 key pair. Public keys are exchanged via the relay inside `Handshake` messages.
2. **Key derivation**: Both sides compute a shared secret via Diffie-Hellman, then derive two 256-bit symmetric keys (one per direction) using HKDF-SHA256 with the session ID as salt and `farwatch/v1/channel-keys` as the info string.
3. **Encryption**: All terminal I/O and agent events are sealed with AES-256-GCM before transmission. Each frame carries a monotonically increasing nonce.
4. **Key confirmation**: After key derivation, both sides exchange an HMAC-SHA256 over the handshake transcript, proving each peer holds the private key corresponding to their advertised public key.

### Replay and reorder protection

Every encrypted frame includes a strictly monotonic 64-bit nonce. The receiver rejects any frame with a nonce less than or equal to the last accepted nonce.

### Identity verification

The pairing URI (displayed as a QR code) includes a SHA-256 fingerprint of the host's public key. The client verifies this fingerprint on connection, detecting any man-in-the-middle substitution by the relay.

### Zero-knowledge relay

The relay server sees only session ID, peer role, message size, and timing. It **never** sees plaintext terminal content, keystrokes, agent events, public keys, or encryption keys. The relay cannot decrypt, modify, or forge messages — any tampering is detected by AES-GCM authentication.

### Cryptographic primitives

| Purpose              | Algorithm                        | Notes                                            |
| -------------------- | -------------------------------- | ------------------------------------------------ |
| Key exchange         | X25519                           | Ephemeral per-session key pairs                  |
| Key derivation       | HKDF-SHA256                      | Session ID as salt, domain-separated info string |
| Authenticated cipher | AES-256-GCM                      | Per-frame encryption with monotonic nonce        |
| Key confirmation     | HMAC-SHA256                      | MAC over handshake transcript                    |
| Fingerprint          | SHA-256 (truncated)              | First 8 bytes of public key hash                 |
| At-rest encryption   | AES-256-GCM                      | Random nonce per file, machine-local key         |
| Nonce construction   | 4 zero bytes + 8-byte BE counter | 96-bit nonce from 64-bit counter                 |

### What farwatch does NOT protect against

- **Compromised endpoints**: If your machine or phone is compromised, the attacker has access to the decrypted session.
- **Traffic analysis**: The relay can see message timing and sizes, revealing activity patterns.
- **Denial of service**: A malicious relay can drop or delay messages (but cannot read or forge content).

## 🖥️ Self-hosting

Run your own relay server — no account, API key, or control API needed:

```bash
# From source
cargo run -p relay -- --bind 0.0.0.0:8080

# With Docker
docker run -p 8080:8080 ghcr.io/yipjunkai/farwatch:latest
```

Point the CLI at your relay:

```bash
FARWATCH_URL=ws://your-server:8080/ws farwatch start
```

> **Note:** `farwatch auth` is for the hosted service only. Self-hosted relays run unauthenticated by default.

## 🛠️ Development

```bash
# Run the relay server locally
cargo run -p relay -- --bind 0.0.0.0:8080

# Run the CLI against local relay
FARWATCH_URL=ws://127.0.0.1:8080/ws cargo run -p cli -- start

# Run with hosted service features (auth, device flow)
FARWATCH_URL=ws://127.0.0.1:8080/ws cargo run -p cli --features hosted -- start

# Run tests
cargo test
```

### Feature flags

| Build command                          | Includes auth? | Use case                   |
| -------------------------------------- | -------------- | -------------------------- |
| `cargo build -p cli`                   | No             | Self-hosted / contributors |
| `cargo build -p cli --features hosted` | Yes            | Hosted service users       |

The AI tool launches in your current working directory, not the repo directory. To run the dev CLI from elsewhere, pass `--manifest-path /path/to/farwatch/Cargo.toml`.

## 🤔 Why farwatch exists

I wanted to check on and steer Claude Code from my phone without handing my terminal, prompts, or code to a third-party server. So farwatch encrypts everything end-to-end and treats the relay as a dumb pipe that only sees ciphertext — the crypto is open (the `protocol` crate) specifically so that claim is auditable.

Since it was built, first-party remote control shipped in Claude Code, Codex, and Copilot CLI, so farwatch is no longer the only way to do this. It stays useful where those don't reach: **any** terminal tool rather than a single vendor, fully self-hostable, and end-to-end encrypted with nothing stored on anyone's server. These days I maintain it as a personal tool rather than a product.

## 🤝 Contributing

This is a personal project without active maintenance commitments — issues and PRs are welcome but may not get a timely response. If you want to build on it, forking is encouraged (the license allows it).

## 📄 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in **farwatch** by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any additional terms or conditions.
