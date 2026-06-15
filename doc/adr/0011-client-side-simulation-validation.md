# ADR 0011: Client-Side Simulation Validation and Coordinator Server Rationale

## Context
In Paradox Plus, gameplay mechanics such as player coordinates, course terrain mapping, and wager cards are simulated within the local loopback server or client instances rather than on the central multiplayer coordinator server. This keeps the coordinator server extremely lightweight, operating purely as a state transition coordinator. 

However, this architecture introduces the risk of client-side validation bypass (e.g. if a compromised client sends a roll request with 2 dice while on a Rough cell). We need to clarify the validation boundaries, security trade-offs, and mitigation strategies.

## Decision
1. **Lightweight Coordinator Server:** The central server remains a pure FSM phase coordinator to minimize latency and processing overhead. It does not track player coordinates, course presets, or wagers.
2. **Authoritative Local Simulation:** The practice mode loopback server acts as the authoritative gameplay simulation layer for single-player mode. It authoritatively clamps dice rolls to 1 die when a player is on Rough (without an active own Shield) or in a Bunker.
3. **UI Mitigation:** The UI client hides the "Roll 2" button when the active player is in the Rough (without an active own Shield) or Bunker.
4. **Active Player Lookup:** The UI client correctly checks the position of the currently active player by looking up their coordinates in the `StateSync` payload rather than hardcoding the first element.

## Consequences
* **Extremely Low Server Footprint:** The coordinator server scales exceptionally well because it does not run heavy physics, coordinate calculation, or geometry lookup.
* **Practice Mode Security:** In practice/offline mode, the loopback handler acts as the local authoritative server, preventing any UI/action bypass.
* **Multiplayer Security Trade-off:** The design accepts that multiplayer state simulation runs in a peer-to-peer / client-replicated fashion where clients validate and execute rules locally. If multiplayer security becomes a priority in the future, the server FSM would need to load course presets and track player positions.
