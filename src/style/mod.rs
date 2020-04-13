//! Style handler, Style specifier and verbs
use super::*;

pub mod font;
pub use font::*;

pub mod variant;
pub use variant::*;

pub mod color;
pub use color::*;
use std::ops::Deref;

pub trait Style<E>: Clone where E: Env, E::Backend: Backend<E,Style=Self> {
    type Font;
    type Cursor;
    type Color: Color;
    type PreprocessedText: PreprocessedText<E>;
    type PreprocessedChar: PreprocessedChar;
    type Variant: StyleVariant;

    fn font(&self, v: &Self::Variant) -> Option<&Self::Font>;
    fn cursor(&self, v: &Self::Variant) -> Self::Cursor;
    fn color(&self, v: &Self::Variant) -> Self::Color;
    
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::PreprocessedText;
    //TODO fix partial eq impl
    fn is_cached_valid(&self, s: &Self::PreprocessedText, _c: &mut E::Context) -> bool;

    fn static_default() -> Self;
}