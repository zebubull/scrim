/// Info-bar-drawing widget
pub mod info_bar;
/// Player-bar-drawing widget
pub mod player_bar;
/// Popup widget that displays a string
pub mod simple_popup;
/// Stat-block-drawing widget
pub mod stat_block;
/// Tab-panel-drawing widget
pub mod tab_panel;
/// Popup widget that can display a vector
pub mod vec_popup;
/// Widget that displays the contents of a vector
pub mod vec_view;

macro_rules! separator {
    ($color: expr) => {
        Span::styled(" | ", Style::default().fg($color))
    };
}

pub enum PopupSize {
    Percentage(u16, u16),
    Absolute(u16, u16),
}

pub(crate) use separator;
