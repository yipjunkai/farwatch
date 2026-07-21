# Fuzzing

Coverage-guided fuzzing of the `protocol` crate's byte-facing surface — every
boundary where farwatch turns attacker-controlled input into a typed value.
The relay decodes frames from any client that connects, peers exchange frames
before authentication completes, and the pairing URI comes straight off a
scanned QR code. Parsing hostile input *is* the job here, so this is a primary
correctness lane, not an edge-case afterthought.

This is a standalone Cargo workspace (excluded from the root one via
`exclude = ["fuzz"]`) so its nightly + `libfuzzer-sys` toolchain never touches
normal `cargo check`/`test`/`build`.

## Targets

| Target | Entry point | Invariant (beyond "no panic") |
|---|---|---|
| `decode_relay` | `protocol::decode_relay` | decode/encode are exact inverses — any accepted `RelayMessage` re-encodes and decodes back equal |
| `decode_peer_frame` | `protocol::decode_peer_frame` | any accepted `PeerFrame` re-encodes and decodes back equal |
| `decode_secure_message` | `protocol::decode_secure_message` | infallible; every recognized (non-`Unknown`) variant re-encodes and decodes back equal |
| `parse_pairing_uri` | `protocol::pairing::parse_pairing_uri` | parse/build agree — any accepted URI rebuilds and re-parses to identical fields (percent-encoding round-trip) |

## Running locally

```sh
# from the repo root, one target for 60s (corpus/ first = writable working set,
# seeds/ = read-only inputs; never pass seeds/ first or libFuzzer writes evolved
# inputs into it):
cd fuzz && mkdir -p corpus/decode_relay && \
    cargo fuzz run decode_relay corpus/decode_relay seeds/decode_relay -- -max_total_time=60
```

`cargo fuzz` installs the pinned nightly automatically from `rust-toolchain.toml`.

## Corpus policy

Two layers, deliberately separated:

- **`seeds/<target>/`** (committed, code-reviewed): benign, valid inputs that
  bootstrap coverage. The wire-decoder seeds are canonical encodings emitted by
  the crate's own `generate_test_vectors` test, so they decode cleanly and give
  the fuzzer a warm start into the type graph; the `parse_pairing_uri` seeds are
  real `farwatch://pair?...` URIs.
- **`corpus/<target>/`** (gitignored): the evolving working set libFuzzer grows.
  In CI it is cached and compounds across nightly runs; locally it is yours.

## When a crash is found

The reproducing input is written under `artifacts/<target>/`. Reproduce with
`cargo fuzz run <target> artifacts/<target>/<crash-file>`, minimize with
`cargo fuzz cmin`, fix the bug, and add the minimized reproducer to
`seeds/<target>/` so the case is regression-tested forever.
