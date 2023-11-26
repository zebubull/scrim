use ratatui::{
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Frame, Layout},
    style::Color,
    widgets::{Block, Borders, Padding},
};

use crate::{
    core::{App, LookupResult, Selected},
    player::{class::Class, skills::SKILL_NAMES, spells::SpellSlots},
    widgets::{
        PopupSize,
        info_bar::InfoBar,
        player_bar::PlayerBar,
        stat_block::StatBlock,
        tab_panel::TabPanel,
        vec_popup::VecPopup, simple_popup::SimplePopup,
    },
};

/// Show the quit confirmation menu
fn show_quit_popup(app: &mut App, f: &mut Frame) {
    let data =  [
        String::from("y - yes (save)"),
        String::from("s - yes (don't save)"),
        String::from("q/n - no"),
    ];

    let popup = VecPopup::new(
        &data[..],
        PopupSize::Absolute(24, 7),
    )
    .fg(Color::Black)
    .bg(Color::Yellow)
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
                .bg(Color::Yellow)
                .fg(Color::Black)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Lookup Failed")
                    .title_alignment(Alignment::Center));

            let frame_height = popup.rect(f.size()).height as u32 - 2; // Border
            f.render_widget(popup, f.size());
            frame_height
        },
        LookupResult::Success(entry) => {
            let entry = entry.as_ref();
            let text = format!("{}\n{}", entry.description_short, entry.description);
            let popup = SimplePopup::new(&text, PopupSize::Percentage(55, 65))
                .fg(Color::Black)
                .bg(Color::Yellow)
                .wrap()
                .scroll_to(app.popup_scroll().get_scroll())
                .block(Block::default()
                    .title(entry.name.clone())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL));

            let frame_height = popup.rect(f.size()).height as u32 - 2; // Border
            f.render_widget(popup, f.size());
            frame_height
        }
        LookupResult::Completion(entries) => {
            let lines: Vec<String> = entries.iter().map(|e| e.name.to_owned()).collect();
            let popup = VecPopup::new(&lines, PopupSize::Percentage(55, 75))
                .bg(Color::Yellow)
                .fg(Color::Black)
                .scroll_to(app.popup_scroll().get_scroll())
                .highlight(app.popup_scroll().get_line(), Color::Black)
                .block(Block::default()
                    .title(format!("{} results found", entries.len()))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .padding(Padding::vertical(1)));

            let frame_height = popup.rect(f.size()).height as u32 - 4; // Border + padding
            f.render_widget(popup, f.size());
            frame_height
        }
        LookupResult::Files(files) => {
            let popup = VecPopup::new(files, PopupSize::Percentage(55, 75))
                .bg(Color::Yellow)
                .fg(Color::Black)
                .highlight(app.popup_scroll().get_line(), Color::Black)
                .scroll_to(app.popup_scroll().get_scroll())
                .block(Block::default()
                    .title("Load Player")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .padding(Padding::vertical(1)));

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
        vec![
            format!("{}{}: {} / {}", level, ordinal(level), t.warlock, r.warlock)
        ]
    } else {
        (0..9)
            .map(|i| {
                format!(
                    "{}{}: {} / {}",
                    i + 1,
                    ordinal(i as u32 + 1),
                    r[i],
                    t[i]
                )
            })
            .collect()
    };

    let popup = VecPopup::new(&lines, PopupSize::Absolute(13, 11))
        .fg(Color::Black)
        .bg(Color::Yellow)
        .highlight(app.popup_scroll().get_line(), Color::Black)
        .alignment(Alignment::Center)
        .block(Block::default()
            .title("Spell Slots")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL));

    app.popup_scroll_mut().update_frame_height(11);

    f.render_widget(popup, f.size());
}

fn show_funds(app: &mut App, f: &mut Frame) {
    const LABELS: [&str; 4] = ["PP", "GP", "SP", "CP"];
    let lines: Vec<String> = (0..4)
        .map(|i| {
            let fundage = app.player.funds.nth(i);
                format!("{}: {}", LABELS[i as usize], fundage)
        })
        .collect();


    let popup = VecPopup::new(&lines, PopupSize::Absolute(12, 8))
        .fg(Color::Black)
        .bg(Color::Yellow)
        .highlight(app.popup_scroll().get_line(), Color::Black)
        .alignment(Alignment::Center)
        .block(Block::default()
            .title("Funds")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::vertical(1)));

    app.popup_scroll_mut().update_frame_height(6);
    f.render_widget(popup, f.size());
}

fn show_free_lookup_prompt(app: &mut App, f: &mut Frame) {
    let popup = SimplePopup::new(&app.lookup_buffer, PopupSize::Absolute(f.size().width-10, 3))
        .fg(Color::Black)
        .bg(Color::Yellow)
        .block(Block::default()
            .title("Lookup")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL));

    f.render_widget(popup, f.size());
}

fn show_proficiencies(app: &mut App, f: &mut Frame) {
    let lines: Vec<String> = app
        .player
        .get_skills()
        .iter()
        .enumerate()
        .map(|(i, skill)| {
                format!("{}: {:+}", SKILL_NAMES[i], skill)
        })
        .collect();
    
    let popup = VecPopup::new(&lines, PopupSize::Percentage(35, 55))
        .bg(Color::Yellow)
        .fg(Color::Black)
        .scroll_to(app.popup_scroll().get_scroll())
        .highlight(app.popup_scroll().get_line(), Color::Black)
        .block(Block::default()
            .title("Proficiencies")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL));

    app.popup_scroll_mut().update_frame_height(popup.rect(f.size()).height as u32 - 2);
    f.render_widget(popup, f.size());
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
        Some(Selected::Quitting) => show_quit_popup(app, f),
        Some(Selected::SpellSlots) => show_spell_slots(app, f),
        Some(Selected::Funds) => show_funds(app, f),
        Some(Selected::FreeLookup) => show_free_lookup_prompt(app, f),
        Some(Selected::Proficiency) => show_proficiencies(app, f),
        _ => {}
    }
}
