use bevy::prelude::*;
use crate::replication::Ball;
use crate::ui::components::{BoardCellNode, PlayerTokenMarker, BoardContainerNode, CurrentHole, GameSettings, WagerTokenMarker, ClientWagers};

pub fn update_board_cell_visuals(
    ball_query: Query<&Ball>,
    cell_query: Query<(Entity, &BoardCellNode)>,
    mut token_query: Query<(&Parent, &mut Style), With<PlayerTokenMarker>>,
) {
    let ball = match ball_query.get_single() {
        Ok(b) => b,
        Err(_) => return,
    };

    let active_cell_idx = ball.cell_index as u32;

    for (cell_entity, cell_node) in cell_query.iter() {
        for (parent, mut style) in token_query.iter_mut() {
            if parent.get() == cell_entity {
                if cell_node.index == active_cell_idx {
                    style.display = Display::Flex;
                } else {
                    style.display = Display::None;
                }
            }
        }
    }
}

pub fn update_wagers_on_board(
    wagers: Res<ClientWagers>,
    cell_query: Query<(Entity, &BoardCellNode)>,
    mut wager_marker_query: Query<(&Parent, &mut Style, &mut BackgroundColor, &Children), With<WagerTokenMarker>>,
    mut text_query: Query<&mut Text>,
) {
    for (cell_entity, cell_node) in cell_query.iter() {
        for (parent, mut style, mut bg_color, children) in wager_marker_query.iter_mut() {
            if parent.get() == cell_entity {
                if let Some(wager) = wagers.0.iter().find(|w| w.cell_index == cell_node.index) {
                    style.display = Display::Flex;
                    *bg_color = match wager.card_type {
                        0 => Color::srgb(0.2, 0.4, 0.8).into(),   // Shield (Blue)
                        1 => Color::srgb(0.9, 0.8, 0.1).into(),   // Banana (Yellow)
                        _ => Color::srgb(0.8, 0.1, 0.1).into(),   // Golden Die (Red)
                    };
                    if let Some(&text_child) = children.first() {
                        if let Ok(mut text) = text_query.get_mut(text_child) {
                            text.sections[0].value = match wager.card_type {
                                0 => "S".to_string(),
                                1 => "B".to_string(),
                                _ => "G".to_string(),
                            };
                        }
                    }
                } else {
                    style.display = Display::None;
                }
            }
        }
    }
}

pub fn rebuild_board_on_hole_change_system(
    mut commands: Commands,
    current_hole: Res<CurrentHole>,
    settings: Res<GameSettings>,
    board_container_query: Query<Entity, With<BoardContainerNode>>,
) {
    if !current_hole.is_changed() {
        return;
    }

    if let Ok(container_entity) = board_container_query.get_single() {
        // Recursive despawn of old board grid and cells
        commands.entity(container_entity).despawn_descendants();

        // Rebuild new board cells matching current_hole.0 config
        commands.entity(container_entity).with_children(|board| {
            board.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(450.0),
                    height: Val::Px(580.0),
                    position_type: PositionType::Relative,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            }).with_children(|relative_board| {
                if let Some(preset) = protocol::terrain::presets::get_course_preset(&settings.course, current_hole.0) {
                    for (idx, &cell_type) in preset.cells.iter().enumerate() {
                        if idx >= crate::ui::layout::board::BOARD_TILE_POSITIONS.len() {
                            break;
                        }
                        let (left_pct, top_pct) = crate::ui::layout::board::BOARD_TILE_POSITIONS[idx];

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

                        relative_board.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(crate::ui::layout::board::TILE_SIZE),
                                    height: Val::Px(crate::ui::layout::board::TILE_SIZE),
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(left_pct - crate::ui::layout::board::TILE_OFFSET_X),
                                    top: Val::Percent(top_pct - crate::ui::layout::board::TILE_OFFSET_Y),
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

                            // Ball location indicator (bottom right)
                            cell.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(8.0),
                                        height: Val::Px(8.0),
                                        display: Display::None,
                                        position_type: PositionType::Absolute,
                                        bottom: Val::Px(2.0),
                                        right: Val::Px(2.0),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.95, 0.85, 0.1).into(),
                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                    ..default()
                                },
                                PlayerTokenMarker,
                            ));

                            // Wager token indicator (top left)
                            cell.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(12.0),
                                        height: Val::Px(12.0),
                                        display: Display::None,
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(2.0),
                                        left: Val::Px(2.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::NONE.into(),
                                    border_radius: BorderRadius::all(Val::Px(3.0)),
                                    ..default()
                                },
                                WagerTokenMarker,
                            )).with_children(|wager_indicator| {
                                wager_indicator.spawn(TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font_size: 8.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                            });
                        });
                    }
                }
            });
        });
    }
}
