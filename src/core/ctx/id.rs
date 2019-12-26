use super::*;
use qwutils::*;

pub trait WidgetID: Clone + PartialEq + Sized {
    #[inline]
    fn id_eq<I: WidgetID + 'static>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, |r| self.eq(r) )
    }
    
    #[inline]
    fn render<E: Env<WidgetID=Self>>(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._render::<E>(c,r) )
    }
    #[inline]
    fn event<E: Env<WidgetID=Self>>(&self, c: &mut E::Context, e: EEvent<E>) -> Result<(),()> {
        c.has_widget(self).result()
            .map(|_| self._event::<E>(c,e) )
    }
    #[inline]
    fn size<E: Env<WidgetID=Self>>(&self, c: &mut E::Context) -> Result<Size,()> {
        c.has_widget(self).result()
            .map(|_| self._size::<E>(c) )
    }

    /// PANICKS if widget doesn't exists
    #[inline]
    fn _render<E: Env<WidgetID=Self>>(&self, c: &mut E::Context, r: (&mut ERenderer<E>,&Bounds)) {
        c._render(self,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _event<E: Env<WidgetID=Self>>(&self, c: &mut E::Context, e: EEvent<E>) {
        c._event(self,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn _size<E: Env<WidgetID=Self>>(&self, c: &mut E::Context) -> Size {
        c._size(self)
    }

    #[inline]
    fn is_hovered<E: Env<WidgetID=Self>>(&self, c: &E::Context) -> bool where ECHandler<E>: AsHandlerStateful<E>, EPressedKey<E>: PressedKey<E> {
        c.state().is_hovered(self)
    }
    #[inline]
    fn is_selected<E: Env<WidgetID=Self>>(&self, c: &E::Context) -> bool where ECHandler<E>: AsHandlerStateful<E>, EPressedKey<E>: PressedKey<E> {
        c.state().is_selected(self)
    }
}