# ADR 0003: MDP State Index Simplification

## Context
The AI Bot Decision Solver utilizes Value Iteration over a flat table to compute optimal policies. The game's state space $\mathcal{S}$ is defined by:
$$s = (x_{cell}, d, x_{origin\_cell}, \text{triggered\_wagers})$$
Where $x_{cell}$ is the discrete cell index, $d$ is the movement direction, $x_{origin\_cell}$ is the shot origin, and $\text{triggered\_wagers}$ is a list of wagers triggered during the current turn.

However, attempting to map this entire combinatoric space into a flat table index under the maximum 131,072 size boundary would lead to:
1. Combinatorial explosion (exceeding index bounds).
2. Index collisions (loss of state uniqueness).
3. Degraded bot performance due to `None` values on overflow.

## Decision
We decided to simplify the value table index representation by separating the value table representation from the procedural transition simulation:
1. The `MdpSolverTable` value entries only index `cell_index` and `direction` using the formula:
   $$\text{index} = (\text{cell\_index} \ll 1) \mid \text{direction}$$
   This completely fits within the first 131,072 indices (supporting up to 65,536 course track cells) and guarantees 0% collision rates.
2. During the procedural transition sweeps, the solver carries `origin_cell` and `triggered_wagers` as dynamic variables to correctly calculate path slide physics, loop damper limits, and opponent wager traps/rewards.
3. Once a transition resolves to a final state $s'$, its expected future cost $V(s')$ is retrieved from the table using the simplified flat index. This is mathematically correct because a turn comes to rest at the end of a shot, resetting `triggered_wagers` and updating `origin_cell` to the new position for the next shot.

## Consequences
* **State Space Integrity:** State mapping is 100% collision-free and fits well under the 131,072 boundary constraint.
* **Algorithm Correctness:** Procedural transitions remain fully wager-aware, preserving anti-loop damping and hazard penalties.
* **Performance Boost:** Shrinks active sweeps to at most 512 states (256 cells * 2 directions), ensuring sub-millisecond sweep execution times.
