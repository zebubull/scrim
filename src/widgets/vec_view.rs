use std::borrow::Cow;

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Widget, Wrap},
};

/// A widget that can render a vector of elements
pub struct VecView<'a, T> {
    data: &'a [T],
    pub(super) fg: Color,
    pub(super) bg: Color,
    pub(super) wrap: bool,
    pub(super) scroll: u32,
    pub(super) highlight: Option<(u32, Color)>,
    pub(super) block: Option<Block<'a>>,
    pub(super) alignment: Alignment,
}

impl<'a, T> VecView<'a, T> {
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

    /// Highlight the specified element in the vector with the specified color
    pub fn highlight(mut self, item: u32, color: Color) -> Self {
        self.highlight = Some((item, color));
        self
    }

    /// Wrap the widget in a block.
    ///
    /// Block padding should not be used and borders should be set. Otherwise,
    /// do not use a block.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the text alignment of the widget's content
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }
}

impl<'a, T> Widget for VecView<'a, T>
where
    &'a T: Into<Cow<'a, str>>,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // This does not account for block padding and assumes borders. AFAIK there is no way to get
        // that data from the block so...
        let content_height = if self.block.is_some() {
            area.height - 2
        } else {
            area.height
        };

        let mut lines: Vec<Line> = self
            .data
            .iter()
            .skip(self.scroll as usize)
            .take(content_height as usize)
            .map(|s| Line::from(Span::styled(s, self.style())))
            .collect();

        if let Some((line, color)) = self.highlight {
            lines[line.saturating_sub(self.scroll) as usize].spans[0]
                .patch_style(Style::default().fg(self.bg).bg(color));
        }

        let mut p = Paragraph::new(lines)
            .alignment(self.alignment)
            .block(self.block.unwrap_or_default());

        if self.wrap {
            p = p.wrap(Wrap { trim: false });
        }

        p.render(area, buf);
    }
}

impl<'a, T> From<&'a [T]> for VecView<'a, T>
where
    &'a T: Into<Cow<'a, str>>,
{
    fn from(value: &'a [T]) -> Self {
        Self {
            data: value,
            fg: Color::White,
            bg: Color::Black,
            wrap: false,
            scroll: 0,
            highlight: None,
            block: None,
            alignment: Alignment::Left,
        }
    }
}

impl<'a, T> From<&'a Vec<T>> for VecView<'a, T>
where
    &'a T: Into<Cow<'a, str>>,
{
    fn from(value: &'a Vec<T>) -> Self {
        Self {
            data: value,
            fg: Color::White,
            bg: Color::Black,
            wrap: false,
            scroll: 0,
            highlight: None,
            block: None,
            alignment: Alignment::Left,
        }
    }
}
