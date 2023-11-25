use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{core::Tab, player::Player, widgets::vec_view::VecView};

/// A widget that renders the current tab panel, tab selection, and scroll display.
pub struct TabPanel<'a> {
    /// The bound player.
    player: &'a Player,
    /// The tab to display
    tab: Tab,
    /// Which line to highlight, if any.
    highlight: Option<u16>,
    /// Whether editing mode is enabled.
    editing: bool,
    /// The amount of lines to scroll the viewport by.
    scroll: u16,
}

impl<'a> TabPanel<'a> {
    pub fn new(player: &'a Player, tab: Tab) -> Self {
        Self {
            player,
            tab,
            highlight: None,
            editing: false,
            scroll: 0,
        }
    }

    /// Set the highlighted line.
    pub fn highlight(mut self, item: Option<u16>) -> Self {
        self.highlight = item;
        self
    }

    /// Set whether to display the highlight using standard or editing colors.
    pub fn editing(mut self, editing: bool) -> Self {
        self.editing = editing;
        self
    }

    /// Set the amount of scroll lines.
    pub fn scroll(mut self, scroll: u16) -> Self {
        self.scroll = scroll;
        self
    }
}

impl<'a> Widget for TabPanel<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Create layouts
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let content_chunk = main_chunks[0];
        let bottom_chunk = main_chunks[1];

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(1), Constraint::Length(4)])
            .split(bottom_chunk);

        let tab_chunk = bottom_chunks[0];
        let scroll_chunk = bottom_chunks[1];

        // Draw the main tab pane
        let tab = match self.tab {
            Tab::Notes => &self.player.notes,
            Tab::Inventory => &self.player.inventory,
            Tab::Spells => &self.player.spells,
        };

        let mut tab_view = VecView::from(tab.as_slice())
            .alignment(Alignment::Left)
            .fg(Color::Yellow)
            .bg(Color::Black)
            .scroll_to(self.scroll as u32)
            .block(
                Block::default()
                    .title(format!("{} (t)", self.tab))
                    .borders(Borders::ALL),
            );

        if let Some(item) = self.highlight {
            if !tab.is_empty() {
                tab_view = tab_view.highlight(
                    item as u32,
                    if self.editing {
                        Color::Green
                    } else {
                        Color::Yellow
                    },
                );
            }
        }

        tab_view.render(content_chunk, buf);

        let mut text = vec![
            Span::from("NOTES").white(),
            Span::from("   "),
            Span::from("INVENTORY").white(),
            Span::from("   "),
            Span::from("SP(E)LLS").white(),
        ];

        let idx = match self.tab {
            Tab::Notes => 0,
            Tab::Inventory => 2,
            Tab::Spells => 4,
        };

        text[idx].patch_style(
            Style::default()
                .fg(Color::Black)
                .bg(if self.highlight.is_some() {
                    Color::Yellow
                } else {
                    Color::White
                }),
        );

        Paragraph::new(vec![Line::from(text)])
            .alignment(Alignment::Left)
            .render(tab_chunk, buf);

        let len = tab.len() as u16;
        let height = content_chunk.height - 2; // -2 for borders
                                               // Draw the scroll percentage display
        let text = if self.scroll == 0 {
            String::from("TOP")
        } else if self.scroll >= len - height {
            String::from("BOT")
        } else {
            format!("{}%", self.scroll * 100 / (len as u16 - height as u16))
        };

        Paragraph::new(vec![Line::from(Span::from(text).white())]).render(scroll_chunk, buf);
    }
}
