# ADR 0017 - In-Progress Scorecard Toggle

## Context

The user requested that the "VIEW FULL" button on the leaderboard ticker be renamed to "SCORECARD", and that clicking it during an active match (before the match is completed) should display the in-progress scorecard summary screen. Furthermore, a "BACK TO GAME" button was needed on the scorecard overlay to allow returning to active play.

To implement this while adhering to the core project constraints, we must address three challenges:
1. **Source Code Length Constraint (300-Line Limit)**: Modifying the existing layout files (`match_summary.rs`) and systems files (`scorecard_render.rs`) with custom button nodes and logic could easily push them past the strict 300-line budget limit.
2. **WASM Heap Allocation Constraints**: Dynamically spawning and despawning button entities inside the hot render loop based on whether the game is in progress or completed would cause heap allocation churn, which violates our WASM safety guidelines.
3. **State Leakage**: The toggle state must not leak across multiple matches (i.e. starting a new game must not leave the scorecard open).

## Decision

We implemented the in-progress scorecard toggle with the following architecture:
1. **ShowScorecard Toggle Resource**: Introduced a simple Bevy resource `ShowScorecard(pub bool)` to track if the scorecard has been manually opened.
2. **Modular Button Layout Module**: Extracted the scorecard buttons container spawning logic into a new sibling layout module: `crates/client/src/ui/layout/scorecard_buttons.rs`. This keeps `match_summary.rs` extremely clean (well below 200 lines).
3. **Upfront Spawning and Visibility Toggling**: Spawned all three buttons (the `CloseScorecardButtonNode` for "BACK TO GAME", and the `PlayAgainButtonNode` / `MainMenuButtonNode` pair) upfront when the scorecard container is initialized. We toggle their display styles (`Display::Flex` vs `Display::None`) inside `toggle_match_completed_ui_system` based on whether the game state is `MatchCompleted` or in-progress. This avoids any runtime entity spawning or despawning allocations in the hot loop.
4. **Interaction Handling and Cleanup**: Clicking `"SCORECARD"` sets `ShowScorecard(true)`, and clicking `"BACK TO GAME"` sets `ShowScorecard(false)`. The toggle state is reset to `false` when transitioning out of gameplay or restarting practice.
5. **DPI/Layout Resiliency**: Handled restoring the bottom bar and wager panel layout settings cleanly when returning to the gameplay screen.

## Consequences

- Players can view their in-progress scorecard at any time during the match.
- Code modularization ensures that all modified files remain far below the 300-line limit.
- Memory safety and zero-allocation performance inside the Bevy UI render loop are preserved.
- No state leakage occurs between successive practice matches.
