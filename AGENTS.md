# tally: rules for every agent and person

tally is a command line tool. The main evidence is an end to end test that runs the real binary.

## Commands

- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`
- `cargo test` runs everything that needs no host access
- `cargo test --test <file> <name>` runs one proof
- `cargo test -- --ignored` runs host proofs (they need a token or the host); agents hand these over

## Who owns which path

| Path | Owner | Others |
|---|---|---|
| `contracts/*.md`, `tests/e2e_*.rs`, `tests/common/`, `tests/golden/`, `tests/fixtures/` | Plan | read only |
| `tests/regress_*.rs` | Prove | read only |
| everything else (`src/`, `Cargo.toml`, `docs/`) | Build | |

A change to a protected path from the wrong owner is a violation. Ship checks it from a sealed commit.

## Outputs that change only on purpose

`tests/golden/*` holds the version output and report files other tools read. A card may change one
only when its contract has a claim naming that golden file.

## Proof names

- `tests/e2e_<slug>.rs`, function `<name>`; a contract names it as `e2e_<slug>::<name>`
- functions starting `perf_` are performance proofs; they run alone, never beside other work
- functions marked `#[ignore]` are host proofs; the contract's kind for them is `host`
- no proof may call a live model. A model is replaced by the fixture in `tests/fixtures/`
