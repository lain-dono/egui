//! Containers are pieces of the UI which wraps other pieces of UI. Examples: [`Window`], [`ScrollArea`], [`Resize`], [`SidePanel`], etc.
//!
//! For instance, a [`Frame`] adds a frame and background to some contained UI.

pub mod area;
pub mod collapsing_header;
mod combo_box;
pub mod frame;
pub mod panel;
pub mod popup;
pub mod resize;
pub mod scroll_area;
pub mod window;

pub use {
    area::Area,
    collapsing_header::{CollapsingHeader, CollapsingResponse},
    combo_box::*,
    frame::Frame,
    panel::{CentralPanel, SidePanel, TopBottomPanel},
    popup::*,
    resize::Resize,
    scroll_area::ScrollArea,
    window::Window,
};
