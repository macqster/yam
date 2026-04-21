# YAM v2 Verification

This document records the current golden-frame verification path.

## Command

```bash
python3 -m v2.check_golden
```

## Contract

- the Go FIGlet one-shot frame must match `docs/v2/GOLDEN_FRAME.md`
- the check should exit non-zero if the frame drifts
- this is a minimal verification path, not the final test suite
- the verification run uses a fixed clock string and day string so the snapshot remains stable
- the Python helper shells out to the Go runtime for the canonical frame
- the scene config file stays separate from the fixed golden test path

## Notes

- keep the check deterministic
- keep it local to the v2 tree
- do not couple it to the legacy visualizer
