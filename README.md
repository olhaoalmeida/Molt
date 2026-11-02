# Molt

A native Rust animation library for Horizon, targeting the behavior and resource formats of GeckoLib 5.5.7.

## Project status

The initial native mixin scaffold is in place. The smoke hook logs a block-break call without changing gameplay. The animation, geometry, controller, and rendering runtime has not been implemented yet.

- [Port plan](PLANO.md)
- [Current progress and handoff notes](PROGRESSO.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

## Build the native smoke example

This project follows the official Horizon native mod template and currently targets Linux with Rust 1.95.0 and Python 3.12+. Windows is not validated; use WSL/Linux for the staging script, which uses `fcntl`.

Clone the exact source revisions from `compatibility.json` beside this repository, then ensure the Horizon source contains the pinned commit. Fetch the mixin CLI dependencies once:

```sh
cargo fetch --locked --manifest-path ../horizon-mixins/Cargo.toml
python3 tools/horizon.py stage --horizon ../horizon
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py build --horizon ../horizon
```

The script stages an exported copy of the pinned Horizon revision and applies the Molt mixin bundle with `horizon-mixins`; it does not edit the original Horizon checkout. The `build` action outputs a Horizon executable with the Molt smoke hook. Breaking a block should print a `[molt]` line to stderr. This scaffold does not yet render animated models.

The tested build status is not yet recorded. See [PROGRESSO.md](PROGRESSO.md) before relying on the current pins.
