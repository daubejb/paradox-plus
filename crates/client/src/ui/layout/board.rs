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
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },
        BoardContainerNode,
    ));
}
