use bevy::prelude::*;
use crate::ui::components::*;
use crate::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

pub fn show_setup_screen_system(
    mut landing_query: Query<&mut Style, With<LandingScreenNode>>,
    mut setup_query: Query<&mut Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>,
    mut gameplay_query: Query<&mut Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>,
) {
    if let Ok(mut style) = landing_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = setup_query.get_single_mut() {
        style.display = Display::Flex;
    }
    if let Ok(mut style) = gameplay_query.get_single_mut() {
        style.display = Display::None;
    }
}

pub fn handle_setup_button_clicks(
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<ClientScreenState>>,
    mut events: EventWriter<ClientActionRequest>,
    green_btn: Query<&Interaction, (Changed<Interaction>, With<CourseGreenButtonNode>)>,
    blue_btn: Query<&Interaction, (Changed<Interaction>, With<CourseBlueButtonNode>)>,
    std_btn: Query<&Interaction, (Changed<Interaction>, With<ModeStandardButtonNode>)>,
    wager_btn: Query<&Interaction, (Changed<Interaction>, With<ModeWagerButtonNode>)>,
    input_box: Query<&Interaction, (Changed<Interaction>, With<NicknameInputContainerNode>)>,
    cancel_btn: Query<&Interaction, (Changed<Interaction>, With<CancelSetupButtonNode>)>,
    play_btn: Query<&Interaction, (Changed<Interaction>, With<PlayGameButtonNode>)>,
) {
    for interaction in green_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.course = "green".to_string();
            settings.is_input_focused = false;
        }
    }

    for interaction in blue_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.course = "blue".to_string();
            settings.is_input_focused = false;
        }
    }

    for interaction in std_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.mode = GameMode::Standard;
            settings.is_input_focused = false;
        }
    }

    for interaction in wager_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.mode = GameMode::WagerCards;
            settings.is_input_focused = false;
        }
    }

    for interaction in input_box.iter() {
        if *interaction == Interaction::Pressed {
            settings.is_input_focused = true;
        }
    }

    for interaction in cancel_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.is_input_focused = false;
            next_state.set(ClientScreenState::Landing);
        }
    }

    for interaction in play_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.is_input_focused = false;
            
            // Dispatch authoritative StartPractice action over client channel
            let name_h = heapless::String::try_from(settings.nickname.as_str()).unwrap_or_default();
            let course_h = heapless::String::try_from(settings.course.as_str()).unwrap_or_default();
            events.send(ClientActionRequest(ClientAction::StartPractice {
                nickname: name_h,
                course: course_h,
                is_wager_mode: settings.mode == GameMode::WagerCards,
            }));
            
            next_state.set(ClientScreenState::Gameplay);
        }
    }
}

pub fn handle_nickname_keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<GameSettings>,
) {
    if !settings.is_input_focused {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Backspace) {
        settings.nickname.pop();
        // Force mutation triggers change detection
        let n = settings.nickname.clone();
        settings.nickname = n;
        return;
    }

    for ev in char_evr.read() {
        let text = ev.char.as_str();
        for c in text.chars() {
            if !c.is_control() && settings.nickname.len() < 16 {
                settings.nickname.push(c);
                // Force mutation trigger
                let n = settings.nickname.clone();
                settings.nickname = n;
            }
        }
    }
}

pub fn update_setup_screen_ui(
    settings: Res<GameSettings>,
    mut green_query: Query<(&mut BackgroundColor, &mut BorderColor), (With<CourseGreenButtonNode>, Without<CourseBlueButtonNode>, Without<ModeStandardButtonNode>, Without<ModeWagerButtonNode>, Without<NicknameInputContainerNode>)>,
    mut blue_query: Query<(&mut BackgroundColor, &mut BorderColor), (With<CourseBlueButtonNode>, Without<CourseGreenButtonNode>, Without<ModeStandardButtonNode>, Without<ModeWagerButtonNode>, Without<NicknameInputContainerNode>)>,
    mut std_query: Query<(&mut BackgroundColor, &mut BorderColor), (With<ModeStandardButtonNode>, Without<CourseGreenButtonNode>, Without<CourseBlueButtonNode>, Without<ModeWagerButtonNode>, Without<NicknameInputContainerNode>)>,
    mut wager_query: Query<(&mut BackgroundColor, &mut BorderColor), (With<ModeWagerButtonNode>, Without<CourseGreenButtonNode>, Without<CourseBlueButtonNode>, Without<ModeStandardButtonNode>, Without<NicknameInputContainerNode>)>,
    mut input_query: Query<&mut BorderColor, (With<NicknameInputContainerNode>, Without<CourseGreenButtonNode>, Without<CourseBlueButtonNode>, Without<ModeStandardButtonNode>, Without<ModeWagerButtonNode>)>,
    mut radio_dot_query: Query<(&mut Style, &RadioDotNode)>,
    mut text_query: Query<&mut Text, With<NicknameTextNode>>,
) {
    if !settings.is_changed() {
        return;
    }

    // Active/Inactive selection colors
    let active_bg = Color::srgb(0.01, 0.30, 0.10);
    let active_border = Color::srgb(0.10, 0.80, 0.20);
    let inactive_bg = Color::srgb(0.04, 0.10, 0.06);
    let inactive_border = Color::srgb(0.15, 0.25, 0.20);

    // Course green button
    if let Ok((mut bg, mut border)) = green_query.get_single_mut() {
        if settings.course == "green" {
            *bg = active_bg.into();
            *border = active_border.into();
        } else {
            *bg = inactive_bg.into();
            *border = inactive_border.into();
        }
    }

    // Course blue button
    if let Ok((mut bg, mut border)) = blue_query.get_single_mut() {
        if settings.course == "blue" {
            *bg = active_bg.into();
            *border = active_border.into();
        } else {
            *bg = inactive_bg.into();
            *border = inactive_border.into();
        }
    }

    // Standard mode button
    if let Ok((mut bg, mut border)) = std_query.get_single_mut() {
        if settings.mode == GameMode::Standard {
            *bg = active_bg.into();
            *border = active_border.into();
        } else {
            *bg = inactive_bg.into();
            *border = inactive_border.into();
        }
    }

    // Wager mode button
    if let Ok((mut bg, mut border)) = wager_query.get_single_mut() {
        if settings.mode == GameMode::WagerCards {
            *bg = active_bg.into();
            *border = active_border.into();
        } else {
            *bg = inactive_bg.into();
            *border = inactive_border.into();
        }
    }

    // Nickname focus border highlighting
    if let Ok(mut border) = input_query.get_single_mut() {
        if settings.is_input_focused {
            *border = active_border.into();
        } else {
            *border = inactive_border.into();
        }
    }

    // Radio dots
    for (mut style, node) in radio_dot_query.iter_mut() {
        style.display = if node.mode == settings.mode {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Nickname text
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.value = if settings.nickname.is_empty() {
                // Blinking cursor or empty state placeholder
                if settings.is_input_focused { "|".to_string() } else { "Type nickname...".to_string() }
            } else if settings.is_input_focused {
                format!("{}|", settings.nickname)
            } else {
                settings.nickname.clone()
            };
        }
    }
}
