use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Frame, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use crate::{
    core::{App, LookupResult, Selected},
    player::{class::Class, skills::SKILL_NAMES, spells::SpellSlots},
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
            let p = Paragraph::new(format!("No entry found for '{search}'"))
                .black()
                .wrap(Wrap { trim: false });
            f.render_widget(
                p,
                Layout::default()
                    .margin(1)
                    .constraints(vec![Constraint::Percentage(100)])
                    .split(chunk)[0],
            );
            app.popup_scroll_mut().update_frame_height(chunk.height as u32 - 2);
        }
        LookupResult::Success(entry) => {
            let entry = entry.clone();
            let render_short = !entry.description_short.is_empty();
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

            app.popup_scroll_mut().update_frame_height(if render_short {
                u32::from(chunk.height) - 6
            } else {
                u32::from(chunk.height) - 4
            });

            let title = Paragraph::new(entry.name.to_string())
                .black()
                .bold()
                .alignment(Alignment::Center);
            f.render_widget(title, vchunks[0]);

            if render_short {
                let short = Paragraph::new(entry.description_short.to_string())
                    .black()
                    .alignment(Alignment::Left);
                f.render_widget(short, vchunks[1]);
            }

            let desc = Paragraph::new(entry.description.to_string())
                .black()
                .alignment(Alignment::Left)
                .scroll((app.popup_scroll().get_scroll() as u16, 0))
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

            let title = Paragraph::new(format!("{} results founds...", entries.len()))
                .black()
                .bold()
                .alignment(Alignment::Center);
            f.render_widget(title, vchunks[0]);

            let mut lines: Vec<Line> = entries
                .iter()
                .skip(app.popup_scroll().get_scroll() as usize)
                .take(vchunks[2].height as usize)
                .map(|e| Line::from(Span::from(&e.name).on_yellow()))
                .collect();

            let selected = app.popup_scroll().get_line();

            lines[selected.saturating_sub(app.popup_scroll().get_scroll()) as usize].spans[0]
                .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

            let options = Paragraph::new(lines)
                .black()
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            f.render_widget(options, vchunks[2]);

            app.popup_scroll_mut().update_frame_height(vchunks[2].height as u32);
        }
        LookupResult::Files(entries) => {
            let mut lines: Vec<Line> = entries
                .iter()
                .skip(app.popup_scroll().get_scroll() as usize)
                .take(chunk.height as usize - 4)
                .map(|e| Line::from(Span::from(e).on_yellow()))
                .collect();

            let layout = Layout::default()
                .constraints(vec![Constraint::Min(1)])
                .margin(1)
                .split(chunk);

            let selected = app.popup_scroll().get_line();

            lines[selected.saturating_sub(app.popup_scroll().get_scroll()) as usize].spans[0]
                .patch_style(Style::default().bg(Color::Black).fg(Color::Yellow));

            let options = Paragraph::new(lines)
                .black()
                .alignment(Alignment::Center)
                .scroll((app.popup_scroll().get_scroll() as u16, 0))
                .wrap(Wrap { trim: false });
            f.render_widget(options, layout[0]);

            app.popup_scroll_mut().update_frame_height(chunk.height as u32 - 4);
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

    app.popup_scroll_mut().update_frame_height(chunk.height as u32 - 2);

    let t = &app.player.spell_slots;
    let r = &app.player.spell_slots_remaining;

    let mut lines = if let Class::Warlock = app.player.class {
        let level = SpellSlots::warlock_slot_level(app.player.level);
        vec![Line::from(
            format!("{}{}: {} / {}", level, ordinal(level), t.warlock, r.warlock)
                .black()
                .on_yellow(),
        )]
    } else {
        (0..9)
            .map(|i| {
                let total = t[i];
                Line::from(
                    Span::from(format!(
                        "{}{}: {} / {}",
                        i + 1,
                        ordinal(i as u32 + 1),
                        r[i],
                        total
                    ))
                    .on_yellow()
                    .black(),
                )
            })
            .collect()
    };

    let selected = app.popup_scroll().get_line();

    lines[selected.saturating_sub(app.popup_scroll().get_scroll()) as usize].spans[0]
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

fn show_funds(f: &mut Frame, app: &mut App) {
    const LABELS: [&str; 4] = ["PP", "GP", "SP", "CP"];
    let chunk = get_popup_rect((35, 75), f.size());
    clear_rect(f, chunk);

    app.popup_scroll_mut().update_frame_height(chunk.height as u32 - 2);

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

    let selected = app.popup_scroll().get_line();

    lines[selected.saturating_sub(app.popup_scroll().get_scroll()) as usize].spans[0]
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

    app.popup_scroll_mut().update_frame_height(chunk.height as u32 - 2);

    let mut lines: Vec<Line> = app
        .player
        .get_skills()
        .iter()
        .enumerate()
        .skip(app.popup_scroll().get_scroll() as usize)
        .take(chunk.height as usize - 2)
        .map(|(i, skill)| {
            Line::from(
                Span::from(format!("{}: {:+}", SKILL_NAMES[i], skill))
                    .on_yellow()
                    .black(),
            )
        })
        .collect();

    let selected = app.popup_scroll().get_line();

    lines[selected.saturating_sub(app.popup_scroll().get_scroll()) as usize]
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

/// Get the player bar, info bar, and stat/tab chunk rects.
fn main_layout(parent: Rect) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ]).split(parent);

    (chunks[0], chunks[1], chunks[2])
}

/// Get the stat bar and tab chunk rects
fn stat_tab_layout(parent: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(9), Constraint::Min(1)])
        .split(parent);

    let stat_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(19), Constraint::Min(1)])
        .split(chunks[0]);

    (stat_chunk[0], chunks[1])
}

/// Get the player bar, info bar, stat block, and tab pane rects.
fn layouts(viewport: Rect) -> (Rect, Rect, Rect, Rect) {
    let (player, info, stat_tab) = main_layout(viewport);
    let (stat, tab) = stat_tab_layout(stat_tab);
    (player, info, stat, tab)
}

/// Get the height of the tab pane given the specified viewport height
pub fn tab_pane_height(viewport_height: u16) -> u16 {
    let (_, _, tab_chunk) = main_layout(Rect::new(0, 0, 1, viewport_height));
    // Border + bottom bar = 3 margin
    tab_chunk.height - 3
}

/// Draw all UI widgets that are always present.
fn draw_static_widgets(app: &mut App, f: &mut Frame) {
    // Create layouts
    let (player_rect, info_rect, stat_rect, tab_rect) = layouts(f.size());

    // Render player bar
    let player_bar = PlayerBar::new(&app.player).editing(app.editing).highlight(
        if let Some(Selected::TopBarItem) = app.selected {
            Some(app.index as u8)
        } else {
            None
        },
    );
    f.render_widget(player_bar, player_rect);

    // Render stat block
    let stat_block = StatBlock::new(&app.player.stats)
        .editing(app.editing)
        .highlight(if let Some(Selected::StatItem) = app.selected {
            Some(app.index as u8)
        } else {
            None
        });
    f.render_widget(stat_block, stat_rect);

    // Render player info bar
    let info_bar = InfoBar::new(&app.player).editing(app.editing).highlight(
        if let Some(Selected::InfoItem) = app.selected {
            Some(app.index as u8)
        } else {
            None
        },
    );
    f.render_widget(info_bar, info_rect);

    // Render the tab panel
    let tab_block = TabPanel::new(&app.player, app.current_tab)
        .scroll(app.tab_scroll().get_scroll() as u16)
        .editing(app.editing)
        .highlight(if let Some(Selected::TabItem) = app.selected {
            Some(app.tab_scroll().get_line() as u16)
        } else {
            None
        });
    f.render_widget(tab_block, tab_rect);

}

/// Render all ui widgets using the data located in `app`.
pub fn render(app: &mut App, f: &mut Frame) {
    draw_static_widgets(app, f);

    match app.selected {
        Some(
            Selected::Completion(_)
            | Selected::ItemLookup(_)
            | Selected::ClassLookup
            | Selected::FreeLookupSelect
            | Selected::Load,
        ) => show_lookup(f, app),
        Some(Selected::Quitting) => show_quit_popup(f),
        Some(Selected::SpellSlots) => show_spell_slots(f, app),
        Some(Selected::Funds) => show_funds(f, app),
        Some(Selected::FreeLookup) => show_free_lookup(f, app),
        Some(Selected::Proficiency) => show_proficiencies(f, app),
        _ => {}
    }
}
