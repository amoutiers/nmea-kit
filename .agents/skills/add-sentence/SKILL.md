---
name: add-sentence
description: Wire pre-drafted NMEA sentence types from drafts/ into nmea-kit, including validation and release preparation. Use when asked to add, wire, or ship sentence types such as APB, BWC, or WPL.
---

# Add Sentence

Wire pre-drafted sentence files into the crate. Keep the change limited to the requested sentence types.

## Source and selection

Unwired implementations live in `drafts/sentences/<tla>.rs`; their tests live in `drafts/tests/<tla>.rs`. `drafts/` is gitignored.

- Use the sentence types supplied by the user. Otherwise, propose up to two candidates from `drafts/sentences/` and wait for confirmation.
- If `drafts/` or the requested files are absent, stop and report it.

## Wire a sentence

Move each selected implementation to `src/nmea/sentences/` and its tests to `tests/nmea/`. Wire the type alphabetically in each of these locations:

1. `Cargo.toml`: add `<tla> = []` and add `"<tla>"` to the `nmea` feature bundle.
2. `src/nmea/sentences/mod.rs`: add the feature-gated module and public re-export.
3. `src/nmea/mod.rs`: register it in the appropriate `nmea_sentences!` arm.
4. `src/lib.rs`: add it to the single `nmea_item!` feature list.
5. `tests/nmea.rs`: register the integration-test module.

## Compatibility checks

- Standard types use their three-letter `SENTENCE_TYPE`.
- Proprietary types use the full address, set `PROPRIETARY: bool = true`, and encode with `to_sentence("")`.
- Coordinates use `Option<f64>`, `r.f64()`, and `w.lat()` or `w.lon()`.
- Hex or enum fields that can contain letters use `Option<char>`, `r.char()`, and `w.char()`.
- Keep `parse()` lenient: return `Some(...)`; do not add `unwrap`, `panic`, `unreachable`, `todo`, or `#[allow(dead_code)]` under `src/`.
- Recompute every NMEA fixture checksum before relying on it.

## Tests and review

Add or complete `<tla>_values`: parse an authoritative fixture and assert every decoded field. Compare floats with a tolerance. For types with fixed indicators, also pin the canonical encoded body after stripping its checksum.

Review that parsing and encoding fields have the same order, fixed indicators use `r.skip()` and `w.fixed()`, new NMEA 4.x trailing fields both parse and encode, and each sentence has empty, round-trip, and values coverage.

Run:

```sh
cargo fmt
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo build --no-default-features --features <tla>
```

All commands must finish without warnings. On a failure, fix the root cause; after two failed attempts on the same issue, stop and report the decisive error.

## Release preparation

When the requested scope includes a release, read the version from `Cargo.toml`, update `CHANGELOG.md` following `RELEASING.md`, and update the sentence/version/test counts in `AGENTS.md`, `SENTENCES.md`, `README.md`, and the crate docs. The test count is the sum of all `test result:` lines, including doctests.

Make a local commit only after the coherent change is verified. Never tag, push, create a GitHub release, or publish to crates.io without the user's explicit authorization in the current task.
