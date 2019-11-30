use crate::widget::Link;
use std::any::Any;
use crate::panel::imp::PaneEntry;
use crate::widget::Widget;
use crate::widget::env::*;
use crate::render::Render;

pub mod imp;

pub trait Pane<E> where E: Env {
    type C: ChildEntry<E> + 'static;

    fn childs(&self) -> &[Self::C];

    fn commit(&self) -> &E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;
    fn parent(&self) -> Option<&E::WidgetID>;
    fn parent_mut(&mut self) -> &mut Option<E::WidgetID>;
}

pub trait ChildEntry<E>: Clone where E: Env {
    fn child(&self) -> E::WidgetID;
    fn bounds(&self) -> (u32,u32,u32,u32);
}

impl<E,T> Widget<E> for T where T: Pane<E> + 'static, E: Env + 'static {
    fn _render(&self) -> fn(Link<E>, E::Renderer) {
        render::<T,E>
    }

    fn _event(&self) -> fn(Link<E>, E::Event) {
        event::<T,E>
    }

    fn commit(&self) -> &E::Commit {
        Pane::commit(self)
    }
    fn commit_mut(&mut self) -> &mut E::Commit {
        Pane::commit_mut(self)
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        Pane::parent(self)
    }

    fn parent_mut(&mut self) -> &mut Option<E::WidgetID> {
        Pane::parent_mut(self)
    }

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=((u32,u32,u32,u32),E::WidgetID)> + 'a> {
        Box::new(
            Pane::childs(self)
            .iter()
            .map(|c| (c.bounds(),c.child()) )
        )
    }

    fn as_any(&self) -> &dyn Any {self}
    fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: Pane<E> + 'static, E: Env + 'static>(l: Link<E>, mut r: E::Renderer) {
    for c in childs::<W,_>(l) {
        let h = l.widgets().get(&c.id)
        .expect("Pane contains lost Widget")
        .render();

        h(l, &c.id, r.slice(c.bounds) );
    }
}

fn event<W: Pane<E> + 'static, E: Env + 'static>(l: Link<E>, mut r: E::Event) {
    unimplemented!()
}

fn childs<W: Pane<E> + 'static, E: Env + 'static>(l: &Link<E>) -> Vec<PaneEntry<E>> {
    l.me::<W>().childs()
        .iter()
        .map(|e| PaneEntry::from(e) )
        .collect()
}