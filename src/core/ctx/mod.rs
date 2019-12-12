use std::borrow::BorrowMut;
use crate::core::widget::handler::fns::WidgetFns;
use crate::core::style::Style;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;
use std::any::Any;

pub mod ctx_meta;
pub use ctx_meta::*;

pub mod id;
pub use id::*;

pub mod aliases;

pub mod queue;
pub use queue::*;

pub trait Context: Sized + 'static {
    type Handler: ContextLayer<Self>;
    type Meta: ContextMeta<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID<Self>;
    type Commit: Eq + Ord;
    type Style: Style;

    fn handler_mut(&mut self) -> &mut Self::Handler;

    fn widget(&self, i: &Self::WidgetID) -> Option<&Self::DynWidget>;
    fn widget_mut(&mut self, i: &Self::WidgetID) -> Option<&mut Self::DynWidget>;

    #[inline]
    fn has_widget(&self, i: &Self::WidgetID) -> bool {
        self.widget(i).is_some()
    }

    #[inline] fn tune_id(&self, _i: &mut Self::WidgetID) {}
    #[inline] fn tune_id_mut(&mut self, _i: &mut Self::WidgetID) {}

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(&mut self, i: &Self::WidgetID, r: Self::Renderer) {
        Self::Handler::_render(self,i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(&mut self, i: &Self::WidgetID, e: Self::Event) {
        Self::Handler::_event(self,i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(&mut self, i: &Self::WidgetID) -> Size {
        Self::Handler::_size(self,i)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn widget_fns(&self, i: &Self::WidgetID) -> WidgetFns<Self> {
        Widget::_fns(self.widget(i).expect("Lost Widget"))
    }

    #[inline] fn link<'a>(&'a mut self, i: &Self::WidgetID) -> Link<'a,Self> {
        Link{
            ctx: self,
            widget_id: i.clone(),
        }
    }

    #[inline]
    fn get_handler<L: ContextLayer<Self>>(&mut self) -> Option<&mut L> {
        self.handler_mut().ref_of()
    }
}

pub trait ContextLayer<E>: Sized + 'static where E: Context {
    type Child: ContextLayer<E> + Sized + 'static;
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(senf: &mut E, i: &E::WidgetID, r: E::Renderer) {
        Self::Child::_render(senf,i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(senf: &mut E, i: &E::WidgetID, e: E::Event) {
        Self::Child::_event(senf,i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(senf: &mut E, i: &E::WidgetID) -> Size {
        Self::Child::_size(senf,i)
    }

    fn _child_mut(&mut self) -> &mut Self::Child;

    #[inline]
    fn ref_of<L: ContextLayer<E>>(&mut self) -> Option<&mut L> {
        if Any::is::<L>(self) {
            Any::downcast_mut::<L>(self)
        }else{
            <Self::Child as ContextLayer<E>>::ref_of( self._child_mut() )
        }
    }

    #[inline]
    fn get_self(senf: &mut E) -> Option<&mut Self> {
        senf.get_handler()
    }
}

pub trait ContextLayerStateful<E>: Sized where E: Context {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}

pub trait ContextStateful: Context where Self::Handler: ContextLayerStateful<Self> {
    #[inline] fn hovered(&self) -> Option<Self::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<Self::WidgetID> {
        None
    }

    #[inline]
    fn is_hovered(&self, i: &Self::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected(&self, i: &Self::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }
}

//TODO move this to impl module
impl<E> ContextLayer<E> for () where E: Context {
    type Child = ();
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(senf: &mut E, i: &E::WidgetID, r: E::Renderer) {
        (senf.widget_fns(i).render)(senf.link(i),r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(senf: &mut E, i: &E::WidgetID, e: E::Event) {
        (senf.widget_fns(i).event)(senf.link(i),e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(senf: &mut E, i: &E::WidgetID) -> Size {
        (senf.widget_fns(i).size)(senf.link(i))
    }
    #[inline]
    fn _child_mut(&mut self) -> &mut Self::Child {
        unreachable!()
    }
    #[inline]
    fn ref_of<L: ContextLayer<E>>(&mut self) -> Option<&mut L> {
        if Any::is::<L>(self) {
            Any::downcast_mut::<L>(self)
        }else{
            None
        }
    }

    #[inline]
    fn get_self(senf: &mut E) -> Option<&mut Self> {
        senf.get_handler()
    }
}