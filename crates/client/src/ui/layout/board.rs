use bevy::prelude::*;
use crate::ui::components::{BoardContainerNode, BoardCellNode, PlayerTokenMarker};

pub fn spawn_central_board(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },
        BoardContainerNode,
    )).with_children(|board| {
        // Oval Board Track Container (representation using wrapped cells)
        board.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(95.0),
                height: Val::Percent(95.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                column_gap: Val::Px(6.0),
                row_gap: Val::Px(6.0),
                ..default()
            },
            ..default()
        }).with_children(|grid| {
            // Load Green Course - Hole 1 dynamically
            if let Some(preset) = protocol::terrain::presets::get_course_preset("green", 1) {
                for (idx, &cell_type) in preset.cells.iter().enumerate() {
                    let name = match cell_type {
                        protocol::terrain::TerrainType::TeeBox => "TEE".to_string(),
                        protocol::terrain::TerrainType::Fairway => format!("{} FW", idx),
                        protocol::terrain::TerrainType::Rough => format!("{} RGH", idx),
                        protocol::terrain::TerrainType::Bunker => format!("{} BNK", idx),
                        protocol::terrain::TerrainType::Water => format!("{} WTR", idx),
                        protocol::terrain::TerrainType::OutOfBounds => format!("{} OB", idx),
                        protocol::terrain::TerrainType::Green(tier) => format!("G{}", tier),
                    };

                    let color = match cell_type {
                        protocol::terrain::TerrainType::TeeBox => Color::srgb(0.2, 0.6, 0.3),
                        protocol::terrain::TerrainType::Fairway => Color::srgb(0.3, 0.7, 0.4),
                        protocol::terrain::TerrainType::Rough => Color::srgb(0.25, 0.5, 0.3),
                        protocol::terrain::TerrainType::Bunker => Color::srgb(0.8, 0.7, 0.5),
                        protocol::terrain::TerrainType::Water => Color::srgb(0.1, 0.4, 0.7),
                        protocol::terrain::TerrainType::OutOfBounds => Color::srgb(0.9, 0.2, 0.2),
                        protocol::terrain::TerrainType::Green(_) => Color::srgb(0.1, 0.5, 0.2),
                    };

                    grid.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(46.0),
                                height: Val::Px(46.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            background_color: color.into(),
                            border_color: Color::srgb(0.05, 0.15, 0.10).into(),
                            ..default()
                        },
                        BoardCellNode { index: idx as u32 },
                    )).with_children(|cell| {
                        cell.spawn(TextBundle::from_section(
                            name,
                            TextStyle {
                                font_size: 9.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));

                        // Spawn a hidden child player token marker inside each cell
                        cell.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(8.0),
                                    height: Val::Px(8.0),
                                    display: Display::None, // Hidden by default
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(1.0),
                                    right: Val::Px(1.0),
                                    ..default()
                                },
                                background_color: Color::srgb(0.95, 0.85, 0.1).into(), // Bright gold player dot
                                ..default()
                            },
                            PlayerTokenMarker,
                        ));
                    });
                }
            }
        });
    });
}
