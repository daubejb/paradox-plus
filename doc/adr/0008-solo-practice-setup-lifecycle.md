# ADR 0008: Solo Practice Setup Lifecycle

## Context
We need to support a configuration screen before entering Solo Practice gameplay. This setup overlay allows the player to choose their nickname, select the course (Green or Blue), and select the gameplay mode (Standard Play or Wager Cards). 

To respect client-server boundary encapsulation, client UI systems must not directly mutate authoritative server state resources (`OfflineServerState`). Additionally, updating screen layouts or text node properties on every frame (hot-loop updating) causes expensive layout recalculations in Bevy's Taffy engine, leading to major performance degradation on mobile and WebAssembly platforms.

## Decision
We made the following architectural decisions to implement the Solo Practice Setup screen securely and performantly:
1. **Three-State Lifecycle Machine:** We expanded the state machine `ClientScreenState` from a binary switch to three discrete states: `Landing`, `SoloSetup`, and `Gameplay`. Visibility changes (`Display::Flex` / `Display::None`) are triggered exactly once during `OnEnter` transition systems to avoid frame-by-frame layout thrashing.
2. **Authoritative Play Request Action:** When the player clicks "PLAY GAME", the client UI dispatches a `ClientAction::StartPractice` action containing the chosen nickname, course, and game mode. The loopback server system intercepts this packet, authoritatively overrides its default parameters, and broadcasts a fresh initial `ServerUpdate::StateSync` to start the match.
3. **Change-Detected UI Binding:** Selection highlights, radio indicators, and name inputs are updated reactively inside the `update_setup_screen_ui` system using Bevy's `Changed<GameSettings>` guard. This ensures layout properties are modified only when the settings resource undergoes actual mutations.
4. **Native Keyboard Input Polling:** Nickname editing is processed completely inside the Bevy ECS by listening to the `ReceivedCharacter` event stream and monitoring backspaces when the input box is focused, eliminating any reliance on DOM input components or external WebViews.
5. **Separate Setup Submodule:** To respect the 300-line budget limit, setup-specific systems are isolated into `crates/client/src/ui/systems/setup.rs` and the layout spawner into `crates/client/src/ui/layout/setup.rs`.

## Consequences
* **Strict Encapsulation:** The client UI remains purely visual and reactive, communicating configuration updates to the authoritative server via type-safe protocol actions. This architecture makes it extremely easy to reuse the setup configuration screen for online multiplayer matches in the future.
* **Layout Performance:** Eliminating per-frame style mutations prevents Bevy from re-evaluating the Taffy UI tree every frame, maintaining a high frame rate on low-end mobile and web platforms.
* **Pure Rust ECS Portability:** Hand-crafted text fields and circular radio buttons render natively in Bevy UI, ensuring full portability across native desktop and WASM targets without requiring web tech (HTML/CSS/JS/DOM).
