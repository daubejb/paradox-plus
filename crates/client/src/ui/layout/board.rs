use bevy::prelude::*;
use crate::ui::components::BoardContainerNode;

pub fn spawn_central_board(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        },
        BoardContainerNode,
    )).with_children(|board| {
        // Oval Board Track Container (Mock representation using wrapped cells)
        board.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            ..default()
        }).with_children(|grid| {
            // Spawn a few sample board spaces representing the course track cells:
            // Tee (0), Fairway (1..5), Bunker (6), Water (7), Green (8)
            let cell_names = ["TEE", "1 FW", "2 FW", "3 ROUGH", "4 BUNKER", "5 FW", "6 WATER", "GREEN"];
            let cell_colors = [
                Color::srgb(0.2, 0.6, 0.3), // Tee
                Color::srgb(0.3, 0.7, 0.4), // Fairway
                Color::srgb(0.3, 0.7, 0.4),
                Color::srgb(0.25, 0.5, 0.3), // Rough
                Color::srgb(0.8, 0.7, 0.5), // Bunker (Sand)
                Color::srgb(0.3, 0.7, 0.4),
                Color::srgb(0.1, 0.4, 0.7), // Water
                Color::srgb(0.1, 0.5, 0.2), // Green
            ];

            for (idx, &name) in cell_names.iter().enumerate() {
                grid.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(70.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: cell_colors[idx].into(),
                    border_color: Color::srgb(0.05, 0.15, 0.10).into(),
                    ..default()
                }).with_children(|cell| {
                    cell.spawn(TextBundle::from_section(
                        name,
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            }
        });
    });
}
