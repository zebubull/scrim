use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{player::Player, widgets::separator};

/// A widget to display the player info bar.
pub struct InfoBar<'a> {
    /// The bound player.
    player: &'a Player,
    /// Which item to highlight, if any.
    highlight: Option<u8>,
    /// Whether the app is editing.
    editing: bool,
}

impl<'a> InfoBar<'a> {
    /// Create a new `InfoBar` bound to the given player.
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

impl<'a> Widget for InfoBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut line = Line::from(vec![
            Span::from(format!("HP: {}", self.player.hp)).yellow(),
            Span::from("/").yellow(),
            Span::from(format!("{}", self.player.max_hp)).yellow(),
            separator!(Color::Yellow),
            Span::from(format!("Temp HP: {}", self.player.temp_hp)).yellow(),
            separator!(Color::Yellow),
            Span::from(format!("AC: {}", self.player.ac)).yellow(),
            separator!(Color::Yellow),
            Span::from(format!("Prof: {:+}", self.player.prof_bonus)).yellow(),
            separator!(Color::Yellow),
            Span::from(format!(
                "Hit dice: {}d{}",
                self.player.hit_dice_remaining, self.player.hit_dice
            ))
            .yellow(),
            separator!(Color::Yellow),
            Span::from(format!("Background: {}", self.player.background)).yellow(),
            separator!(Color::Yellow),
            Span::from("(F)unds").yellow(),
            separator!(Color::Yellow),
            Span::from("(P)roficiencies").yellow(),
        ]);

        if let Some(item) = self.highlight {
            // Actual item and '|' separator make stride of 2.
            line.spans[item as usize * 2].patch_style(Style::default().fg(Color::Black).bg(
                if self.editing {
                    Color::LightGreen
                } else {
                    Color::Yellow
                },
            ));
        }

        Paragraph::new(vec![line])
            .block(
                Block::new()
                    .title("Info (i)")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
