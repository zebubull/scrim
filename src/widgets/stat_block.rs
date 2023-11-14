use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{player::Stats, widgets::colored_span};

/// A widget that renders the player's stat block.
pub struct StatBlock<'a> {
    /// The bound player stats.
    stats: &'a Stats,
    /// Which stat to highlight, if any.
    highlight: Option<u8>,
    /// Whether the app is editing.
    editing: bool,
}

impl<'a> StatBlock<'a> {
    /// Create a new `StatBlock` bound to the given stats.
    pub fn new(stats: &'a Stats) -> Self {
        Self {
            stats,
            highlight: None,
            editing: false,
        }
    }

    /// Set which stat to highlight, if any.
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
                lines.push(Line::from(colored_span!(
                    format!("{}: {: <2}", STAT_NAMES[idx], stat),
                    Color::Yellow
                )));
                lines.push(Line::from(colored_span!(
                    format!("{:+}", ((stat as f32 - 10.0) / 2.0).floor() as i8),
                    Color::Yellow
                )));
                lines.push(Line::from(colored_span!("-------", Color::Yellow)));
            });

        if let Some(item) = self.highlight {
            // Main line, modifier, and separator make stride of 3.
            lines[item as usize * 3].spans[0].patch_style(Style::default().fg(Color::Black).bg(
                if self.editing {
                    Color::LightGreen
                } else {
                    Color::Yellow
                },
            ));
        }

        Paragraph::new(lines)
            .block(
                Block::new()
                    .title("Stats")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
