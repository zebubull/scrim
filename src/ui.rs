use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style}, text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, Selected};

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
        colored_span!(format!("Background: {: <14}", app.player.background), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Level: {: <2}", app.player.level), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Class: {: <10}", app.player.class), Color::Yellow),
        separator!(Color::Yellow),
        colored_span!(format!("Alignment: {: <2}", app.player.alignment), Color::Yellow),
    ];

    if let Some(Selected::TopBarItem(item)) = app.selected {
        items[item as usize * 2].style = Style::default().fg(Color::Black).bg(if app.editing {Color::LightRed} else {Color::Yellow});
    }
    
    Paragraph::new(vec![Line::from(items)])
        .block(Block::new()
            .title("Player Sheet").title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .alignment(Alignment::Center)
}

fn get_stat_block<'a>(app: &mut App) -> Paragraph<'a> {
    let mut lines = vec![
        Line::from(colored_span!(format!("STR: {: <2}", app.player.stats.strength), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("DEX: {: <2}", app.player.stats.dexterity), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("CON: {: <2}", app.player.stats.constitution), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("INT: {: <2}", app.player.stats.intelligence), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("WIS: {: <2}", app.player.stats.wisdom), Color::Yellow)),
        Line::from(colored_span!("-------", Color::Yellow)),
        Line::from(colored_span!(format!("CHA: {: <2}", app.player.stats.charisma), Color::Yellow)),
    ];

    if let Some(Selected::StatItem(item)) = app.selected {
        lines[item as usize * 2].spans[0].style = Style::default().fg(Color::Black).bg(if app.editing {Color::LightRed} else {Color::Yellow});
    }

    Paragraph::new(lines)
        .block(Block::new()
            .title("Stats").title_alignment(Alignment::Center)
            .borders(Borders::ALL))
        .alignment(Alignment::Left)
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
            Constraint::Length(13),
            Constraint::Min(1)
        ]).split(hchunks[0]);

    let top_bar = get_top_bar(app);
    f.render_widget(top_bar, vchunks[0]);

    let stat_block = get_stat_block(app);
    f.render_widget(stat_block, statchunk[0]);
}