# Changelog

All notable changes to `muconde-ipc` are documented here. The crate follows
[Semantic Versioning](https://semver.org/); before 1.0, minor bumps carry additive protocol changes.

## [0.2.0] - Unreleased

### Added

- `IntegrationTestProgress { deployment_id, test_name, critical, state, duration_ms, timestamp_epoch_ms }`:
  per-test progress emitted by `MuConDe-agent` while the integration gate runs (#13, #14, #15).
- `TestState` enum (`Pending`, `Running`, `Passed`, `Failed`; JSON `snake_case`) with `is_terminal()`.
- New fory registration IDs: `FORY_ID_IT_PROGRESS = 13` (`IntegrationTestProgress`) and
  `FORY_ID_TEST_STATE = 14` (`TestState`). IDs `1..=12` are unchanged (#16).
- New frame discriminant `MSG_TYPE_IT_PROGRESS = 0x0C`, plus `is_legacy_msg_type()` and
  `is_known_msg_type()` helpers.
- Round-trip and backward-compatibility tests (`tests/it_progress_compat_tests.rs`) proving a
  0.1.x peer skips `0x0C` frames and still decodes the final `IntegrationTestReport` (#17).

### Compatibility

- Wire protocol version (`CURRENT_VERSION = 1`) and the 24-byte header are unchanged.
- The aggregate `IntegrationTestReport` stays the final gate verdict. Older controllers
  ignore the unknown message type, so the change is phase 1 (side-by-side) of the
  three-phase migration.

## [0.1.0]

- Initial MCDE 24-byte framing and 11 deployment RPC schemas.
