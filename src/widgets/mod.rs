pub mod info_bar;
pub mod player_bar;
pub mod stat_block;
pub mod tab_panel;

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
