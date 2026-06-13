# ADR 0016 - Scorecard History Tracking

## Context

To implement the end-of-round scorecard summary screen, the game needs to know the player's stroke counts for each individual completed hole (1 through 18).
Prior to this decision, the `Scorecard` struct in the shared `protocol` crate only tracked cumulative statistics (`running_strokes` for the current hole and `total_strokes` across all played holes). It had no concept of individual hole scores, meaning the client could not display a hole-by-hole score breakdown at the end of the round.

Furthermore, we operate under strict Zero-Heap and Type-Safe Bounded Serialization constraints to prevent WASM heap exhaustion and potential network security vectors (attacks on memory deserialization).

## Decision

We updated the `protocol` and client loopback simulation to persistently track and synchronize hole-by-hole scores:
1. **Protocol Update**: Added `strokes_per_hole: HVec<u16, 18>` to the shared `Scorecard` struct definition. This utilizes a stack-allocated bounded vector (`heapless::Vec`) that consumes exactly 36 bytes (18 * 2 bytes) of packet payload overhead, preserving maximum packet size safety limits.
2. **Client State Persistence**: Added `strokes_per_hole: HVec<u16, 18>` to the loopback simulation's `OfflineServerState`.
3. **Recording Logic**: When a hole is completed, loopback handlers (e.g. `roll.rs` and `banana.rs`) cast the current hole strokes to `u16` safely using `u16::try_from` and push it to `state.strokes_per_hole`.
4. **Authoritative Sync**: The scorecard sent to the client via `ServerUpdate::StateSync` is populated from `state.strokes_per_hole` using a centralized `build_scorecard()` method, which calculates cumulative `total_strokes` safely via a saturating fold over the completed hole strokes.
5. **Server Stubs**: Headless server synchronizations and tests in `fsm_tests.rs` continue to compile successfully by initializing the field with `HVec::new()`.

## Consequences

- The client now possesses complete hole-by-hole stroke data, enabling detailed scorecard rendering.
- Bounded serialization guarantees zero-heap overhead during postcard encoding/decoding.
- Backward compatibility is maintained with stubbed server-side test frames.
- Code duplication was eliminated across local simulation handlers.
