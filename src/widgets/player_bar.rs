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

/// A widget that renders the top player bar.
pub struct PlayerBar<'a> {
    /// The bound player.
    player: &'a Player,
    /// Which item to highlight, if any.
    highlight: Option<u8>,
    /// Whether that app is editing.
    editing: bool,
}

impl<'a> PlayerBar<'a> {
    /// Construct a new `PlayerBar` bound to the given player.
    pub fn new(player: &'a Player) -> Self {
        Self {
            player,
            highlight: None,
            editing: false,
        }
    }

    /// Set which item to highlight, if any.
    pub fn highlight(mut self, item: Option<u8>) -> Self {
        self.highlight = item;
        self
    }

    /// Set whether to display the highlight using standard or editing colors.
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
            // Actual item and '|' separator make stride of 2.
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
