use ratatui::{
    layout::Rect,
    prelude::{Constraint, Direction, Frame, Layout},
};

use crate::{
    app::{App, Selected},
    widgets::{
        info_bar::InfoBar, player_bar::PlayerBar, stat_block::StatBlock, tab_panel::TabPanel,
    },
};

fn get_popup_rect((width, height): (u16, u16), parent: Rect) -> Rect {
    let hpart = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(1),
            Constraint::Percentage(width),
            Constraint::Min(1),
        ])
        .split(parent);

    let vpart = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(1),
            Constraint::Percentage(height),
            Constraint::Min(1),
        ])
        .split(hpart[1]);

    vpart[1]
}

pub fn render(app: &mut App, f: &mut Frame) {
    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(9), Constraint::Min(1)])
        .split(vchunks[1]);

    let statchunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(19), Constraint::Min(1)])
        .split(hchunks[0]);

    let rchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
        .split(hchunks[1]);

    let top_bar = PlayerBar::new(&app.player).editing(app.editing).highlight(
        if let Some(Selected::TopBarItem(i)) = app.selected {
            Some(i as u8)
        } else {
            None
        },
    );
    f.render_widget(top_bar, vchunks[0]);

    let stat_block = StatBlock::new(&app.player.stats)
        .editing(app.editing)
        .highlight(if let Some(Selected::StatItem(i)) = app.selected {
            Some(i as u8)
        } else {
            None
        });
    f.render_widget(stat_block, statchunk[0]);

    let info_block = InfoBar::new(&app.player).editing(app.editing).highlight(
        if let Some(Selected::InfoItem(i)) = app.selected {
            Some(i as u8)
        } else {
            None
        },
    );
    f.render_widget(info_block, rchunks[0]);

    let tab_block = TabPanel::new(&app.player, app.current_tab)
        .scroll(app.vscroll)
        .editing(app.editing)
        .highlight(if let Some(Selected::TabItem(i)) = app.selected {
            Some(i as u16)
        } else {
            None
        });
    f.render_widget(tab_block, rchunks[1]);
}
