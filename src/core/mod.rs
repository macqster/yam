//! CORE MODULE
//!
//! Rules:
//! - No rendering
//! - No terminal access
//! - No ratatui/crossterm usage
//! - Pure data + logic only

pub mod cell;
pub mod entity;
pub mod flora;
pub mod grid;
pub mod guide;
pub mod guide_line;
pub mod spatial;
pub mod world;
