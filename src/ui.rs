use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style}, text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, Selected, Tab};

macro_rules! separator {
    ($color: expr) => { Span::styled(" | ", Style::default().fg($color)) }
}

macro_rules! colored_span {
    ($text: expr, $color: expr) => { Span::styled($text, Style::default().fg($color))}
}

fn get_top_bar<'a>(app: &mut App) -> Paragraph<'a> {
    let mut items = vec![
        colored_span!(format!("Name: {: <24}", app.player.name), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Race: {: <10}", app.player.race), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Level: {: <2}", app.player.level), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Class: {: <10}", app.player.class), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Alignment: {: <2}", app.player.alignment), Color::Yellow),
    ];

    if let Some(Selected::TopBarItem(item)) = app.selected {
        items[item as usize * 2].style = Style::default().fg(Color::Black).bg(if app.editing {Color::LightGreen} else {Color::Yellow});
    }
    
    Paragraph::new(vec![Line::from(items)])
        .block(Block::new()
            .title("Player Sheet").title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .alignment(Alignment::Center)
}

fn calculate_modifier(stat: u8) -> i8 {
    let stat = stat as f32;
    ((stat - 10.0) / 2.0).floor() as i8
}

fn get_stat_block<'a>(app: &mut App) -> Paragraph<'a> {
    let mut lines = vec![
        Line::from(colored_span!(format!("STR: {: <2}", app.player.stats.strength), Color::Yellow)),
        Line::from(colored_span!(format!("{:+}", calculate_modifier(app.player.stats.strength)), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("DEX: {: <2}", app.player.stats.dexterity), Color::Yellow)),
        Line::from(colored_span!(format!("{:+}", calculate_modifier(app.player.stats.dexterity)), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("CON: {: <2}", app.player.stats.constitution), Color::Yellow)),
        Line::from(colored_span!(format!("{:+}", calculate_modifier(app.player.stats.constitution)), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("INT: {: <2}", app.player.stats.intelligence), Color::Yellow)),
        Line::from(colored_span!(format!("{:+}", calculate_modifier(app.player.stats.intelligence)), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("WIS: {: <2}", app.player.stats.wisdom), Color::Yellow)),
        Line::from(colored_span!(format!("{:+}", calculate_modifier(app.player.stats.wisdom)), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("CHA: {: <2}", app.player.stats.charisma), Color::Yellow)),
        Line::from(colored_span!(format!("{:+}", calculate_modifier(app.player.stats.charisma)), Color::Yellow)),
    ];

    if let Some(Selected::StatItem(item)) = app.selected {
        lines[item as usize * 3].spans[0].style = Style::default().fg(Color::Black).bg(if app.editing {Color::LightGreen} else {Color::Yellow});
    }

    Paragraph::new(lines)
        .block(Block::new()
            .title("Stats").title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .alignment(Alignment::Center)
}

fn get_info_block<'a>(app: &mut App) -> Paragraph<'a> {
    let mut lines = vec![
        Line::from(vec![
            colored_span!(format!("HP: {}", app.player.hp), Color::Yellow),
            colored_span!("/", Color::Yellow),
            colored_span!(format!("{}", app.player.max_hp), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Temp HP: {}", app.player.temp_hp), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("AC: {}", app.player.ac), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Prof: {:+}", app.player.prof_bonus), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Hit dice: {}d{}", app.player.hit_dice_remaining, app.player.hit_dice), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Background: {}", app.player.background), Color::Yellow),
        ]),
    ];

    if let Some(Selected::InfoItem(item)) = app.selected {
        lines[0].spans[item as usize * 2].style = Style::default().fg(Color::Black).bg(if app.editing {Color::LightGreen} else {Color::Yellow});
    }

    Paragraph::new(lines)
        .block(Block::new()
            .title("Info").title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .alignment(Alignment::Left)
}

fn get_tab_block<'a>(app: &'a mut App) -> Paragraph<'a> {
    let mut lines: Vec<Line> = match app.current_tab {
        Tab::Notes => {
            app.player.notes.iter()
                .skip(app.vscroll.into())
                .take(app.viewport_height.into())
                .map(|s| Line::from(colored_span!(s, Color::Yellow))).collect()
        },
        Tab::Inventory => {
            app.player.inventory.iter()
                .skip(app.vscroll.into())
                .take(app.viewport_height.into())
                .map(|s| Line::from(colored_span!(s, Color::Yellow))).collect()
        },
        Tab::Spells => {
            app.player.spells.iter()
                .skip(app.vscroll.into())
                .take(app.viewport_height.into())
                .map(|s| Line::from(colored_span!(s, Color::Yellow))).collect()
        },
    };

    if let Some(Selected::TabItem(item)) = app.selected {
        if app.can_edit_tab() {
            // can_edit_tab returns true if there is at least one item in the section. If it is false
            // then this would crash as it would try to index an empty vector
            lines[(item as u16 - app.vscroll) as usize].spans[0].style = Style::default().fg(Color::Black).bg(if app.editing {Color::LightGreen} else {Color::Yellow});
        }
    }

    Paragraph::new(lines)
        .block(Block::new()
            .title(format!("{}", app.current_tab))
            .borders(Borders::ALL))
        .alignment(Alignment::Left)
}

fn get_tab_display<'a>(app: &mut App) -> Paragraph<'a> {
    let mut text = vec![
        colored_span!("NOTES", Color::White),
        Span::from("   "),
        colored_span!("INVENTORY", Color::White),
        Span::from("   "),
        colored_span!("SPELLS", Color::White),
    ];

    let idx = match app.current_tab {
        Tab::Notes => 0,
        Tab::Inventory => 2,
        Tab::Spells => 4,
    };

    text[idx].style = Style::default().fg(Color::Black).bg(if app.selected.is_some() { Color::Yellow } else { Color::White });

    Paragraph::new(vec![Line::from(text)])
        .alignment(Alignment::Left)
}

fn get_percent_display<'a>(app: &mut App) -> Paragraph<'a> {
    let text = if app.vscroll == 0 {
        String::from("TOP")
    } else if app.vscroll == app.current_tab_len() as u16 - app.viewport_height {
        String::from("BOT")
    } else {
        format!("{}%", app.vscroll * 100 / (app.current_tab_len() as u16 - app.viewport_height))
    };

    Paragraph::new(vec![Line::from(colored_span!(text, Color::White))])
}

pub fn render(app: &mut App, f: &mut Frame) {
    let vchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ]).split(f.size());

    let hchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(9),
            Constraint::Min(1),
        ]).split(vchunks[1]);

    let statchunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(19),
            Constraint::Min(1)
        ]).split(hchunks[0]);

    let rchunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ]).split(hchunks[1]);

    let bchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(1),
            Constraint::Length(4),
        ]).split(rchunks[2]);

    let top_bar = get_top_bar(app);
    f.render_widget(top_bar, vchunks[0]);

    let stat_block = get_stat_block(app);
    f.render_widget(stat_block, statchunk[0]);

    let info_block = get_info_block(app);
    f.render_widget(info_block, rchunks[0]);
    
    let tab_block = get_tab_block(app);
    f.render_widget(tab_block, rchunks[1]);

    let tab_display = get_tab_display(app);
    f.render_widget(tab_display, bchunks[0]);

    let percent_display = get_percent_display(app);
    f.render_widget(percent_display, bchunks[1]);
}
