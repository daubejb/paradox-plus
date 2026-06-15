use bevy::prelude::*;
use protocol::terrain::presets::get_course_preset;
use crate::replication::ClientGameState;
use crate::ui::components::{
    TopHudNode, LeaderboardTickerContainerNode, BoardContainerNode,
    BottomBarNode, WagerPanelNode, MatchCompletedScreenNode,
    ScorecardCellTextNode, ClientScorecards, GameSettings,
    ShowScorecard, CloseScorecardButtonNode, PlayAgainButtonNode,
    MainMenuButtonNode, GameMode
};

pub fn toggle_match_completed_ui_system(
    game_state: Res<State<ClientGameState>>,
    show_scorecard: Res<ShowScorecard>,
    settings: Res<GameSettings>,
    mut hud_query: Query<&mut Style, (With<TopHudNode>, Without<MatchCompletedScreenNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut ticker_query: Query<&mut Style, (With<LeaderboardTickerContainerNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut board_query: Query<&mut Style, (With<BoardContainerNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut bottom_query: Query<&mut Style, (With<BottomBarNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut wager_query: Query<&mut Style, (With<WagerPanelNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut summary_query: Query<&mut Style, (With<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut close_btn_query: Query<&mut Style, (With<CloseScorecardButtonNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<PlayAgainButtonNode>, Without<MainMenuButtonNode>)>,
    mut play_again_query: Query<&mut Style, (With<PlayAgainButtonNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<MainMenuButtonNode>)>,
    mut main_menu_query: Query<&mut Style, (With<MainMenuButtonNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>, Without<CloseScorecardButtonNode>, Without<PlayAgainButtonNode>)>,
) {
    if game_state.is_changed() || show_scorecard.is_changed() {
        let is_completed = *game_state.get() == ClientGameState::MatchCompleted;
        let visible = show_scorecard.0 || is_completed;

        if visible {
            for mut style in hud_query.iter_mut() { style.display = Display::None; }
            for mut style in ticker_query.iter_mut() { style.display = Display::None; }
            for mut style in board_query.iter_mut() { style.display = Display::None; }
            for mut style in bottom_query.iter_mut() { style.display = Display::None; }
            for mut style in wager_query.iter_mut() { style.display = Display::None; }
            for mut style in summary_query.iter_mut() { style.display = Display::Flex; }

            if is_completed {
                for mut style in close_btn_query.iter_mut() { style.display = Display::None; }
                for mut style in play_again_query.iter_mut() { style.display = Display::Flex; }
                for mut style in main_menu_query.iter_mut() { style.display = Display::Flex; }
            } else {
                for mut style in close_btn_query.iter_mut() { style.display = Display::Flex; }
                for mut style in play_again_query.iter_mut() { style.display = Display::None; }
                for mut style in main_menu_query.iter_mut() { style.display = Display::None; }
            }
        } else {
            for mut style in summary_query.iter_mut() { style.display = Display::None; }
            for mut style in hud_query.iter_mut() { style.display = Display::Flex; }
            for mut style in ticker_query.iter_mut() { style.display = Display::Flex; }
            for mut style in board_query.iter_mut() { style.display = Display::Flex; }
            for mut style in bottom_query.iter_mut() { style.display = Display::Flex; }
            
            let wager_display = match settings.mode {
                GameMode::Standard => Display::None,
                GameMode::WagerCards => Display::Flex,
            };
            for mut style in wager_query.iter_mut() { style.display = wager_display; }

            for mut style in close_btn_query.iter_mut() { style.display = Display::None; }
            for mut style in play_again_query.iter_mut() { style.display = Display::None; }
            for mut style in main_menu_query.iter_mut() { style.display = Display::None; }
        }
    }
}

pub fn render_scorecard_system(
    mut scorecard_cell_query: Query<(&mut Text, &ScorecardCellTextNode)>,
    mut title_query: Query<&mut Text, (With<crate::ui::components::ScorecardTitleTextNode>, Without<ScorecardCellTextNode>)>,
    scorecards: Res<ClientScorecards>,
    settings: Res<GameSettings>,
    game_state: Res<State<ClientGameState>>,
) {
    if scorecards.is_changed() || game_state.is_changed() {
        let is_completed = *game_state.get() == ClientGameState::MatchCompleted;
        if let Ok(mut text) = title_query.get_single_mut() {
            if is_completed {
                text.sections[0].value = "MATCH COMPLETED".to_string();
            } else {
                let course_name = settings.course.to_uppercase();
                text.sections[0].value = format!("{} COURSE", course_name);
            }
        }

        let Some(scorecard) = scorecards.0.first() else { return; };

        let mut out_par = 0u16;
        let mut in_par = 0u16;
        let mut out_score = 0u16;
        let mut in_score = 0u16;

        // First pass: accumulate pars
        for hole in 1..=18 {
            if let Some(preset) = get_course_preset(&settings.course, hole) {
                let par = preset.par as u16;
                if hole <= 9 {
                    out_par = out_par.saturating_add(par);
                } else {
                    in_par = in_par.saturating_add(par);
                }
            }
        }

        // Second pass: accumulate scores from history and par of completed holes
        let mut completed_par = 0u16;
        for hole in 1..=18 {
            if let Some(&strokes) = scorecard.strokes_per_hole.get((hole as usize).saturating_sub(1)) {
                if hole <= 9 {
                    out_score = out_score.saturating_add(strokes);
                } else {
                    in_score = in_score.saturating_add(strokes);
                }
                if let Some(preset) = get_course_preset(&settings.course, hole) {
                    completed_par = completed_par.saturating_add(preset.par as u16);
                }
            }
        }

        for (mut text, cell) in scorecard_cell_query.iter_mut() {
            let hole = cell.hole_num;
            if cell.is_par {
                if hole >= 1 && hole <= 18 {
                    if let Some(preset) = get_course_preset(&settings.course, hole) {
                        text.sections[0].value = preset.par.to_string();
                    }
                } else if hole == 20 {
                    text.sections[0].value = out_par.to_string();
                } else if hole == 21 {
                    text.sections[0].value = in_par.to_string();
                }
            } else if cell.is_score {
                if hole >= 1 && hole <= 18 {
                    if let Some(&strokes) = scorecard.strokes_per_hole.get((hole as usize).saturating_sub(1)) {
                        text.sections[0].value = strokes.to_string();
                        
                        if let Some(preset) = get_course_preset(&settings.course, hole) {
                            let par = preset.par as i32;
                            let rel = strokes as i32 - par;
                            if rel < 0 {
                                text.sections[0].style.color = Color::srgb(0.95, 0.8, 0.2); // Under par
                            } else if rel == 0 {
                                text.sections[0].style.color = Color::WHITE; // Par
                            } else {
                                text.sections[0].style.color = Color::srgb(1.0, 0.4, 0.2); // Over par
                            }
                        }
                    } else {
                        text.sections[0].value = "-".to_string();
                        text.sections[0].style.color = Color::WHITE;
                    }
                } else if hole == 20 {
                    text.sections[0].value = out_score.to_string();
                } else if hole == 21 {
                    text.sections[0].value = in_score.to_string();
                }
            } else if hole == 22 {
                let total_score = out_score.saturating_add(in_score);
                let diff = total_score as i32 - completed_par as i32;
                let diff_str = if diff < 0 {
                    format!(" ({} Under Par)", diff.abs())
                } else if diff == 0 {
                    " (Even Par)".to_string()
                } else {
                    format!(" ({} Over Par)", diff)
                };
                text.sections[0].value = format!("TOTAL STROKES: {}{}", total_score, diff_str);
            }
        }
    }
}
