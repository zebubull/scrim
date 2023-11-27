use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
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
    highlight: Option<(u16, Color)>,
    /// The amount of lines to scroll the viewport by.
    scroll: u16,
    fg: Color,
    bg: Color,
    select: Color,
}

impl<'a> TabPanel<'a> {
    pub fn new(player: &'a Player, tab: Tab) -> Self {
        Self {
            player,
            tab,
            highlight: None,
            scroll: 0,
            fg: Color::Yellow,
            bg: Color::Black,
            select: Color::White,
        }
    }

    /// Set the highlighted line.
    pub fn highlight(mut self, item: u16, color: Color) -> Self {
        self.highlight = Some((item, color));
        self
    }

    /// Set the amount of scroll lines.
    pub fn scroll(mut self, scroll: u16) -> Self {
        self.scroll = scroll;
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

    /// Set the tab select color of the widget
    pub fn select(mut self, color: Color) -> Self {
        self.select = color;
        self
    }

    pub fn style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
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
            .fg(self.fg)
            .bg(self.bg)
            .scroll_to(self.scroll as u32)
            .block(
                Block::default()
                    .title(format!("{} (t)", self.tab))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.fg)),
            );

        if let Some((item, color)) = self.highlight {
            if !tab.is_empty() {
                tab_view = tab_view.highlight(item as u32, color);
            }
        }

        tab_view.render(content_chunk, buf);

        let mut text = vec![
            Span::styled("NOTES", Style::default().fg(self.select).bg(self.bg)),
            Span::from("   "),
            Span::styled("INVENTORY", Style::default().fg(self.select).bg(self.bg)),
            Span::from("   "),
            Span::styled("SP(E)LLS", Style::default().fg(self.select).bg(self.bg)),
        ];

        let idx = match self.tab {
            Tab::Notes => 0,
            Tab::Inventory => 2,
            Tab::Spells => 4,
        };

        text[idx].patch_style(
            Style::default()
                .fg(self.bg)
                .bg(if self.highlight.is_some() {
                    self.fg
                } else {
                    self.select
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
            format!("{}%", self.scroll * 100 / (len - height))
        };

        Paragraph::new(vec![Line::from(Span::styled(
            text,
            Style::default().bg(self.bg).fg(self.select),
        ))])
        .render(scroll_chunk, buf);
    }
}
