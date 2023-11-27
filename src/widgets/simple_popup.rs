use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use super::PopupSize;

/// A simple popup menu that displays just a string
pub struct SimplePopup<'a> {
    text: &'a String,
    size: PopupSize,
    fg: Color,
    bg: Color,
    wrap: bool,
    scroll: u32,
    block: Option<Block<'a>>,
    alignment: Alignment,
}

impl<'a> SimplePopup<'a> {
    /// Create a new [`SimplePopup`] with the given size and the given data
    pub fn new(data: &'a String, size: PopupSize) -> Self {
        Self {
            text: data,
            size,
            fg: Color::White,
            bg: Color::Black,
            wrap: false,
            scroll: 0,
            block: None,
            alignment: Alignment::Left,
        }
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

    /// Set whether or not the widget will wrap content
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Scroll the widget content by the given height
    pub fn scroll_to(mut self, height: u32) -> Self {
        self.scroll = height;
        self
    }

    /// Wrap the widget in a block.
    ///
    /// Block padding should not be used.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the text alignment of the widget's content
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Get the popup syle
    pub fn style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    pub fn rect(&self, parent: Rect) -> Rect {
        use PopupSize::*;
        match self.size {
            Absolute(width, height) => {
                let vspacing = (parent.height - height) / 2;
                let vchunk = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Length(vspacing),
                        Constraint::Length(height),
                        Constraint::Length(vspacing),
                    ])
                    .split(parent)[1];

                let hspacing = (parent.width - width) / 2;
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Length(hspacing),
                        Constraint::Length(width),
                        Constraint::Length(hspacing),
                    ])
                    .split(vchunk)[1]
            }
            Percentage(width, height) => {
                let vchunk = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage((100 - height) / 2),
                        Constraint::Percentage(height),
                        Constraint::Length((100 - height) / 2),
                    ])
                    .split(parent)[1];

                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage((100 - width) / 2),
                        Constraint::Percentage(width),
                        Constraint::Percentage((100 - width) / 2),
                    ])
                    .split(vchunk)[1]
            }
        }
    }
}

impl<'a> Widget for SimplePopup<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let chunk = self.rect(area);

        // Clear widget does not properly set background color, so this will have to do for now.
        // Not necessarily the fastest but oh well :p
        let clear_string = " ".repeat(chunk.width as usize);
        for y in chunk.top()..chunk.bottom() {
            buf.set_string(chunk.x, y, &clear_string, self.style())
        }

        let mut p = Paragraph::new(self.text.clone())
            .style(self.style())
            .block(self.block.unwrap_or_default())
            .scroll((self.scroll as u16, 0))
            .alignment(self.alignment);

        if self.wrap {
            p = p.wrap(Wrap { trim: false });
        }

        p.render(chunk, buf);
    }
}
