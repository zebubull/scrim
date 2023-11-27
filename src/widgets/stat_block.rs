use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};

use crate::player::stats::Stats;

/// A widget that renders the player's stat block.
pub struct StatBlock<'a> {
    /// The bound player stats.
    stats: &'a Stats,
    /// Which stat to highlight, if any.
    highlight: Option<(u8, Color)>,
    fg: Color,
    bg: Color,
}

impl<'a> StatBlock<'a> {
    /// Create a new `StatBlock` bound to the given stats.
    pub fn new(stats: &'a Stats) -> Self {
        Self {
            stats,
            highlight: None,
            fg: Color::Yellow,
            bg: Color::Black,
        }
    }

    /// Set which stat to highlight.
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

impl<'a> Widget for StatBlock<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        const STAT_NAMES: [&str; 6] = ["STR", "DEX", "CON", "INT", "WIS", "CHA"];

        // Stat block is 17 lines long
        let mut lines = Vec::with_capacity(17);
        self.stats
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(idx, stat)| {
                lines.push(Line::from(Span::styled(
                    format!("{}: {: <2}", STAT_NAMES[idx], stat),
                    self.style(),
                )));
                lines.push(Line::from(Span::styled(
                    format!("{:+}", ((stat as f32 - 10.0) / 2.0).floor() as i8),
                    self.style(),
                )));
                lines.push(Line::from(Span::from("-------").yellow()));
            });

        if let Some((item, color)) = self.highlight {
            // Main line, modifier, and separator make stride of 3.
            lines[item as usize * 3].spans[0]
                .patch_style(Style::default().fg(Color::Black).bg(color));
        }

        Paragraph::new(lines)
            .block(
                Block::new()
                    .title("Stats")
                    .title_alignment(Alignment::Center)
                    .title(Title::from("(s)").position(Position::Bottom))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.fg)),
            )
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
