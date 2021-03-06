//! Handlers can be chained and dispatch events and other stuff
use super::*;

pub mod standard;

/// Handlers are stacked inside a Context and any render/event/size action goes through the handler stack
pub trait Handler<E>: Sized + 'static where E: Env {
    fn _render(l: Link<E>, r: &mut RenderLink<E>);
    fn _event_direct(l: Link<E>, e: &EventCompound<E>) -> EventResp;
    fn _event_root(l: Link<E>, e: &EventCompound<E>) -> EventResp;
    fn _size(l: Link<E>, e: &EStyle<E>) -> ESize<E>;
    fn _send_event(l: Link<E>, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,GuionError<E>>;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(mut l: Link<E>, r: &mut RenderLink<E>) {
        l._render(r)
    }
    #[inline] 
    fn _event_direct(mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        l._event_direct(e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: &EventCompound<E>) -> EventResp {
        if !e.event._root_only() {//TODO warn eprint??
            l.ctx.event_direct(l.widget,e)
        }else{
            false
        }
    }
    #[inline] 
    fn _size(mut l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        l._size(e)
    }
    #[inline]
    fn _send_event(mut l: Link<E>, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,GuionError<E>> {
        l._send_event(e,child)
    }
}
