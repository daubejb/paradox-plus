use bevy::prelude::*;
use crate::ui::components::*;

pub fn spawn_match_summary_screen(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
                justify_content: JustifyContent::Center, padding: UiRect::all(Val::Px(16.0)),
                display: Display::None, ..default()
            },
            background_color: Color::srgba(0.01, 0.05, 0.02, 0.95).into(),
            z_index: ZIndex::Global(100), ..default()
        },
        MatchCompletedScreenNode,
    )).with_children(|screen| {
        screen.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0), max_width: Val::Px(420.0),
                flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)), border: UiRect::all(Val::Px(2.0)), ..default()
            },
            background_color: Color::srgb(0.01, 0.12, 0.04).into(),
            border_color: Color::srgb(0.85, 0.65, 0.15).into(),
            border_radius: BorderRadius::all(Val::Px(16.0)), ..default()
        }).with_children(|card| {
            card.spawn((
                TextBundle::from_section(
                    "MATCH COMPLETED",
                    TextStyle { font_size: 26.0, color: Color::srgb(0.95, 0.8, 0.2), ..default() },
                ).with_style(Style { margin: UiRect::bottom(Val::Px(4.0)), ..default() }),
                ScorecardTitleTextNode,
            ));

            card.spawn((
                TextBundle::from_section(
                    "Scorecard summary",
                    TextStyle { font_size: 14.0, color: Color::srgb(0.7, 0.85, 0.75), ..default() },
                ).with_style(Style { margin: UiRect::bottom(Val::Px(12.0)), ..default() }),
                PlayerNameTextNode,
            ));

            card.spawn(TextBundle::from_section(
                "FRONT 9",
                TextStyle { font_size: 11.0, color: Color::srgb(0.6, 0.8, 0.65), ..default() },
            ).with_style(Style { align_self: AlignSelf::Start, margin: UiRect::bottom(Val::Px(4.0)), ..default() }));
            spawn_scorecard_table(card, 1..=9, 20);

            card.spawn(NodeBundle { style: Style { height: Val::Px(12.0), ..default() }, ..default() });

            card.spawn(TextBundle::from_section(
                "BACK 9",
                TextStyle { font_size: 11.0, color: Color::srgb(0.6, 0.8, 0.65), ..default() },
            ).with_style(Style { align_self: AlignSelf::Start, margin: UiRect::bottom(Val::Px(4.0)), ..default() }));
            spawn_scorecard_table(card, 10..=18, 21);

            card.spawn(NodeBundle { style: Style { height: Val::Px(16.0), ..default() }, ..default() });

            card.spawn((
                TextBundle::from_section(
                    "TOTAL STROKES: -",
                    TextStyle { font_size: 16.0, color: Color::srgb(0.95, 0.8, 0.2), ..default() }
                ),
                ScorecardCellTextNode { hole_num: 22, is_par: false, is_score: false },
            ));

            card.spawn(NodeBundle { style: Style { height: Val::Px(16.0), ..default() }, ..default() });

            super::scorecard_buttons::spawn_scorecard_buttons(card);
        });
    });
}

fn spawn_scorecard_table(parent: &mut ChildBuilder, hole_range: std::ops::RangeInclusive<u8>, total_idx: u8) {
    parent.spawn(NodeBundle {
        style: Style { flex_direction: FlexDirection::Column, width: Val::Percent(100.0), border: UiRect::all(Val::Px(1.0)), ..default() },
        background_color: Color::srgb(0.0, 0.04, 0.01).into(),
        border_color: Color::srgb(0.1, 0.25, 0.15).into(), ..default()
    }).with_children(|table| {
        spawn_row(table, "Hole", hole_range.clone(), total_idx, true, false, false, Some(Color::srgb(0.01, 0.18, 0.06)));
        spawn_divider(table);
        spawn_row(table, "Par", hole_range.clone(), total_idx, false, true, false, None);
        spawn_divider(table);
        spawn_row(table, "Score", hole_range, total_idx, false, false, true, Some(Color::srgba(1.0, 1.0, 1.0, 0.02)));
    });
}

fn spawn_row(
    table: &mut ChildBuilder,
    label: &str,
    hole_range: std::ops::RangeInclusive<u8>,
    total_idx: u8,
    is_header: bool,
    is_par: bool,
    is_score: bool,
    bg_color: Option<Color>,
) {
    let cell_width = Val::Percent(8.5);
    let label_width = Val::Percent(15.0);
    let mut row_bundle = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row, width: Val::Percent(100.0),
            height: Val::Px(if is_score { 24.0 } else { 22.0 }), align_items: AlignItems::Center, ..default()
        },
        ..default()
    };
    if let Some(bg) = bg_color { row_bundle.background_color = bg.into(); }

    table.spawn(row_bundle).with_children(|row| {
        let label_color = if is_header { Color::WHITE } else if is_par { Color::srgb(0.7, 0.85, 0.75) } else { Color::WHITE };
        spawn_table_cell(row, label_width, label, is_header, label_color);

        for h in hole_range {
            let mut cell = row.spawn(NodeBundle {
                style: Style {
                    width: cell_width, height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default()
                },
                ..default()
            });

            if is_header {
                cell.with_children(|c| {
                    c.spawn(TextBundle::from_section(h.to_string(), TextStyle { font_size: 11.0, color: Color::WHITE, ..default() }));
                });
            } else {
                cell.with_children(|c| {
                    c.spawn((
                        TextBundle::from_section(
                            "-",
                            TextStyle {
                                font_size: 12.0,
                                color: if is_par { Color::srgb(0.7, 0.85, 0.75) } else { Color::WHITE },
                                ..default()
                            },
                        ),
                        ScorecardCellTextNode { hole_num: h, is_par, is_score },
                    ));
                });
            }
        }

        let mut total_cell = row.spawn(NodeBundle {
            style: Style {
                width: cell_width, height: Val::Percent(100.0),
                justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default()
            },
            ..default()
        });

        let total_label = if total_idx == 20 { "OUT" } else { "IN" };
        let total_color = Color::srgb(0.95, 0.8, 0.2); // Gold

        if is_header {
            total_cell.with_children(|c| {
                c.spawn(TextBundle::from_section(total_label, TextStyle { font_size: 11.0, color: total_color, ..default() }));
            });
        } else {
            total_cell.with_children(|c| {
                c.spawn((
                    TextBundle::from_section("-", TextStyle { font_size: 12.0, color: total_color, ..default() }),
                    ScorecardCellTextNode { hole_num: total_idx, is_par, is_score },
                ));
            });
        }
    });
}

fn spawn_table_cell(row: &mut ChildBuilder, width: Val, text: &str, is_bold: bool, color: Color) {
    row.spawn(NodeBundle {
        style: Style {
            width, height: Val::Percent(100.0),
            justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default()
        },
        ..default()
    }).with_children(|cell| {
        cell.spawn(TextBundle {
            text: Text::from_section(text, TextStyle { font_size: 11.0, color, ..default() }),
            style: Style {
                margin: if is_bold { UiRect::all(Val::Px(1.0)) } else { UiRect::ZERO },
                ..default()
            },
            ..default()
        });
    });
}

fn spawn_divider(table: &mut ChildBuilder) {
    table.spawn(NodeBundle {
        style: Style { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
        background_color: Color::srgb(0.1, 0.25, 0.15).into(), ..default()
    });
}
