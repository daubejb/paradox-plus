use bevy::prelude::*;
use protocol::terrain::presets::get_course_preset;
use crate::replication::ClientGameState;
use crate::ui::components::{
    TopHudNode, LeaderboardTickerContainerNode, BoardContainerNode,
    BottomBarNode, WagerPanelNode, MatchCompletedScreenNode,
    ScorecardCellTextNode, ClientScorecards, GameSettings
};

pub fn toggle_match_completed_ui_system(
    game_state: Res<State<ClientGameState>>,
    mut hud_query: Query<&mut Style, (With<TopHudNode>, Without<MatchCompletedScreenNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>)>,
    mut ticker_query: Query<&mut Style, (With<LeaderboardTickerContainerNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>)>,
    mut board_query: Query<&mut Style, (With<BoardContainerNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>)>,
    mut bottom_query: Query<&mut Style, (With<BottomBarNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<WagerPanelNode>)>,
    mut wager_query: Query<&mut Style, (With<WagerPanelNode>, Without<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>)>,
    mut summary_query: Query<&mut Style, (With<MatchCompletedScreenNode>, Without<TopHudNode>, Without<LeaderboardTickerContainerNode>, Without<BoardContainerNode>, Without<BottomBarNode>, Without<WagerPanelNode>)>,
) {
    if game_state.is_changed() {
        let is_completed = *game_state.get() == ClientGameState::MatchCompleted;

        if is_completed {
            for mut style in hud_query.iter_mut() { style.display = Display::None; }
            for mut style in ticker_query.iter_mut() { style.display = Display::None; }
            for mut style in board_query.iter_mut() { style.display = Display::None; }
            for mut style in bottom_query.iter_mut() { style.display = Display::None; }
            for mut style in wager_query.iter_mut() { style.display = Display::None; }
            for mut style in summary_query.iter_mut() { style.display = Display::Flex; }
        } else {
            for mut style in summary_query.iter_mut() { style.display = Display::None; }
            for mut style in hud_query.iter_mut() { style.display = Display::Flex; }
            for mut style in ticker_query.iter_mut() { style.display = Display::Flex; }
            for mut style in board_query.iter_mut() { style.display = Display::Flex; }
        }
    }
}

pub fn render_scorecard_system(
    mut scorecard_cell_query: Query<(&mut Text, &ScorecardCellTextNode)>,
    scorecards: Res<ClientScorecards>,
    settings: Res<GameSettings>,
) {
    if scorecards.is_changed() {
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

        // Second pass: accumulate scores from history
        for hole in 1..=18 {
            if let Some(&strokes) = scorecard.strokes_per_hole.get((hole as usize).saturating_sub(1)) {
                if hole <= 9 {
                    out_score = out_score.saturating_add(strokes);
                } else {
                    in_score = in_score.saturating_add(strokes);
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
                let total_par = out_par.saturating_add(in_par);
                let total_score = out_score.saturating_add(in_score);
                let diff = total_score as i32 - total_par as i32;
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
