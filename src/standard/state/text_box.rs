//TODO implement this stuff

use crate::core::ctx::aliases::*;
use crate::core::ctx::*;
use crate::core::style::font::PreprocessedText;

pub struct TextBoxState<E> where
    E: Env,
    ECHLink<E>: AsHandlerStateful<E>,
    ECHLink<E>: for<'a> AsHandler<'a,ECStateful<E>>,
{
    pp: EPPText<E>,
    id: E::WidgetID,
}

/*impl ITextBox { TODO impl
    /// So you are the currently cached/selected TextBox any you invalidated, so please fix the state
    /// return if revalidating was successful, else state will be resetted
    /// 
    /// This function exists to make selections of mutating textboxes (e.g. CRDTs) maintable
    fn revalidate_state(&mut self, s: &mut Option<TextBoxState>) -> bool {
        false
    }
}*/