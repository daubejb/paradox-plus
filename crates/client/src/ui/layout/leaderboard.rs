use bevy::prelude::*;
use crate::ui::components::{
    LeaderboardTickerContainerNode, LeaderboardTickerTrackNode, ScorecardButtonNode
};

pub fn spawn_leaderboard_ticker(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(35.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
        background_color: Color::srgba(0.03, 0.09, 0.06, 0.95).into(), // Dark green translucent background
        border_color: Color::srgb(0.1, 0.25, 0.18).into(),
        ..default()
    }).with_children(|ticker| {
        // 1. Leaderboard label (Fixed Left)
        ticker.spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.15, 0.10).into(), // Dark solid green
            border_color: Color::srgb(0.1, 0.25, 0.18).into(),
            ..default()
        }).with_children(|label| {
            label.spawn(TextBundle::from_section(
                "📊 LEADERBOARD",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.9, 0.8, 0.2), // Bright gold
                    ..default()
                },
            ));
        });

        // 2. Middle Scroll Container (Clipping viewport)
        ticker.spawn((
            NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    overflow: Overflow::clip(), // Clip overflowing children
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                    ..default()
                },
                ..default()
            },
            LeaderboardTickerContainerNode,
        )).with_children(|scroll_viewport| {
            // Ticker Track containing players (offset starts at 0)
            scroll_viewport.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(20.0),
                        position_type: PositionType::Relative,
                        left: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                LeaderboardTickerTrackNode { scroll_offset: 0.0 },
            ));
        });

        // 3. Scorecard Button (Fixed Right)
        ticker.spawn((
            ButtonBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    border: UiRect::left(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgb(0.05, 0.15, 0.10).into(),
                border_color: Color::srgb(0.1, 0.25, 0.18).into(),
                ..default()
            },
            ScorecardButtonNode,
        )).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "SCORECARD",
                TextStyle {
                    font_size: 10.0,
                    color: Color::srgb(0.7, 0.9, 0.8),
                    ..default()
                },
            ));
        });
    });
}
