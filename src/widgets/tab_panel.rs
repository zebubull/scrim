use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{app::Tab, player::Player, widgets::colored_span};

pub struct TabPanel<'a> {
    player: &'a Player,
    tab: Tab,
    highlight: Option<u16>,
    editing: bool,
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

    pub fn highlight(mut self, item: Option<u16>) -> Self {
        self.highlight = item;
        self
    }

    pub fn editing(mut self, editing: bool) -> Self {
        self.editing = editing;
        self
    }

    pub fn scroll(mut self, scroll: u16) -> Self {
        self.scroll = scroll;
        self
    }
}

impl<'a> Widget for TabPanel<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(1), Constraint::Length(4)])
            .split(vchunks[1]);

        let len = match self.tab {
            Tab::Notes => &self.player.notes,
            Tab::Inventory => &self.player.inventory,
            Tab::Spells => &self.player.spells,
        }
        .len();

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

        Paragraph::new(lines)
            .block(
                Block::new()
                    .title(format!("{}", self.tab))
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .render(vchunks[0], buf);

        let mut text = vec![
            colored_span!("NOTES", Color::White),
            Span::from("   "),
            colored_span!("INVENTORY", Color::White),
            Span::from("   "),
            colored_span!("SPELLS", Color::White),
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
            .render(hchunks[0], buf);

        let text = if self.scroll == 0 {
            String::from("TOP")
        } else if self.scroll == len as u16 - height as u16 {
            String::from("BOT")
        } else {
            format!("{}%", self.scroll * 100 / (len as u16 - height as u16))
        };

        Paragraph::new(vec![Line::from(colored_span!(text, Color::White))]).render(hchunks[1], buf);
    }
}
