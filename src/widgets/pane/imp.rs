use crate::core::lazout::TOrientation;
use crate::core::lazout::size::Size;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::env::Env;

pub struct Pane<E,O> where E: Env, O: TOrientation {
    id: E::WidgetID,
    childs: Vec<E::WidgetID>,
    invalid: bool,
    parent: Option<E::WidgetID>,
}

impl<E,O> super::Pane<E> for Pane<E,O> where E: Env + 'static, O: TOrientation {
    type O=O;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }

    fn childs(&self) -> &[E::WidgetID] {
        &self.childs[..]
    }

    fn invalid(&self) -> bool {
        self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        self.invalid = v;
    }
    
    fn size(&self) -> Size {
        unimplemented!()
    }

    fn parent(&self) -> Option<&E::WidgetID> {
        self.parent.as_ref()
    }
    fn set_parent(&mut self, v: Option<E::WidgetID>) {
        self.parent=v;
    }
}