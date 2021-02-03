//! Unique indentifier for widgets
use super::*;
use std::hash::Hash;

pub mod standard;

pub trait WidgetID: AsRefMut<Self> + Clone + PartialEq + Eq + Sized + Hash + 'static {
    #[inline]
    fn id_eq<I: WidgetID + 'static>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, #[inline] |r| self.eq(r) )
    }

    #[inline]
    fn is_hovered<E: Env>(&self, c: &E::Context) -> bool where E::Context: CtxStdState<E>, EPressedKey<E>: PressedKey<E>, Self: AsRefMut<E::WidgetID> {
        c.state().is_hovered(self.as_ref())
    }
    #[inline]
    fn is_focused<E: Env>(&self, c: &E::Context) -> bool where E::Context: CtxStdState<E>, EPressedKey<E>: PressedKey<E>, Self: AsRefMut<E::WidgetID> {
        c.state().is_focused(self.as_ref())
    }
}

pub trait WidgetIDAlloc: WidgetID {
    fn new_id() -> Self where Self: Sized;
}

/*impl WidgetID for Vec<Box<dyn Any>> {
    
}*/
