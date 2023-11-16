use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{app::Tab, player::Player, widgets::colored_span};

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

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(1), Constraint::Length(4)])
            .split(main_chunks[1]);

        // Draw the main tab pane
        let len = match self.tab {
            Tab::Notes => &self.player.notes,
            Tab::Inventory => &self.player.inventory,
            Tab::Spells => &self.player.spells,
        }
        .len();

        // 2 lines for border, 1 line for bottom bar.
        let height = area.height as usize - 3;

        let mut lines: Vec<Line> = match self.tab {
            Tab::Notes => self.player.notes.iter(),
            Tab::Inventory => self.player.inventory.iter(),
            Tab::Spells => self.player.spells.iter(),
        }
        .skip(self.scroll as usize)
        .take(height)
        .map(|s| Line::from(colored_span!(s, Color::Yellow)))
        .collect();

        if let Some(item) = self.highlight {
            if len > 0 {
                lines[(item - self.scroll) as usize].spans[0].patch_style(
                    Style::default().fg(Color::Black).bg(if self.editing {
                        Color::LightGreen
                    } else {
                        Color::Yellow
                    }),
                );
            }
        }

        // Draw the selected tab display
        Paragraph::new(lines)
            .block(
                Block::new()
                    .title(format!("{} (t)", self.tab))
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .render(main_chunks[0], buf);

        let mut text = vec![
            colored_span!("NOTES", Color::White),
            Span::from("   "),
            colored_span!("INVENTORY", Color::White),
            Span::from("   "),
            colored_span!("SPELLS (o)", Color::White),
        ];

        let idx = match self.tab {
            Tab::Notes => 0,
            Tab::Inventory => 2,
            Tab::Spells => 4,
        };

        text[idx].style = Style::default()
            .fg(Color::Black)
            .bg(if self.highlight.is_some() {
                Color::Yellow
            } else {
                Color::White
            });

        Paragraph::new(vec![Line::from(text)])
            .alignment(Alignment::Left)
            .render(bottom_chunks[0], buf);

        // Draw the scroll percentage display
        let text = if self.scroll == 0 {
            String::from("TOP")
        } else if self.scroll == len as u16 - height as u16 {
            String::from("BOT")
        } else {
            format!("{}%", self.scroll * 100 / (len as u16 - height as u16))
        };

        Paragraph::new(vec![Line::from(colored_span!(text, Color::White))])
            .render(bottom_chunks[1], buf);
    }
}
