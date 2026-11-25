# Changelog

## 0.2.4 — 2026-11-25

- Bound decompilation to the configured timeout across symbolic execution, simplification, and folding. Those stages share one deadline, and trace construction stops at a fixed step budget so a branch-heavy contract truncates instead of exhausting memory (GitHub #18).
- `folder::fold_until` returns `Result` and no longer hides failures with `catch_unwind`. On timeout or path limit the unfolded trace is kept and the failure is logged (GitHub #5).
- Replace always-true assertions in `test_invalid_hex` and `test_decompile_push_add_return` with the expected outcome (GitHub #1).
- Drop the unused `#[allow(deprecated)]` around `assert_cmd::Command` in the CLI tests (GitHub #9).

## 0.2.3 — 2026-10-15

- Fix panic `attempt to negate with overflow` in signed EVM arithmetic (`to_signed` / `SLT` / `SDIV` / `SMOD` / `SAR`) on values whose two's-complement magnitude has low 128 bits equal to `2^127` (GitHub #17). Signed ops now use wrapping `U256` two's complement (`negate_u256`); `to_signed` / `from_signed` use `wrapping_neg`.
- Correct `UINT_256_CEILING` documentation: it is `2^256 ≡ 0 (mod 2^256)`, not `2^255`. Add `UINT_255_CEILING` (`2^255`) for the signed-arithmetic boundary (GitHub #6).
- Replace crash-adjacent `.unwrap()` / mutex `.lock().unwrap()` in library paths (`sparser`, `folder`, `decompiler`, signature cache) with `if let` / `unwrap_or` / poison recovery (GitHub #4).
