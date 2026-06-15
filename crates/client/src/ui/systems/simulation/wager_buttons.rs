use bevy::prelude::*;
use protocol::messages::CardType;
use crate::ui::components::{SelectedWagerCard, ClientScorecards, WagerCardButtonNode, WagerCardQtyTextNode};

pub fn update_wager_buttons_render_system(
    selected_card: Res<SelectedWagerCard>,
    scorecards: Res<ClientScorecards>,
    mut button_query: Query<(&mut Style, &mut BackgroundColor, &mut BorderColor, &WagerCardButtonNode, &Children)>,
    mut text_query: Query<&mut Text, With<WagerCardQtyTextNode>>,
) {
    // Prevent layout thrashing: only run when scorecards or selection state actually changes
    if !scorecards.is_changed() && !selected_card.is_changed() {
        return;
    }

    let score_val = scorecards.0.first();
    let shield_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 0).count()).unwrap_or(0);
    let banana_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 1).count()).unwrap_or(0);
    let die_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 2).count()).unwrap_or(0);

    for (mut style, mut bg_color, mut border_color, button_node, children) in button_query.iter_mut() {
        let count = match button_node.card_type {
            CardType::Shield => shield_count,
            CardType::Banana => banana_count,
            CardType::GoldenDie => die_count,
        };

        let is_selected = selected_card.0 == Some(button_node.card_type);

        if count == 0 {
            // Disabled state: desaturated and semi-transparent
            *bg_color = Color::srgba(0.15, 0.15, 0.15, 0.3).into();
            *border_color = Color::srgba(0.25, 0.25, 0.25, 0.3).into();
            style.border = UiRect::all(Val::Px(1.0));

            // Mute the child text colors
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = Color::srgba(0.6, 0.6, 0.6, 0.3);
                    }
                }
            }
        } else {
            // Enabled state: set base colors based on card type
            let (base_bg, base_border) = match button_node.card_type {
                CardType::Shield => (Color::srgb(0.2, 0.35, 0.5), Color::srgb(0.4, 0.6, 0.8)),
                CardType::Banana => (Color::srgb(0.7, 0.6, 0.1), Color::srgb(0.9, 0.8, 0.2)),
                CardType::GoldenDie => (Color::srgb(0.6, 0.1, 0.1), Color::srgb(0.8, 0.3, 0.3)),
            };

            if is_selected {
                // Highlighted/Selected state: thick golden border
                *bg_color = base_bg.into();
                *border_color = Color::srgb(1.0, 0.84, 0.0).into(); // Gold
                style.border = UiRect::all(Val::Px(3.0));
            } else {
                // Normal enabled state
                *bg_color = base_bg.into();
                *border_color = base_border.into();
                style.border = UiRect::all(Val::Px(1.0));
            }

            // Restore text color
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = Color::WHITE;
                    }
                }
            }
        }
    }
}
