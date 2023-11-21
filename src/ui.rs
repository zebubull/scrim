use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Frame, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::{
    app::{App, LookupResult, Selected},
    player::{Class, SpellSlots, SKILL_NAMES},
    widgets::{
        info_bar::InfoBar, player_bar::PlayerBar, stat_block::StatBlock, tab_panel::TabPanel,
    },
};

/// Get a rectangle for a popup with the given width and height percentages
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

    let text = Paragraph::new("y - yes (save)\ns - yes (don't save)\nq/n - no")
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
    let chunk = get_popup_rect((55, 65), f.size());
    clear_rect(f, chunk);

    app.popup_height = chunk.height as u32 - 4;

    let block = Block::default()
        .title(if let Some(LookupResult::Files(_)) = app.current_lookup {
            "File Select"
        } else {
            "Reference Lookup"
        })
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
            let render_short = entry.description_short.len() > 0;
            let vchunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(if render_short {
                    vec![
                        Constraint::Length(1),
                        Constraint::Length(2),
                        Constraint::Length(1),
                        Constraint::Min(1),
                    ]
                } else {
                    vec![
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Min(1),
                    ]
                })
                .split(chunk);
            let title = Paragraph::new(format!("{}", entry.name))
                .black()
                .bold()
                .alignment(Alignment::Center);
            f.render_widget(title, vchunks[0]);
            if render_short {
                let short = Paragraph::new(format!("{}", entry.description_short))
                    .black()
                    .alignment(Alignment::Left);
                f.render_widget(short, vchunks[1]);
            }
            let desc = Paragraph::new(format!("{}", entry.description))
                .black()
                .alignment(Alignment::Left)
                .scroll((app.popup_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(desc, vchunks[if render_short { 3 } else { 2 }]);
        }
        LookupResult::Completion(entries) => {
            let vchunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                ])
                .split(chunk);
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
                Some(Selected::Completion(idx, _)) => idx,
                Some(Selected::FreeLookupSelect(idx)) => idx,
                _ => 0,
            } as u32;

            lines[selected.saturating_sub(app.popup_scroll) as usize].spans[0]
                .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

            let options = Paragraph::new(lines)
                .black()
                .alignment(Alignment::Center)
                .scroll((app.popup_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(options, vchunks[2]);
        }
        LookupResult::Files(entries) => {
            let mut lines: Vec<Line> = entries
                .iter()
                .skip(app.popup_scroll as usize)
                .take(app.popup_height as usize)
                .map(|e| Line::from(Span::from(e).on_yellow()))
                .collect();

            let layout = Layout::default()
                .constraints(vec![Constraint::Min(1)])
                .margin(1)
                .split(chunk);

            let selected = match app.selected {
                Some(Selected::Load(idx)) => idx,
                _ => 0,
            } as u32;

            lines[selected.saturating_sub(app.popup_scroll) as usize].spans[0]
                .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

            let options = Paragraph::new(lines)
                .black()
                .alignment(Alignment::Center)
                .scroll((app.popup_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(options, layout[0]);
        }
    }
}

/// Get the ordinal suffix corresponding to the given digit
fn ordinal(n: u32) -> &'static str {
    match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

/// Show the player's spell slot menu
///
/// This could probably be moved to its own widget but I'm lazy :p
fn show_spell_slots(f: &mut Frame, app: &mut App) {
    let chunk = get_popup_rect((35, 75), f.size());
    clear_rect(f, chunk);

    let t = &app.player.spell_slots;
    let r = &app.player.spell_slots_remaining;

    let mut lines = if let Class::Warlock = app.player.class {
        let level = SpellSlots::warlock_slot_level(app.player.level);
        vec![Line::from(Span::from(
            format!("{}{}: {} / {}", level, ordinal(level), t.warlock, r.warlock)
                .black()
                .on_yellow(),
        ))]
    } else {
        (0..9)
            .map(|i| {
                let total = t.nth(i, &app.player.class);
                Line::from(
                    Span::from(format!(
                        "{}{}: {} / {}",
                        i + 1,
                        ordinal(i + 1),
                        r.nth(i, &app.player.class),
                        total
                    ))
                    .on_yellow()
                    .black(),
                )
            })
            .collect()
    };

    let selected = match app.selected {
        Some(Selected::SpellSlots(idx)) => idx as u32,
        _ => 0,
    };

    lines[selected.saturating_sub(app.popup_scroll) as usize].spans[0]
        .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

    let offset = lines.len() / 2;
    let p = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .title("Spell Slots")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .on_yellow()
            .black()
            .padding(Padding::new(0, 0, chunk.height / 2 - offset as u16, 0)),
    );

    f.render_widget(p, chunk);
}

fn show_funds(f: &mut Frame, app: &App) {
    const LABELS: [&str; 4] = ["PP", "GP", "SP", "CP"];
    let chunk = get_popup_rect((35, 75), f.size());
    clear_rect(f, chunk);

    let mut lines: Vec<Line> = (0..4)
        .map(|i| {
            let fundage = app.player.funds.nth(i);
            Line::from(
                Span::from(format!("{}: {}", LABELS[i as usize], fundage,))
                    .on_yellow()
                    .black(),
            )
        })
        .collect();

    let selected = match app.selected {
        Some(Selected::Funds(idx)) => idx as u32,
        _ => 0,
    };

    lines[selected.saturating_sub(app.popup_scroll) as usize].spans[0]
        .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

    let offset = lines.len() / 2;
    let p = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .title("Funds")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .on_yellow()
            .black()
            .padding(Padding::new(0, 0, chunk.height / 2 - offset as u16, 0)),
    );

    f.render_widget(p, chunk);
}

fn show_free_lookup(f: &mut Frame, app: &mut App) {
    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(10),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(vchunks[1]);

    clear_rect(f, hchunks[1]);

    let p = Paragraph::new(app.lookup_buffer.clone())
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Lookup")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .on_yellow()
                .black(),
        );

    f.render_widget(p, hchunks[1]);
}

fn show_proficiencies(f: &mut Frame, app: &mut App) {
    let chunk = get_popup_rect((35, 55), f.size());
    clear_rect(f, chunk);

    app.popup_height = chunk.height as u32 - 3;

    let mut lines: Vec<Line> = app
        .player
        .get_skills()
        .iter()
        .skip(app.popup_scroll as usize)
        .take(chunk.height as usize - 2)
        .enumerate()
        .map(|(i, skill)| {
            Line::from(
                Span::from(format!("{}: {:+}", SKILL_NAMES[i], skill))
                    .on_yellow()
                    .black(),
            )
        })
        .collect();

    let selected = match app.selected {
        Some(Selected::Proficiency(i)) => i,
        _ => unreachable!(),
    };

    lines[selected.saturating_sub(app.popup_scroll) as usize]
        .patch_style(Style::new().on_black().yellow());

    let p = Paragraph::new(lines).alignment(Alignment::Left).block(
        Block::default()
            .title("Proficiencies")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .on_yellow()
            .black(),
    );

    f.render_widget(p, chunk);
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
        | Some(Selected::ClassLookup)
        | Some(Selected::FreeLookupSelect(_))
        | Some(Selected::Load(_)) => show_lookup(f, app),
        Some(Selected::Quitting) => show_quit_popup(f),
        Some(Selected::SpellSlots(_)) => show_spell_slots(f, app),
        Some(Selected::Funds(_)) => show_funds(f, app),
        Some(Selected::FreeLookup) => show_free_lookup(f, app),
        Some(Selected::Proficiency(_)) => show_proficiencies(f, app),
        _ => {}
    }
}
