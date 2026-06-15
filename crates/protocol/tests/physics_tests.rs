use protocol::messages::{ClientAction, MAX_PACKET_SIZE};
use protocol::physics::{SlideTracker, SlideError};
use protocol::terrain::{
    TerrainType, resolve_bunker_escape, resolve_green_putting, resolve_standard_landing
};
use fixed::types::I32F32;
use heapless::String as HString;

#[test]
fn test_postcard_serialization_bounds() {
    let action = ClientAction::JoinRoom {
        code: HString::try_from("123456").unwrap(),
        name: HString::try_from("Alice").unwrap(),
    };
    
    let mut buffer = [0u8; MAX_PACKET_SIZE];
    let serialized = postcard::to_slice(&action, &mut buffer).expect("Serialization failed");
    
    assert!(serialized.len() > 0);
    assert!(serialized.len() <= MAX_PACKET_SIZE);
    
    let deserialized: ClientAction = postcard::from_bytes(serialized).expect("Deserialization failed");
    assert_eq!(action, deserialized);
}

#[test]
fn test_slide_tracker_cycle_detection() {
    let mut tracker = SlideTracker::new();
    assert_eq!(tracker.slide_count(), 0);
    
    // Visit first few cells
    tracker.record_and_check_cycle(1).expect("Should record");
    tracker.record_and_check_cycle(2).expect("Should record");
    tracker.record_and_check_cycle(3).expect("Should record");
    assert_eq!(tracker.slide_count(), 3);
    
    // Visit duplicate cell (cycle)
    let result = tracker.record_and_check_cycle(2);
    assert_eq!(result, Err(SlideError::CycleDetected));
}

#[test]
fn test_slide_tracker_limit_exceeded() {
    let mut tracker = SlideTracker::new();
    for i in 1..=16 {
        tracker.record_and_check_cycle(i).expect("Should record");
    }
    
    // Exceed limit
    let result = tracker.record_and_check_cycle(17);
    assert_eq!(result, Err(SlideError::LimitExceeded));
}

#[test]
fn test_safe_fixed_math() {
    let a = I32F32::from_num(5);
    let b = I32F32::from_num(10);
    
    assert_eq!(a.saturating_add(b), I32F32::from_num(15));
    assert_eq!(b.saturating_sub(a), I32F32::from_num(5));
    assert_eq!(a.saturating_mul(b), I32F32::from_num(50));
    assert_eq!(b.saturating_div(a), I32F32::from_num(2));
}

#[test]
fn test_terrain_strategies() {
    // Bunker escapes
    let res_bunker_fail = resolve_bunker_escape(10, 15, 3); // Odd roll sum
    assert_eq!(res_bunker_fail.final_cell, 10);
    assert_eq!(res_bunker_fail.shot_strokes, 1);
    assert_eq!(res_bunker_fail.penalty_strokes, 0);
    assert_eq!(res_bunker_fail.completed_hole, false);

    let res_bunker_success = resolve_bunker_escape(10, 15, 4); // Even roll sum
    assert_eq!(res_bunker_success.final_cell, 15);
    assert_eq!(res_bunker_success.shot_strokes, 1);
    assert_eq!(res_bunker_success.penalty_strokes, 0);
    assert_eq!(res_bunker_success.completed_hole, false);

    // Green automatic putting
    let res_g0 = resolve_green_putting(20, 0);
    assert_eq!(res_g0.final_cell, 20);
    assert_eq!(res_g0.penalty_strokes, 0);
    assert_eq!(res_g0.completed_hole, true);

    let res_g1 = resolve_green_putting(20, 1);
    assert_eq!(res_g1.penalty_strokes, 1);

    let res_g2 = resolve_green_putting(20, 2);
    assert_eq!(res_g2.penalty_strokes, 2);

    let res_g3 = resolve_green_putting(20, 3);
    assert_eq!(res_g3.penalty_strokes, 3);

    // Standard terrain
    let res_fairway = resolve_standard_landing(15, 10, TerrainType::Fairway);
    assert_eq!(res_fairway.final_cell, 15);
    assert_eq!(res_fairway.shot_strokes, 1);
    assert_eq!(res_fairway.penalty_strokes, 0);

    let res_water = resolve_standard_landing(15, 10, TerrainType::Water);
    assert_eq!(res_water.final_cell, 15);
    assert_eq!(res_water.shot_strokes, 1);
    assert_eq!(res_water.penalty_strokes, 1);

    let res_ob = resolve_standard_landing(15, 10, TerrainType::OutOfBounds);
    assert_eq!(res_ob.final_cell, 10); // Resets back to origin
    assert_eq!(res_ob.shot_strokes, 1);
    assert_eq!(res_ob.penalty_strokes, 1);
}
