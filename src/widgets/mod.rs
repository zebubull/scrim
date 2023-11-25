/// Info-bar-drawing widget
pub mod info_bar;
/// Player-bar-drawing widget
pub mod player_bar;
/// Stat-block-drawing widget
pub mod stat_block;
/// Tab-panel-drawing widget
pub mod tab_panel;
pub mod vec_popup;
pub mod vec_view;

macro_rules! separator {
    ($color: expr) => {
        Span::styled(" | ", Style::default().fg($color))
    };
}

macro_rules! colored_span {
    ($text: expr, $color: expr) => {
        Span::styled($text, Style::default().fg($color))
    };
}

pub(crate) use colored_span;
pub(crate) use separator;
