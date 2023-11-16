use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Frame, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{
    app::{App, LookupResult, Selected},
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

/// Clear the given rectangle by writing a paragraph of spaces.
///
/// There is probably a better way to do this but I have not figured
/// it out yet so this will do for now.
fn clear_rect(f: &mut Frame, rect: Rect) {
    let s = " ".repeat(rect.width as usize);
    let lines: Vec<Line> = (0..rect.height).map(|_| Line::from(s.clone())).collect();
    f.render_widget(Paragraph::new(lines), rect);
}

/// Show the quit confirmation menu
///
/// This could probably be moved to its own widget but I haven't done that yet.
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

/// Show the reference lookup menu.
///
/// This could probably be moved to its own widget but I haven't done that yet.
fn show_lookup(f: &mut Frame, app: &mut App) {
    let chunk = get_popup_rect((65, 55), f.size());
    clear_rect(f, chunk);

    app.popup_height = chunk.height as u32 - 4;

    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(chunk);

    let block = Block::default()
        .title("Reference Lookup")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .on_yellow()
        .black();
    f.render_widget(block, chunk);

    let lookup = app.current_lookup.as_ref().unwrap();

    match lookup {
        LookupResult::Invalid(search) => {
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
        LookupResult::Success(entry) => {
            let title = Paragraph::new(format!("{}", entry.name))
                .black()
                .bold()
                .alignment(Alignment::Center);
            f.render_widget(title, vchunks[0]);
            let short = Paragraph::new(format!("{}", entry.description_short))
                .black()
                .alignment(Alignment::Left);
            f.render_widget(short, vchunks[1]);
            let desc = Paragraph::new(format!("{}", entry.description))
                .black()
                .alignment(Alignment::Left)
                .scroll((app.popup_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(desc, vchunks[3]);
        }
        LookupResult::Completion(entries) => {
            let title = Paragraph::new(format!("{} results found", entries.len()))
                .black()
                .bold()
                .alignment(Alignment::Center);
            f.render_widget(title, vchunks[0]);

            let mut lines: Vec<Line> = entries
                .iter()
                .skip(app.popup_scroll as usize)
                .take(app.popup_height as usize)
                .map(|e| Line::from(Span::from(&e.name).on_yellow()))
                .collect();

            let selected = match app.selected {
                Some(Selected::Completion(idx, _)) => idx as u32,
                _ => 0,
            };

            lines[(selected - app.popup_scroll) as usize].spans[0]
                .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

            let options = Paragraph::new(lines)
                .black()
                .alignment(Alignment::Center)
                .scroll((app.popup_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(options, vchunks[3]);
        }
    }
}

/// Render all ui widgets using the data located in `app`.
pub fn render(app: &mut App, f: &mut Frame) {
    // Create layouts
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    let stat_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(9), Constraint::Min(1)])
        .split(main_chunks[1]);

    let stat_vertical_bound = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(19), Constraint::Min(1)])
        .split(stat_split[0]);

    let info_tab_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(1)])
        .split(stat_split[1]);

    // Render player bar
    let top_bar = PlayerBar::new(&app.player).editing(app.editing).highlight(
        if let Some(Selected::TopBarItem(i)) = app.selected {
            Some(i as u8)
        } else {
            None
        },
    );
    f.render_widget(top_bar, main_chunks[0]);

    // Render stat block
    let stat_block = StatBlock::new(&app.player.stats)
        .editing(app.editing)
        .highlight(if let Some(Selected::StatItem(i)) = app.selected {
            Some(i as u8)
        } else {
            None
        });
    f.render_widget(stat_block, stat_vertical_bound[0]);

    // Render player info bar
    let info_block = InfoBar::new(&app.player).editing(app.editing).highlight(
        if let Some(Selected::InfoItem(i)) = app.selected {
            Some(i as u8)
        } else {
            None
        },
    );
    f.render_widget(info_block, info_tab_chunks[0]);

    // Render the tab panel
    let tab_block = TabPanel::new(&app.player, app.current_tab)
        .scroll(app.vscroll as u16)
        .editing(app.editing)
        .highlight(if let Some(Selected::TabItem(i)) = app.selected {
            Some(i as u16)
        } else {
            None
        });
    f.render_widget(tab_block, info_tab_chunks[1]);

    match app.selected {
        Some(Selected::Completion(_, _))
        | Some(Selected::ItemLookup(_))
        | Some(Selected::ClassLookup) => show_lookup(f, app),
        Some(Selected::Quitting) => show_quit_popup(f),
        _ => {}
    }
}
