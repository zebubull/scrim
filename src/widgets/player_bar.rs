use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    player::Player,
    widgets::{colored_span, separator},
};

pub struct PlayerBar<'a> {
    player: &'a Player,
    highlight: Option<u8>,
    editing: bool,
}

impl<'a> PlayerBar<'a> {
    pub fn new(player: &'a Player) -> Self {
        Self {
            player,
            highlight: None,
            editing: false,
        }
    }

    pub fn highlight(mut self, item: Option<u8>) -> Self {
        self.highlight = item;
        self
    }

    pub fn editing(mut self, editing: bool) -> Self {
        self.editing = editing;
        self
    }
}

impl<'a> Widget for PlayerBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut items = vec![
            colored_span!(format!("Name: {: <5}", self.player.name), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Race: {: <10}", self.player.race), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Level: {: <2}", self.player.level), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(format!("Class: {: <10}", self.player.class), Color::Yellow),
            separator!(Color::Yellow),
            colored_span!(
                format!("Alignment: {: <2}", self.player.alignment),
                Color::Yellow
            ),
        ];

        if let Some(item) = self.highlight {
            items[item as usize * 2].patch_style(Style::default().fg(Color::Black).bg(
                if self.editing {
                    Color::LightGreen
                } else {
                    Color::Yellow
                },
            ));
        }

        Paragraph::new(vec![Line::from(items)])
            .block(
                Block::new()
                    .title("Player Sheet")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center)
            .render(area, buf)
    }
}
