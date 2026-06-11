use bevy::prelude::*;
use bevy::window::{Window, WindowResolution};
use client::ui::{
    components::*,
    ClientUiPlugin,
};
use client::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

fn setup_headless_ui_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        ClientUiPlugin,
    ));

    app.add_event::<ClientActionRequest>();

    // Spawn a dummy window to ensure layout computations and updates run
    app.world_mut().spawn(Window {
        resolution: WindowResolution::new(1920.0, 1080.0),
        ..default()
    });

    app
}

#[test]
fn test_ui_node_hierarchy() {
    let mut app = setup_headless_ui_app();

    // Run startup systems to spawn the UI layout
    app.update();

    // Verify RootUiNode is present
    let mut root_query = app.world_mut().query_filtered::<Entity, With<RootUiNode>>();
    let root_entity = root_query.get_single(app.world()).expect("Root UI Node missing");

    // Verify presence of main sections: Top HUD, Central Board, Bottom controls
    let mut top_hud_query = app.world_mut().query_filtered::<Entity, With<TopHudNode>>();
    assert!(top_hud_query.get_single(app.world()).is_ok(), "Top HUD not spawned");

    let mut board_query = app.world_mut().query_filtered::<Entity, With<BoardContainerNode>>();
    assert!(board_query.get_single(app.world()).is_ok(), "Board container not spawned");

    let mut bottom_bar_query = app.world_mut().query_filtered::<Entity, With<BottomBarNode>>();
    assert!(bottom_bar_query.get_single(app.world()).is_ok(), "Bottom Bar not spawned");

    // Verify hierarchy children logic
    let children = app.world().get::<Children>(root_entity).expect("Root has no children");
    assert!(!children.is_empty(), "Root Node has no spawned children");
}

#[test]
fn test_wager_card_selection_interaction() {
    let mut app = setup_headless_ui_app();

    // Run startup systems
    app.update();

    // Query for a WagerCardButtonNode with card_type = 1 (Banana)
    let mut banana_query = app.world_mut().query_filtered::<(Entity, &WagerCardButtonNode), With<Button>>();
    let (banana_entity, _) = banana_query
        .iter(app.world())
        .find(|(_, node)| node.card_type == 1)
        .expect("Banana wager card button not found");

    // Simulate clicking the Banana button
    app.world_mut().entity_mut(banana_entity).insert(Interaction::Pressed);

    // Update to trigger interaction systems
    app.update();

    // Verify that ClientActionRequest event was dispatched
    let events = app.world().resource::<Events<ClientActionRequest>>();
    let mut reader = events.get_reader();
    let sent_events: Vec<&ClientActionRequest> = reader.read(events).collect();

    assert_eq!(sent_events.len(), 1, "Expected exactly one ClientActionRequest to be sent");
    if let ClientAction::DraftCard { card_type, cell_index } = &sent_events[0].0 {
        assert_eq!(*card_type, 1, "Expected card_type to be 1 (Banana)");
        assert_eq!(*cell_index, 10, "Expected cell_index to match drafted spot");
    } else {
        panic!("Sent event was not a ClientAction::DraftCard variant");
    }
}
