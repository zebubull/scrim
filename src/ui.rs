use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Frame, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{
    app::{App, LookupEntry, Selected},
    widgets::{
        info_bar::InfoBar, player_bar::PlayerBar, stat_block::StatBlock, tab_panel::TabPanel,
    },
};

fn get_popup_rect((width, height): (u16, u16), parent: Rect) -> Rect {
    let hpart = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ])
        .split(parent);

    let vpart = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(hpart[1]);

    vpart[1]
}

fn clear_rect(f: &mut Frame, rect: Rect) {
    let s = " ".repeat(rect.width as usize);
    let lines: Vec<Line> = (0..rect.height).map(|_| Line::from(s.clone())).collect();
    f.render_widget(Paragraph::new(lines), rect);
}

fn show_quit_popup(f: &mut Frame) {
    let chunk = get_popup_rect((25, 20), f.size());
    clear_rect(f, chunk);

    let text = Paragraph::new("y - yes (save)\nq - yes (don't save)\nn - no")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Really quit?")
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Black).bg(Color::Yellow));

    f.render_widget(text, chunk);
}

fn show_lookup(f: &mut Frame, app: &App) {
    let chunk = get_popup_rect((65, 55), f.size());
    clear_rect(f, chunk);

    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(chunk);

    let block = Block::default()
        .title("Spell Lookup")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .on_yellow()
        .black();
    f.render_widget(block, chunk);

    let lookup = app.current_lookup.as_ref().unwrap();

    match lookup {
        LookupEntry::Invalid(search) => {
            let p = Paragraph::new(format!("No entry found for '{}'", search))
                .black()
                .wrap(Wrap { trim: false });
            f.render_widget(
                p,
                Layout::default()
                    .margin(1)
                    .constraints(vec![Constraint::Percentage(100)])
                    .split(chunk)[0],
            );
        }
        LookupEntry::Spell(entry) => {
            let title = Paragraph::new(format!("{}", entry.spell))
                .black()
                .bold()
                .alignment(Alignment::Center);
            f.render_widget(title, vchunks[0]);
            let short = Paragraph::new(format!("{}", entry.description_short))
                .black()
                .bold()
                .alignment(Alignment::Left);
            f.render_widget(short, vchunks[1]);
            let desc = Paragraph::new(format!("{}", entry.description))
                .black()
                .bold()
                .alignment(Alignment::Left)
                .scroll((app.lookup_scroll, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(desc, vchunks[3]);
        }
    }
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

    if let Some(Selected::Quitting) = app.selected {
        show_quit_popup(f);
    }

    if let Some(Selected::SpellLookup) = app.selected {
        show_lookup(f, app);
    }
}
