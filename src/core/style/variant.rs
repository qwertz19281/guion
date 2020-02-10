use std::iter::once;
use std::iter::Once;
/// verbs enable/disable specific parts of styles.  
/// Style implementations may ignore verbs.  
#[non_exhaustive]
#[derive(Copy,Clone)]
pub enum StyleVerb {
    ObjDefault(),
    ObjBackground(),
    ObjBorder(),
    ObjButton(),
    ObjList(),
    ObjTextBox(),
    ObjLabel(),
    ObjScroll(),
    
    DesignDefault(),
    DesignNormal(),
    DesignFlat(),

    Accent(u32),

    VariantDefault(),
    VariantNormal(),
    VariantOK(),
    VariantCaution(),
    VariantSecondary(),

    Hovered(bool),
    Selected(bool),
    Locked(bool),

    CursorDefault(),
    CursorArrow(),
    CursorIBeam(),
    CursorWait(),
    CursorCrosshair(),
    CursorWaitArrow(),
    CursorSizeNWSE(),
    CursorSizeNESW(),
    CursorSizeWE(),
    CursorSizeNS(),
    CursorSizeAll(),
    CursorNo(),
    CursorHand(),
}

impl IntoIterator for StyleVerb {
    type Item = StyleVerb;
    type IntoIter = Once<StyleVerb>;

    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}