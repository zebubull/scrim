use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Frame, Layout},
    style::Style,
    widgets::{Block, Borders, Padding},
};

use crate::{
    core::{App, LookupResult, Selected},
    player::{class::Class, skills::SKILL_NAMES, spells::SpellSlots},
    widgets::{
        info_bar::InfoBar, player_bar::PlayerBar, simple_popup::SimplePopup, stat_block::StatBlock,
        tab_panel::TabPanel, vec_popup::VecPopup, PopupSize,
    },
};

/// Show the quit confirmation menu
fn show_quit_popup(app: &mut App, f: &mut Frame) {
    let data = [
        String::from("y - yes (save)"),
        String::from("s - yes (don't save)"),
        String::from("q/n - no"),
    ];

    let popup = VecPopup::new(&data[..], PopupSize::Absolute(24, 7))
        .bg(app.settings().popup_background.into())
        .fg(app.settings().popup_foreground.into())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Really Quit?")
                .title_alignment(Alignment::Center)
                .padding(Padding::uniform(1)),
        )
        .alignment(Alignment::Left);

    app.popup_scroll_mut().update_frame_height(7);
    f.render_widget(popup, f.size());
}

/// Show the reference lookup menu.
///
/// This could probably be moved to its own widget but I haven't done that yet.
fn show_lookup(f: &mut Frame, app: &mut App) {
    let lookup = app.current_lookup.as_ref().unwrap();

    // Store the frame height for use after the lookup reference is dropped
    let frame_height = match lookup {
        LookupResult::Invalid(search) => {
            let text = format!("No results found for '{}'", search);
            let popup = SimplePopup::new(&text, PopupSize::Percentage(65, 25))
                .bg(app.settings().popup_background.into())
                .fg(app.settings().popup_foreground.into())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Lookup Failed")
                        .title_alignment(Alignment::Center),
                );

            let frame_height = popup.rect(f.size()).height as u32 - 2; // Border
            f.render_widget(popup, f.size());
            frame_height
        }
        LookupResult::Success(entry) => {
            let entry = entry.as_ref();
            let text = format!("{}\n{}", entry.description_short, entry.description);
            let popup = SimplePopup::new(&text, PopupSize::Percentage(55, 65))
                .bg(app.settings().popup_background.into())
                .fg(app.settings().popup_foreground.into())
                .wrap()
                .scroll_to(app.popup_scroll().get_scroll())
                .block(
                    Block::default()
                        .title(entry.name.clone())
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL),
                );

            let frame_height = popup.rect(f.size()).height as u32 - 2; // Border
            f.render_widget(popup, f.size());
            frame_height
        }
        LookupResult::Completion(entries) => {
            let lines: Vec<String> = entries.iter().map(|e| e.name.to_owned()).collect();
            let popup = VecPopup::new(&lines, PopupSize::Percentage(55, 75))
                .bg(app.settings().popup_background.into())
                .fg(app.settings().popup_foreground.into())
                .scroll_to(app.popup_scroll().get_scroll())
                .highlight(
                    app.popup_scroll().get_line(),
                    app.settings().popup_foreground.into(),
                )
                .block(
                    Block::default()
                        .title(format!("{} results found", entries.len()))
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .padding(Padding::vertical(1)),
                );

            let frame_height = popup.rect(f.size()).height as u32 - 4; // Border + padding
            f.render_widget(popup, f.size());
            frame_height
        }
        LookupResult::Files(files) => {
            let popup = VecPopup::new(files, PopupSize::Percentage(55, 75))
                .bg(app.settings().popup_background.into())
                .fg(app.settings().popup_foreground.into())
                .highlight(
                    app.popup_scroll().get_line(),
                    app.settings().popup_foreground.into(),
                )
                .scroll_to(app.popup_scroll().get_scroll())
                .block(
                    Block::default()
                        .title("Load Player")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .padding(Padding::vertical(1)),
                );

            let frame_height = popup.rect(f.size()).height as u32 - 4; // Border + padding
            f.render_widget(popup, f.size());
            frame_height
        }
    };

    app.popup_scroll_mut().update_frame_height(frame_height);
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
fn show_spell_slots(app: &mut App, f: &mut Frame) {
    let t = &app.player.spell_slots;
    let r = &app.player.spell_slots_remaining;

    let lines = if let Class::Warlock = app.player.class {
        let level = SpellSlots::warlock_slot_level(app.player.level);
        vec![format!(
            "{}{}: {} / {}",
            level,
            ordinal(level),
            t.warlock,
            r.warlock
        )]
    } else {
        (0..9)
            .map(|i| format!("{}{}: {} / {}", i + 1, ordinal(i as u32 + 1), r[i], t[i]))
            .collect()
    };

    let popup = VecPopup::new(&lines, PopupSize::Absolute(13, 11))
        .bg(app.settings().popup_background.into())
        .fg(app.settings().popup_foreground.into())
        .highlight(
            app.popup_scroll().get_line(),
            app.settings().popup_foreground.into(),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Spell Slots")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        );

    app.popup_scroll_mut().update_frame_height(11);

    f.render_widget(popup, f.size());
}

/// display the player funds popup
fn show_funds(app: &mut App, f: &mut Frame) {
    const LABELS: [&str; 4] = ["PP", "GP", "SP", "CP"];
    let lines: Vec<String> = (0..4)
        .map(|i| {
            let fundage = app.player.funds.nth(i);
            format!("{}: {}", LABELS[i as usize], fundage)
        })
        .collect();

    let popup = VecPopup::new(&lines, PopupSize::Absolute(12, 8))
        .bg(app.settings().popup_background.into())
        .fg(app.settings().popup_foreground.into())
        .highlight(
            app.popup_scroll().get_line(),
            app.settings().popup_foreground.into(),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Funds")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .padding(Padding::vertical(1)),
        );

    app.popup_scroll_mut().update_frame_height(6);
    f.render_widget(popup, f.size());
}

/// display the free lookup prompt menu
fn show_free_lookup_prompt(app: &mut App, f: &mut Frame) {
    let popup = SimplePopup::new(
        &app.lookup_buffer,
        PopupSize::Absolute(f.size().width - 10, 3),
    )
    .bg(app.settings().popup_background.into())
    .fg(app.settings().popup_foreground.into())
    .block(
        Block::default()
            .title("Lookup")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL),
    );

    f.render_widget(popup, f.size());
}

/// display the proficiency popup menu
fn show_proficiencies(app: &mut App, f: &mut Frame) {
    let lines: Vec<String> = app
        .player
        .get_skills()
        .iter()
        .enumerate()
        .map(|(i, skill)| format!("{}: {:+}", SKILL_NAMES[i], skill))
        .collect();

    let popup = VecPopup::new(&lines, PopupSize::Percentage(35, 55))
        .bg(app.settings().popup_background.into())
        .fg(app.settings().popup_foreground.into())
        .scroll_to(app.popup_scroll().get_scroll())
        .highlight(
            app.popup_scroll().get_line(),
            app.settings().popup_foreground.into(),
        )
        .block(
            Block::default()
                .title("Proficiencies")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        );

    app.popup_scroll_mut()
        .update_frame_height(popup.rect(f.size()).height as u32 - 2);
    f.render_widget(popup, f.size());
}

fn show_error_popup(app: &mut App, f: &mut Frame) {
    let popup = SimplePopup::new(
        app.error
            .as_ref()
            .expect("cannot show error popup with no error message"),
        PopupSize::Percentage(75, 75),
    )
    .bg(app.settings().popup_background.into())
    .fg(app.settings().popup_foreground.into())
    .scroll_to(app.popup_scroll().get_line())
    .wrap()
    .block(
        Block::default()
            .title("Error")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL),
    );

    f.render_widget(popup, f.size())
}

/// Get the player bar, info bar, and stat/tab chunk rects.
fn main_layout(parent: Rect) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(parent);

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
    let mut player_bar = PlayerBar::new(&app.player)
        .fg(app.settings().foreground.into())
        .bg(app.settings().background.into());
    if let Some(Selected::TopBarItem) = app.selected {
        player_bar = player_bar.highlight(
            app.index as u8,
            if app.editing {
                app.settings().highlight.into()
            } else {
                app.settings().foreground.into()
            },
        )
    }
    f.render_widget(player_bar, player_rect);

    // Render stat block
    let mut stat_block = StatBlock::new(&app.player.stats)
        .fg(app.settings().foreground.into())
        .bg(app.settings().background.into());
    if let Some(Selected::StatItem) = app.selected {
        stat_block = stat_block.highlight(
            app.index as u8,
            if app.editing {
                app.settings().highlight.into()
            } else {
                app.settings().foreground.into()
            },
        )
    }
    f.render_widget(stat_block, stat_rect);

    // Render player info bar
    let mut info_bar = InfoBar::new(&app.player)
        .fg(app.settings().foreground.into())
        .bg(app.settings().background.into());
    if let Some(Selected::InfoItem) = app.selected {
        info_bar = info_bar.highlight(
            app.index as u8,
            if app.editing {
                app.settings().highlight.into()
            } else {
                app.settings().foreground.into()
            },
        )
    }
    f.render_widget(info_bar, info_rect);

    // Render the tab panel
    let mut tab_block = TabPanel::new(&app.player, app.current_tab)
        .fg(app.settings().foreground.into())
        .bg(app.settings().background.into())
        .select(app.settings().tab_select.into())
        .scroll(app.tab_scroll().get_scroll() as u16);
    if let Some(Selected::TabItem) = app.selected {
        tab_block = tab_block.highlight(
            (app.tab_scroll().get_line() as u16, app.index as u16),
            if app.editing {
                app.settings().highlight.into()
            } else {
                app.settings().foreground.into()
            },
        )
    }
    f.render_widget(tab_block, tab_rect);
}

fn draw_background_color(app: &mut App, f: &mut Frame) {
    let b = Block::default().style(Style::default().bg(app.settings().background.into()));
    f.render_widget(b, f.size())
}

/// Render all ui widgets using the data located in `app`.
pub fn render(app: &mut App, f: &mut Frame) {
    draw_background_color(app, f);
    draw_static_widgets(app, f);

    match app.selected {
        Some(
            Selected::Completion(_)
            | Selected::ItemLookup(_)
            | Selected::ClassLookup
            | Selected::FreeLookupSelect
            | Selected::Load,
        ) => show_lookup(f, app),
        Some(Selected::Quitting) => show_quit_popup(app, f),
        Some(Selected::SpellSlots) => show_spell_slots(app, f),
        Some(Selected::Funds) => show_funds(app, f),
        Some(Selected::FreeLookup) => show_free_lookup_prompt(app, f),
        Some(Selected::Proficiency) => show_proficiencies(app, f),
        Some(Selected::Error) => show_error_popup(app, f),
        _ => {}
    }
}
