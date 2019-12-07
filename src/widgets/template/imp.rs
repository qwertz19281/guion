use crate::core::lazout::size::Size;
use super::*;

#[macro_export]
macro_rules! impl_template {
    ($t:ty) => {
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::ITemplate<E>, E: $crate::macro_prelude::Context + 'static {
            $crate::impl_template_inner!($t,E);
        }
    };
}

#[macro_export]
macro_rules! impl_template_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::ITemplate::id(self)
        }
        #[inline]
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$c> {
            $crate::macro_prelude::HandlerFns{
                render: $crate::widgets::template::_render::<$s,$c>,
                event: $crate::widgets::template::_event::<$s,$c>,
                size: $crate::widgets::template::_size::<$s,$c>,
            }
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::ITemplate::invalid(self)
        }
        #[inline]
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::ITemplate::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::ITemplate::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::ITemplate::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=<$c>::WidgetID> + 'a> {
            Box::new(
                std::iter::empty()
            )
        }

        #[inline]
        fn childs_vec<'a>(&'a self) -> Vec<<$c>::WidgetID> {
            std::vec![]
        }
        #[inline]
        fn selectable(&self) -> bool {
            false
        }
        #[inline]
        fn has_childs(&self) -> bool {
            false
        }
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: ITemplate<E> + 'static, E: Context + 'static>(mut l: Link<E>, r: &mut E::Renderer) {
    unimplemented!()
}

pub fn _event<W: ITemplate<E> + 'static, E: Context + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: ITemplate<E> + 'static, E: Context + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}