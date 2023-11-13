/// Application state module
pub mod app;

/// UI rendering module
pub mod ui;

/// UI Widgets
pub mod widgets;

/// Event handler module
pub mod event;

/// TUI utility module
pub mod tui;

/// Application updater module
pub mod update;

/// D&D player definitions
pub mod player;

/// Lookup spell and item descriptions
pub mod lookup;

/// A trait for enums that allow cycling through contents. Use [`impl_cycle`] to
/// automatically implement a non-wrapping version of this trait.
pub trait Cycle: num_traits::FromPrimitive + strum::EnumCount {
    fn next(self) -> Self;
    fn prev(self) -> Self;
}

// I literally spent 2 hours trying to write a proc macro to do this and gave up... so this is good enough
/// Implements a non-wrapping version of the [`Cycle`] trait on the given enum.
macro_rules! impl_cycle {
    ($t: ty) => {
        impl crate::Cycle for $t {
            fn next(self) -> $t {
                use strum::EnumCount;
                num_traits::FromPrimitive::from_u8(std::cmp::min(
                    self as u8 + 1,
                    <$t>::COUNT as u8 - 1,
                ))
                .unwrap()
            }

            fn prev(self) -> $t {
                num_traits::FromPrimitive::from_u8(std::cmp::max(self as i8 - 1, 0) as u8).unwrap()
            }
        }
    };
}

pub(crate) use impl_cycle;
