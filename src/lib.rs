//#![warn(clippy::all)]

pub mod aliases;
pub mod backend;
pub mod ctx;
pub mod env;
pub mod event;
pub mod handler;
pub mod id;
pub mod layout;
pub mod path;
pub mod render;
pub mod state;
pub mod style;
pub mod util;
pub mod widget;
pub mod widgets;
pub mod validation;

pub(crate) use aliases::*;
pub(crate) use backend::*;
pub(crate) use ctx::queue::*;
pub(crate) use ctx::clipboard::*;
pub(crate) use ctx::*;
pub(crate) use env::*;
pub(crate) use event::imp::*;
pub(crate) use event::key::*;
pub(crate) use event::variant::*;
pub(crate) use event::variants::*;
pub(crate) use event::filter::*;
pub(crate) use event::compound::*;
pub(crate) use event::*;
pub(crate) use handler::*;
pub(crate) use id::standard::*;
pub(crate) use id::*;
pub(crate) use layout::*;
pub(crate) use path::standard::*;
pub(crate) use path::*;
pub(crate) use qwutils::*;
pub(crate) use render::link::*;
pub(crate) use render::widgets::*;
pub(crate) use render::*;
pub(crate) use state::*;
pub(crate) use state::dyn_state::*;
pub(crate) use std::any::Any;
pub(crate) use style::color::*;
pub(crate) use style::font::*;
pub(crate) use style::selector::standard::*;
pub(crate) use style::selector::*;
pub(crate) use style::*;
pub(crate) use util::border::*;
pub(crate) use util::bounded_widget::*;
pub(crate) use util::bounds::*;
pub(crate) use util::traitcast::*;
pub(crate) use util::*;
pub(crate) use widget::array::*;
pub(crate) use widget::as_widget::*;
pub(crate) use widget::cast::*;
pub(crate) use widget::link::*;
pub(crate) use widget::resolvable::*;
pub(crate) use widget::resolved::*;
pub(crate) use widget::root::*;
pub(crate) use widget::ident::*;
pub(crate) use widget::*;

pub type EventResp = bool;
