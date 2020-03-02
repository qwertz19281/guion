use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;
use super::state::StdState;
use event::variants::{GainedFocus, LostFocus};
use crate::core::ctx::widgets::Widgets;

pub mod imp;

pub struct StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self>, EEvent<E>: StdVarSup<E> {
    pub sup: S,
    pub s: StdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self>, EEvent<E>: StdVarSup<E> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            s: StdState::new(),
            _c: PhantomData,
        }
    }

    pub fn unfocus(ctx: &mut E::Context, root: &E::Storage, root_bounds: &Bounds, ts: u64) {
        if let Some(p) = ctx.as_mut().s.kbd.focused.take() {
            if let Ok(w) = root.widget(p.slice()) {
                let bounds = root.trace_bounds(ctx,p.slice(),root_bounds,false).unwrap();
                ctx.link(w)._event_root((Event::from(LostFocus{}),&bounds,ts));
            }
        }
    }

    pub fn focus(mut l: Link<E>, ts: u64, root_bounds: &Bounds, widget_bounds: &Bounds) {
        /*if let Some(p) = l.as_mut().s.kbd.focused.take() {
            l.with_widget(p.slice())
                .expect("TODO")
                ._event_root((Event::from(LostFocus{}),bounds,ts));
            }*/
        Self::unfocus(l.ctx,l.widget.stor,root_bounds,ts);
        l.as_mut().s.kbd.focused = Some(l.widget.path.clone());
        l._event_root((Event::from(GainedFocus{}),widget_bounds,ts));
    }
}