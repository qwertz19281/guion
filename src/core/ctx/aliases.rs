use crate::core::util::border::Border;
use crate::core::*;
use ctx::*;
use style::Style;

pub type ERenderer<E: Env> = <E::Backend as Backend<E>>::Renderer;
pub type EEvent<E: Env> = <E::Backend as Backend<E>>::Event;
pub type EEventDest<E: Env> = <E::Backend as Backend<E>>::EventDest;
pub type EEventKey<E: Env> = <E::Backend as Backend<E>>::EventKey;
pub type EEventConsuming<E: Env> = <E::Backend as Backend<E>>::EventConsuming;
pub type EStyle<E: Env> = <E::Backend as Backend<E>>::Style;

pub type ESPPText<E: Env> = <EStyle<E> as Style<E>>::PreprocessedText;
pub type ESPPChar<E: Env> = <EStyle<E> as Style<E>>::PreprocessedText;
pub type ESFont<E: Env> = <EStyle<E> as Style<E>>::Font;
pub type ESColor<E: Env> = <EStyle<E> as Style<E>>::Color;
pub type ESCursor<E: Env> = <EStyle<E> as Style<E>>::Cursor;
pub type ECHandler<E: Env> = <E::Context as Context>::Handler;
pub type ECStateful<E: Env> = <ECHandler<E> as AsHandlerStateful<E>>::T;
pub type EPressedKey<E: Env> = <ECStateful<E> as HandlerStateful<E>>::K;

#[inline]
pub fn e_default_style<E: Env>() -> &'static EStyle<E> {
    <EStyle<E> as Style<E>>::default()
}
#[inline]
pub fn e_default_border<E: Env>() -> &'static Border {
    <EStyle<E> as Style<E>>::default_border()
}