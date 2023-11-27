use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::player::Player;

/// A widget that renders the top player bar.
pub struct PlayerBar<'a> {
    /// The bound player.
    player: &'a Player,
    /// Which item to highlight, if any.
    highlight: Option<(u8, Color)>,
    fg: Color,
    bg: Color,
}

impl<'a> PlayerBar<'a> {
    /// Construct a new `PlayerBar` bound to the given player.
    pub fn new(player: &'a Player) -> Self {
        Self {
            player,
            highlight: None,
            fg: Color::Yellow,
            bg: Color::Black,
        }
    }

    /// Set which item to highlight, if any.
    pub fn highlight(mut self, item: u8, color: Color) -> Self {
        self.highlight = Some((item, color));
        self
    }

    /// Set the foreground color of the widget
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set the background color of the widget
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }
}

impl<'a> Widget for PlayerBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut items = vec![
            Span::styled(format!("Name: {: <5}", self.player.name), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(format!("(R)ace: {: <10}", self.player.race), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(format!("Level: {: <2}", self.player.level), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(format!("(C)lass: {: <10}", self.player.class), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(
                format!("Alignment: {: <2}", self.player.alignment),
                self.style(),
            ),
        ];

        if let Some((item, color)) = self.highlight {
            // Actual item and '|' separator make stride of 2.
            items[item as usize * 2].patch_style(Style::default().fg(Color::Black).bg(color));
        }

        Paragraph::new(vec![Line::from(items)])
            .block(
                Block::new()
                    .title("Player Sheet (u)")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.fg)),
            )
            .alignment(Alignment::Center)
            .render(area, buf)
    }
}
