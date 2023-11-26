use std::borrow::Cow;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Widget},
};

use super::{vec_view::VecView, PopupSize};

/// A vector popup menu. Like a [`VecView`], but for popups instead.
pub struct VecPopup<'a, T> {
    view: VecView<'a, T>,
    size: PopupSize,
}

impl<'a, T> VecPopup<'a, T> {
    /// Create a new [`VecPopup`] with the given size and the given data
    pub fn new<D>(data: D, size: PopupSize) -> Self
    where
        VecView<'a, T>: From<D>,
    {
        Self {
            view: VecView::from(data),
            size,
        }
    }

    /// Set the foreground color of the widget
    pub fn fg(mut self, color: Color) -> Self {
        self.view.fg = color;
        self
    }

    /// Set the background color of the widget
    pub fn bg(mut self, color: Color) -> Self {
        self.view.bg = color;
        self
    }

    /// Set whether or not the widget will wrap content
    pub fn wrap(mut self) -> Self {
        self.view.wrap = true;
        self
    }

    /// Scroll the widget content by the given height
    pub fn scroll_to(mut self, height: u32) -> Self {
        self.view.scroll = height;
        self
    }

    /// Highlight the specified element in the vector with the specified color
    pub fn highlight(mut self, item: u32, color: Color) -> Self {
        self.view.highlight = Some((item, color));
        self
    }

    /// Wrap the widget in a block.
    ///
    /// Block padding should not be used.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.view.block = Some(block);
        self
    }

    /// Set the text alignment of the widget's content
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.view.alignment = alignment;
        self
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

impl<'a, T> Widget for VecPopup<'a, T>
where
    Cow<'a, str>: From<&'a T>,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let chunk = self.rect(area);

        let clear_string = " ".repeat(chunk.width as usize);
        for y in chunk.top()..chunk.bottom() {
            buf.set_string(chunk.x, y, &clear_string, self.view.style())
        }

        self.view.render(chunk, buf);
    }
}
