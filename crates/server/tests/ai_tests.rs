use server::ai::mdp_state::MdpState;
use server::ai::mdp_solver::{
    MdpSolverTable, initialize_table, value_iteration_sweep, resolve_physics_step
};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use protocol::physics::MovementDirection;
use protocol::messages::WagerToken;
use std::sync::atomic::AtomicBool;
use fixed::types::I32F32;

#[test]
fn test_solver_convergence() {
    let mut cells = heapless::Vec::new();
    // TeeBox (0), Fairway (1..5), Green(0) (6)
    cells.push(TerrainType::TeeBox).unwrap();
    for _ in 1..=5 {
        cells.push(TerrainType::Fairway).unwrap();
    }
    cells.push(TerrainType::Green(0)).unwrap();

    let course = ActiveCourseTrack {
        hole_index: 1,
        par: 3,
        total_cells: 7,
        cells,
    };

    let mut table = MdpSolverTable::new();
    initialize_table(&mut table, &course);

    // Verify initial values
    // Cup index is 6 (Green 0), so table.values[12] (Forward) and [13] (Reverse) should be 0
    let cup_idx_f = MdpState::new(6, MovementDirection::Forward, 6, heapless::Vec::new()).to_index().unwrap();
    assert_eq!(table.values[cup_idx_f], I32F32::ZERO);

    let cancel_flag = AtomicBool::new(false);
    let converged = value_iteration_sweep(&mut table, &course, &[], 1, &cancel_flag);
    
    assert!(converged, "Solver failed to converge");

    // The start state cell 1 should have a converged expected strokes value > 0
    let start_idx = MdpState::new(1, MovementDirection::Forward, 1, heapless::Vec::new()).to_index().unwrap();
    let start_value = table.values[start_idx];
    assert!(start_value > I32F32::ZERO, "Start cell expected strokes should be positive");
    assert!(start_value < I32F32::from_num(10), "Start cell expected strokes should be reasonable");
}

#[test]
fn test_solver_damper_prevention() {
    let mut cells = heapless::Vec::new();
    cells.push(TerrainType::TeeBox).unwrap();
    for _ in 1..=10 {
        cells.push(TerrainType::Fairway).unwrap();
    }
    cells.push(TerrainType::Green(0)).unwrap();

    let course = ActiveCourseTrack {
        hole_index: 1,
        par: 3,
        total_cells: 12,
        cells,
    };

    let wagers = vec![
        WagerToken {
            card_type: 1, // Trickster Banana
            owner_id: 2, // Opponent owned
            cell_index: 5,
        },
        WagerToken {
            card_type: 1, // Trickster Banana
            owner_id: 2, // Opponent owned
            cell_index: 1,
        },
    ];

    // Case 1: Start at cell 1, roll 4 to land on cell 5 (untriggered).
    // Land on 5: triggers Banana, pushes back 4 spaces to cell 1.
    // Since cell 1 is occupied by another Trickster Banana, ball slides forward to cell 2.
    // Cell 2 is valid/unoccupied, so ball stops at cell 2.
    // Only cell 5 wager is triggered.
    let start_state = MdpState::new(1, MovementDirection::Forward, 1, heapless::Vec::new());
    let table = vec![I32F32::ZERO; 131072];
    let (next_state, reward) = resolve_physics_step(start_state, 4, &course, &wagers, 1, &table);

    assert_eq!(next_state.cell_index, 2);
    assert_eq!(next_state.triggered_wagers.len(), 1);
    assert_eq!(next_state.triggered_wagers[0], 5);
    assert_eq!(reward, I32F32::from_num(1));

    // Case 2: Start at cell 1, roll 4 to land on cell 5, but cell 5 is already in triggered_wagers.
    // It should NOT trigger again, and the ball remains on cell 5.
    let mut triggered = heapless::Vec::new();
    triggered.push(5).unwrap();
    let start_state_damper = MdpState::new(1, MovementDirection::Forward, 1, triggered);
    let (next_state_damper, reward_damper) = resolve_physics_step(start_state_damper, 4, &course, &wagers, 1, &table);

    assert_eq!(next_state_damper.cell_index, 5);
    assert_eq!(next_state_damper.triggered_wagers.len(), 1);
    assert_eq!(reward_damper, I32F32::from_num(1), "Reward should only be the standard 1 stroke");
}
