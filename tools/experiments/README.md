# Experiments

Small, currently-relevant Python tooling that isn't part of the Rust runtime.

Contents:

- `config.py` - the scene-config model cited by `docs/config.md` and
  `docs/audit.md`; self-contained (stdlib only)
- `app.py` - a demo entrypoint for the legacy Python renderer; it inserts
  `tools/legacy-python/` onto `sys.path` at import time to reach
  `engine`/`runtime`/`ui`, which live there rather than here (see
  `tools/legacy-python/README.md`)

Run `app.py` from this directory, e.g. `python3 app.py --steps 1`.
