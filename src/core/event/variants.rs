//! standard variants
use crate::core::event::key::Key;
use super::*;

#[derive(Clone)]
pub struct KbdDown<E> where E: Env {
    pub key: EEKey<E>,
    //pub widget: E::WidgetPath,
}
#[derive(Clone)]
pub struct KbdUp<E> where E: Env {
    pub key: EEKey<E>,
    pub down_widget: E::WidgetPath,
    pub down_ts: u64,
}
#[derive(Clone)]
pub struct KbdPress<E> where E: Env {
    pub key: EEKey<E>,
    pub down_widget: E::WidgetPath,
    pub down_ts: u64,
}

#[derive(Clone)]
pub struct MouseDown<E> where E: Env {
    pub key: EEKey<E>,
    pub pos: Offset,
}
#[derive(Clone)]
pub struct MouseUp<E> where E: Env {
    pub key: EEKey<E>,
    pub pos: Offset,
    pub down_pos: Offset,
    pub down_widget: E::WidgetPath,
    pub down_ts: u64,
}

#[derive(Clone)]
pub struct MouseMove {
    pub dest: Offset,
}

#[derive(Clone)]
pub struct MouseEnter {
    pub dest: Offset,
}
#[derive(Clone)]
pub struct MouseLeave {
    pub dest: Offset,
}

#[derive(Clone)]
pub struct WindowMove {
    pub pos: Offset,
    pub size: Size,
}

#[derive(Clone)]
pub struct WindowResize {
    pub size: Size,
}

#[derive(Clone)]
pub struct GainedFocus {
}

#[derive(Clone)]
pub struct LostFocus {
}

macro_rules! pos {
    ($field:ident) => {
        #[inline]
        fn position(&self) -> Option<Offset> {
            Some(self.$field.clone())
        }
    };
}
macro_rules! bounds {
    () => {
        #[inline]
        fn _bounds_mut(&mut self) -> Option<&mut Bounds> {
            Some(&mut self.current_bounds)
        }
    };
}

macro_rules! consuming {
    () => {
        #[inline]
        fn consuming(&self) -> bool {
            true
        }
    };
}

macro_rules! selected {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::SELECTED
        }
    };
}

macro_rules! hovered {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::HOVERED
        }
    };
}

macro_rules! root {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::ROOT
        }
    };
}

macro_rules! invalid {
    () => {
        #[inline]
        fn destination(&self) -> EEDest<E> {
            Destination::INVALID
        }
    };
}

impl<E> Variant<E> for KbdDown<E> where E: Env {selected!();}
impl<E> Variant<E> for KbdPress<E> where E: Env {selected!();}
impl<E> Variant<E> for KbdUp<E> where E: Env {selected!();}

impl<E> Variant<E> for MouseDown<E> where E: Env {consuming!();hovered!();pos!(pos);}
impl<E> Variant<E> for MouseUp<E> where E: Env {consuming!();hovered!();pos!(pos);}
impl<E> Variant<E> for MouseMove where E: Env {consuming!();root!();pos!(dest);}
impl<E> Variant<E> for MouseEnter where E: Env {consuming!();invalid!();pos!(dest);}
impl<E> Variant<E> for MouseLeave where E: Env {consuming!();invalid!();pos!(dest);}

impl<E> Variant<E> for GainedFocus where E: Env {consuming!();invalid!();}
impl<E> Variant<E> for LostFocus where E: Env {consuming!();invalid!();}

/*impl<E> KbdDown<E> where E: Env, E::Context: AsHandlerStateful<E> {
    pub fn widget(&self, c: &E::Context) -> &E::WidgetPath {
        c.state()
    }
}*/

#[non_exhaustive]
#[derive(Clone)]
pub enum RootEvent<E> where E: Env {
    KbdDown{key: EEKey<E>},
    KbdPress{key: EEKey<E>},
    KbdUp{key: EEKey<E>},
    MouseDown{key: EEKey<E>},
    MouseUp{key: EEKey<E>},
    MouseMove{dest: Offset},
    WindowMove{pos: Offset, size: Size},
    WindowResize{size: Size},
}

impl<E> Variant<E> for RootEvent<E> where E: Env {
    #[inline]
    fn position(&self) -> Option<Offset> {
        None
    }
    #[inline]
    fn filter(&self, subbounds: &Bounds) -> bool {
        false
    }
    #[inline]
    fn consuming(&self) -> bool {
        true
    }
    #[inline]
    fn destination(&self) -> EEDest<E> {
        Destination::ROOT
    }
    #[inline]
    fn _root_only(&self) -> bool {
        true
    }
}