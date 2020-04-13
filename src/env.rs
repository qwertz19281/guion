//! The Env type defines a compound over any generic types
use super::*;

/// Type compound
pub trait Env: Sized + Clone + PartialEq + Send + Sync + 'static {
    type Backend: Backend<Self>;
    type Context: Context<Self>;
    type Storage: Widgets<Self>;
    type WidgetID: WidgetID;
    type WidgetPath: WidgetPath<Self>;
    type ValidState: ValidState;
    //type Commit: Eq + Ord;
}

pub trait EnvFlexStyleVariant: Env {
    type StyleVariant: StyleVariant;
}
pub trait EnvFlexCtxHandler: Env {
    type CtxHandler: Handler<Self>;
}


pub trait ValidState {
    fn valid() -> Self;
    fn rerender(&self) -> bool;
    fn relayout(&self) -> bool;
}