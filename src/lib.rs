#![warn(missing_docs)]

//! A wrapper for [`tui`] which should make starting projects
//! easier and quicker, with some minor constraints on design.
//! As progress continues I will try to sacrifice as little
//! design adaptability as possible. However, this may not
//! be possible and I may be in future forced to redesign
//! the crate. Keep in mind that this is my first crate I
//! have published to crates.io, and may not be as well 
//! designed as some Rustaceans would like.
//! 
//! This crate is dependent on both [`tui`] and [`crossterm`]:
//! ```toml
//! [dependencies]
//! crossterm = "0.25"
//! tui = "0.19.0"
//! ```

/// This module contains the [`UI`](crate::ui::UI) struct.
pub mod ui;

/// This module contains the structs [`App`](crate::app::App)
/// and [`Ticked`](crate::app::Ticked) which are used with the
/// [`ui`](crate::ui) module to create applications.
pub mod app;

mod tests;