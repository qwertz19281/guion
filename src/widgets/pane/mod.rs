use crate::core::lazout::TOrientation;
use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::*;
use crate::core::widget::handler::HandlerFns;
use crate::core::widget::link::Link;
use std::any::Any;
use crate::core::widget::Widget;
use crate::core::env::*;
use crate::core::render::*;
use crate::core::event::Event;
use crate::core::lazout::Orientation;

pub mod imp;

pub trait Pane<E> where E: Env {
    type O: TOrientation;

    fn id(&self) -> E::WidgetID;

    fn childs(&self) -> &[E::WidgetID];

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn size(&self) -> Size;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}

impl<E,T,O> Widget<E> for T where T: Pane<E,O=O> + 'static, E: Env + 'static, O: TOrientation {
    #[inline]
    fn id(&self) -> E::WidgetID {
        Pane::id(self)
    }
    #[inline]
    fn _handler(&self) -> HandlerFns<E> {
        HandlerFns{
            render: render::<T,E,O>,
            event: event::<T,E,O>,
            size: size::<T,E,O>,
        }
    }
    #[inline]
    fn invalid(&self) -> bool {
        Pane::invalid(self)
    }
    fn set_invalid(&mut self, v: bool) {
        Pane::set_invalid(self,v)
    }
    #[inline]
    fn parent(&self) -> Option<&E::WidgetID> {
        Pane::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        Pane::set_parent(self,v)
    }
    #[inline]
    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=BoundedWidget<E>> + 'a> {
        Box::new(
            Pane::childs(self)
            .iter()
            .map(IBoundedWidget::into_a)
        )
    }
    
    #[inline] fn as_any(&self) -> &dyn Any {self}
    #[inline] fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

fn render<W: Pane<E,O=O> + 'static, E: Env + 'static, O: TOrientation>(mut l: Link<E>, mut r: E::Renderer) {
    for c in childs::<W,_,O>(&l) {
        l.widget(&c.id)
            .expect("Pane contains lost Widget")
            .handler()
            .render( &mut *l, r.slice(&c.bounds) );
    }
}

fn event<W: Pane<E,O=O> + 'static, E: Env + 'static, O: TOrientation>(mut l: Link<E>, e: E::Event) {
    //TODO special focus/hover enter/leave handling
    for c in childs::<W,_,O>(&l).into_iter().rev() {
        if let Some(e) = e.filter_cloned(&c.bounds) {
            let consuming = e.consuming();

            l.widget(&c.id)
                .expect("Pane contains lost Widget")
                .handler()
                .event( &mut *l, e );

            if consuming {return;}
        }
    }
}

fn size<W: Pane<E,O=O> + 'static, E: Env + 'static, O: TOrientation>(mut l: Link<E>) -> Size {
    unimplemented!()
}
#[inline]
fn childs<W: Pane<E,O=O> + 'static, E: Env + 'static, O: TOrientation>(l: &Link<E>) -> Vec<E::WidgetID> {
    l.me::<W>().childs().to_owned()
}