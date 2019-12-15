use super::*;


impl<T,E,C> ILabel<E> for AsLabel<T,E,C> where C: Borrow<T> + BorrowMut<T> + 'static, T: ILabel<E>, E: Env + 'static {
    #[inline]
    fn id(&self) -> E::WidgetID {
        <T as ILabel<E>>::id(self)
    }
    
    #[inline]
    fn invalid(&self) -> bool {
        <T as ILabel<E>>::invalid(self)
    }
    #[inline]
    fn set_invalid(&mut self, v: bool) {
        <T as ILabel<E>>::set_invalid(self,v)
    }
    
    #[inline]
    fn parent(&self) -> Option<E::WidgetID> {
        <T as ILabel<E>>::parent(self)
    }
    #[inline]
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        <T as ILabel<E>>::set_parent(self,v)
    }
    #[inline]
    fn style(&self) -> &E::Style {
        <T as ILabel<E>>::style(self)
    }
}

impl<T,E,C> Widget<E> for AsLabel<T,E,C> where C: Borrow<T> + BorrowMut<T> + 'static, T: ILabel<E>, E: Env + 'static {
    crate::impl_label_inner!(AsLabel<T,E,C>,E);
}