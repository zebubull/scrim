use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::player::Player;

/// A widget to display the player info bar.
pub struct InfoBar<'a> {
    /// The bound player.
    player: &'a Player,
    /// Which item to highlight, if any.
    highlight: Option<(u8, Color)>,
    fg: Color,
    bg: Color,
}

impl<'a> InfoBar<'a> {
    /// Create a new `InfoBar` bound to the given player.
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

impl<'a> Widget for InfoBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut line = Line::from(vec![
            Span::styled(format!("HP: {}", self.player.hp), self.style()),
            Span::styled("/", self.style()),
            Span::styled(format!("{}", self.player.max_hp), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(format!("Temp HP: {}", self.player.temp_hp), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(format!("AC: {}", self.player.ac), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(format!("Prof: {:+}", self.player.prof_bonus), self.style()),
            Span::styled(" | ", self.style()),
            Span::styled(
                format!(
                    "Hit dice: {}d{}",
                    self.player.hit_dice_remaining, self.player.hit_dice
                ),
                self.style(),
            ),
            Span::styled(" | ", self.style()),
            Span::styled(
                format!("Background: {}", self.player.background),
                self.style(),
            ),
            Span::styled(" | ", self.style()),
            Span::styled("(F)unds", self.style()),
            Span::styled(" | ", self.style()),
            Span::styled("(P)roficiencies", self.style()),
        ]);

        if let Some((item, color)) = self.highlight {
            // Actual item and '|' separator make stride of 2.
            line.spans[item as usize * 2].patch_style(Style::default().fg(self.bg).bg(color));
        }

        Paragraph::new(vec![line])
            .block(
                Block::new()
                    .title("Info (i)")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.fg)),
            )
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
