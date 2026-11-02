# Molt

A native Rust animation library for Horizon, targeting the behavior and resource formats of GeckoLib 5.5.7.

## Project status

Part 1 (native base) is built and checked: the Molt bundle applies to the pinned Horizon, compiles, and produces an executable whose smoke hook logs a block-break call without changing gameplay. Running that executable in a game, and seeing the `[molt]` line, is the one check still done by hand (see [PROGRESSO.md](PROGRESSO.md)). The animation, geometry, controller, and rendering runtime has not been implemented yet.

- [Port plan](PLANO.md)
- [Current progress and handoff notes](PROGRESSO.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

## Build the native smoke example

This project follows the official Horizon native mod template and currently targets Linux with Rust 1.95.0 and Python 3.12+. Windows is not validated; use WSL/Linux for the staging script, which uses `fcntl`.

From a Linux/WSL shell, clone the dependencies beside this repository and check out the exact revisions recorded in `compatibility.json`:

```sh
git clone https://github.com/horizon-rust/horizon ../horizon
git -C ../horizon checkout ba6344a59cb9e755b730dcf02285fc0d33b97d3c
git clone https://github.com/horizon-rust/horizon-mixins ../horizon-mixins
git -C ../horizon-mixins checkout d9c83d7096b7ea5b83678da2efdfd6161f74997e
cargo fetch --locked --manifest-path ../horizon-mixins/Cargo.toml
python3 tools/horizon.py stage --horizon ../horizon
python3 tools/horizon.py fetch --horizon ../horizon   # once, with network: check/build/test run --offline
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test --horizon ../horizon    # the runtime's unit test inside Horizon's test binary
python3 tools/horizon.py build --horizon ../horizon
```

The `build` takes about 16 minutes on a fresh target directory (`check` about 3, `test` about 5). The Horizon checkout must be able to resolve the pinned commit (`git fetch origin <revision>` is enough).

Molt's own checks (they do not need Horizon): `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`, `cargo test --workspace --locked --offline`, `python3 tools/test_template.py`. With a Horizon checkout also `python3 tools/test_stage.py --horizon ../horizon`.

The script stages an exported copy of the pinned Horizon revision and applies the Molt mixin bundle with `horizon-mixins`; it does not edit the original Horizon checkout. The `build` action outputs a Horizon executable with the Molt smoke hook. Breaking a block should print a `[molt]` line to stderr. This scaffold does not yet render animated models.

See [PROGRESSO.md](PROGRESSO.md) for what was verified, with which toolchain, and what is still pending.
