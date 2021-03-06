//! Widgets are interfaced in two Traits for immutable and mutable operations  
//! The Traits features interface for queuering e.g. id or style, and also accessing or resolving child widgets  
//! Note that some functions in the traits are not meant to be called from externel, but over `Link`'s methods  
use super::*;
use std::any::{TypeId, type_name};
use traitcast::TraitObject;

pub mod link;
pub mod as_widget;
#[doc(hidden)]
pub mod cast;
pub mod ext;
#[doc(hidden)]
pub mod imp;
pub mod resolved;
pub mod resolvable;
pub mod root;
pub mod array;
pub mod ident;

/// Core Trait of guion ™️
pub trait Widget<E>: WBase<E> where E: Env + 'static {
    fn id(&self) -> E::WidgetID;

    /// ![RENDER](https://img.shields.io/badge/-render-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![RENDER](https://img.shields.io/badge/-render-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::render`](Link::render)
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>);
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::event`](Link::event)
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp;
    /// ![LAYOUT](https://img.shields.io/badge/-layout-000?style=flat-square)
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  
    /// ![LAYOUT](https://img.shields.io/badge/-layout-000?style=flat-square)
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square)
    /// generally not called directly, rather through [`Link::size`](Link::size)
    fn _size(&self, l: Link<E>, e: &EStyle<E>) -> ESize<E>;

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn childs(&self) -> usize;
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()>;
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn into_child<'s>(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> where Self: 's;

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    #[deprecated]
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> {
        (0..self.childs())
            .map(#[inline] |i| self.child(i).unwrap() )
            .collect::<Vec<_>>()
    }
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w;
    
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    #[deprecated]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (0..self.childs())
            .map(#[inline] |i| self.child(i).unwrap().in_parent_path(own_path.refc()) )
            .collect::<Vec<_>>()
    }

    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget  
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but throught [`Widgets::widget`](Widgets::widget)
    #[inline]
    fn resolve<'s>(&'s self, i: E::WidgetPath) -> Result<Resolvable<'s,E>,GuionError<E>> {
        if i.is_empty() {
            return Ok(Resolvable::Widget(self.box_ref()))
        }
        let (c,sub) = self.resolve_child(&i)?;
        self.child(c).unwrap().resolve_child(sub)
    }
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget  
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but throught [`Widgets::widget`](Widgets::widget)
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,GuionError<E>> where Self: 'w {
        if i.is_empty() {
            return Ok(Resolvable::Widget(self.box_box()))
        }
        let (c,sub) = self.resolve_child(&i)?;
        self.into_child(c).unwrap_nodebug().resolve_child(sub)
    }
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// to (or through) which child path would the given sub_path resolve?
    /// returns the child index and the subpath inside the child widget to resolve further
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but throught [`Widgets::widget`](Widgets::widget)
    #[inline]
    fn resolve_child(&self, sub_path: &E::WidgetPath) -> Result<(usize,E::WidgetPath),GuionError<E>> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        for c in 0..self.childs() {
            if let Some(r) = self.child(c).unwrap().resolved_by_path(sub_path) {
                return Ok((c,r.sub_path));
            }
        }
        Err(self.gen_diag_error_resolve_fail(sub_path, "resolve"))
    }
    /// ![LAYOUT](https://img.shields.io/badge/-resolving-000?style=flat-square)
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,GuionError<E>> {
        if i.is_empty() {
            return Ok(*b)
        }
        let (child,_) = self.resolve_child(&i)?;
        let bounds = self.child_bounds(l,b,e,force)?;
        
        Ok(bounds[child])
    }
    /// ![LAYOUT](https://img.shields.io/badge/-resolving-000?style=flat-square)
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()>;
    
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// attach widget's id to the given parent path
    #[inline]
    #[deprecated]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        parent.for_child_widget_id(self.id())
    }
    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// Refer [`WidgetPath::_resolves_thru`](WidgetPath::_resolves_thru)
    /// [`sub_path`]: subpath in parent widget (which contains this widget as child) which would probably resolve to/through this widget
    #[deprecated]
    fn resolved_by_path(&self, sub_path: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        E::WidgetPath::resolves_thru_child_id(self.id(), sub_path)
    }

    /// if the widget should be focusable.  
    /// regularly true for interactive widgets, false for layouts.
    fn focusable(&self) -> bool;
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        self.focusable()
    }
    //if tab/shift-tab should tabulate away from this widget
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        true
    }
    
    #[inline]
    fn inner(&self) -> Option<&dyn Widget<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn Widget<E>+'w)> where Self: 'w
        None //TODO fix inner mechanism AsWidget
    }
    #[inline]
    fn innest(&self) -> Option<&dyn Widget<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn Widget<E>+'w)> where Self: 'w
        let mut i = self.erase();
        loop {
            let v = i.inner();
            if let Some(v) = v {
                i = v;
            }else{
                return Some(i);
            }
        }
    }

    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        dest.push(self.type_name());
    }
    fn debugged_type_name(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        self.debug_type_name(&mut v);
        v.shrink_to_fit();
        v
    }

    /// ![TRAITCAST](https://img.shields.io/badge/-traitcast-000?style=flat-square)  
    /// The impl_traitcast! macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        None
    }

    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// box reference of this widget. Use [WidgetMut::box_mut] to box into mutable [WidgetRef]
    #[inline]
    fn box_ref<'s>(&'s self) -> WidgetRef<'s,E> {
        WBase::_box_ref(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// move widget into box. Use [WidgetMut::box_box_mut] to box into mutable [WidgetRef]
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        WBase::_box_box(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// move widget into box. Use [WidgetMut::boxed_mut] to box into mutable [WidgetRef]
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        WBase::_boxed(self)
    }

    #[inline(never)]
    fn gen_diag_error_resolve_fail(&self, sub_path: &E::WidgetPath, op: &'static str) -> GuionError<E> {
        /*
        Failed to resolve 3/5/2 in Pane<Vec<WidgetRefMut<E>>>
            Child #0: 6 Button<E,Label<&str>>
            Child #1: 8 CheckBox<E,Label<&str>>
            Child #2: 4 ProgressBar<E>
        */
        /*
        Traitcast(_mut) from Label<E,&str> to ICheckBox<E> not implemented (strip "dyn " prefix)
        */
        let widget_type = self.debugged_type_name();
        let child_info = (0..self.childs())
            .map(#[inline] |i| self.child(i).unwrap().guion_resolve_error_child_info(i) )
            .collect::<Vec<_>>();
        GuionError::ResolveError(Box::new(ResolveError{
            op,
            sub_path: sub_path.clone(),
            widget_type,
            child_info,
        }))
    }
}

pub trait WidgetMut<E>: Widget<E> + WBaseMut<E> where E: Env + 'static {
    /// ![EVENT](https://img.shields.io/badge/-event-000?style=flat-square)  
    /// an alternative way to pass mutations. See [Link::Message]
    #[allow(unused)]
    #[inline]
    fn message(&mut self, m: E::Message) {

    }

    #[allow(unused)]
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        
    }

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()>;
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn into_child_mut<'w>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w;

    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    #[deprecated]
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>>;
    /// ![CHILDS](https://img.shields.io/badge/-childs-000?style=flat-square)
    fn into_childs_mut<'w>(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> where Self: 'w;

    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget  
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but throught [`Widgets::widget`](Widgets::widget)
    #[inline]
    fn resolve_mut<'s>(&'s mut self, i: E::WidgetPath) -> Result<ResolvableMut<'s,E>,GuionError<E>> { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        if i.is_empty() {
            return Ok(ResolvableMut::Widget(self.box_mut()))
        }
        let (c,sub) = self.resolve_child(&i)?;
        self.child_mut(c).unwrap().resolve_child_mut(sub)
    }

    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// resolve a deep child item by the given relative path  
    /// an empty path will resolve to this widget  
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but throught [`Widgets::widget`](Widgets::widget)
    #[inline]
    fn into_resolve_mut<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,GuionError<E>> where Self: 'w {
        if i.is_empty() {
            return Ok(ResolvableMut::Widget(self.box_box_mut()))
        }
        let (c,sub) = self.resolve_child(&i)?;
        self.into_child_mut(c).unwrap_nodebug().resolve_child_mut(sub)
    }

    /// ![RESOLVING](https://img.shields.io/badge/-resolving-000?style=flat-square)  
    /// to (or through) which child path would the given sub_path resolve?
    /// returns the child index and the subpath inside the child widget to resolve further
    /// ![USER](https://img.shields.io/badge/-user-0077ff?style=flat-square) generally not used directly, but throught [`Widgets::widget`](Widgets::widget)
    #[inline]
    fn resolve_child_mut(&mut self, sub_path: &E::WidgetPath) -> Result<(usize,E::WidgetPath),GuionError<E>> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        for c in 0..self.childs() {
            if let Some(r) = self.child(c).unwrap().resolved_by_path(sub_path) {
                return Ok((c,r.sub_path));
            }
        }
        Err(self.gen_diag_error_resolve_fail_mut(sub_path,"resolve_mut"))
    }

    #[inline]
    fn inner_mut(&mut self) -> Option<&mut dyn WidgetMut<E>> {
        None
    }

    #[inline]
    fn pass(self) -> Self where Self: Sized {
        self
    }

    fn debug_type_name_mut(&mut self, dest: &mut Vec<&'static str>) {
        dest.push(self.type_name());
    }
    fn debugged_type_name_mut(&mut self) -> Vec<&'static str> {
        let mut v = Vec::new();
        self.debug_type_name_mut(&mut v);
        v.shrink_to_fit();
        v
    }

    /// ![TRAITCAST](https://img.shields.io/badge/-traitcast-000?style=flat-square)  
    /// The impl_traitcast_mut! macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_mut(&mut self, t: TypeId) -> Option<TraitObject> {
        None
    }

    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// box mut reference of this widget
    #[inline]
    fn box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> {
        WBaseMut::_box_mut(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// move widget into box
    #[inline]
    fn box_box_mut<'w>(self: Box<Self>) -> WidgetRefMut<'w,E> where Self: 'w {
        WBaseMut::_box_box_mut(self)
    }
    /// ![BOXING](https://img.shields.io/badge/-boxing-000?style=flat-square)  
    /// move widget into box
    #[inline]
    fn boxed_mut<'w>(self) -> WidgetRefMut<'w,E> where Self: Sized+'w {
        WBaseMut::_boxed_mut(self)
    }

    #[inline(never)]
    fn gen_diag_error_resolve_fail_mut(&mut self, sub_path: &E::WidgetPath, op: &'static str) -> GuionError<E> {
        let widget_type = self.debugged_type_name_mut();
        let child_info = (0..self.childs())
            .map(#[inline] |i| self.child_mut(i).unwrap().guion_resolve_error_child_info(i) )
            .collect::<Vec<_>>();
        GuionError::ResolveError(Box::new(ResolveError{
            op,
            sub_path: sub_path.clone(),
            widget_type,
            child_info,
        }))
    }
}

/// this trait is blanket implemented for all widget and provides functions which require compile-time knowledge of types
#[doc(hidden)]
pub trait WBase<E> where E: Env {
    fn type_name(&self) -> &'static str;
    fn erase(&self) -> &dyn Widget<E>;
    fn _box_ref<'s>(&'s self) -> WidgetRef<'s,E>;
    fn _box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w;
    fn _boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w;
}
impl<T,E> WBase<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    #[inline]
    fn erase(&self) -> &dyn Widget<E> {
        self
    }
    #[inline]
    fn _box_ref<'s>(&'s self) -> WidgetRef<'s,E> {
        Box::new(self.erase())
    }
    #[inline]
    fn _box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        self
    }
    #[inline]
    fn _boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized + 'w {
        Box::new(self)
    }
}

/// this trait is blanket implemented for all widget and provides functions which require compile-time knowledge of types
#[doc(hidden)]
pub trait WBaseMut<E> where E: Env {
    fn base(&self) -> &dyn Widget<E>;
    fn erase_mut(&mut self) -> &mut dyn WidgetMut<E>;
    fn _box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E>;
    fn _box_box_mut<'w>(self: Box<Self>) -> WidgetRefMut<'w,E> where Self: 'w;
    fn _boxed_mut<'w>(self) -> WidgetRefMut<'w,E> where Self: Sized+'w;
}
impl<T,E> WBaseMut<E> for T where T: WidgetMut<E>, E: Env {
    #[inline]
    fn base(&self) -> &dyn Widget<E> {
        self
    }
    #[inline]
    fn erase_mut(&mut self) -> &mut dyn WidgetMut<E> {
        self
    }
    #[inline]
    fn _box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> {
        Box::new(self.erase_mut())
    }
    #[inline]
    fn _box_box_mut<'w>(self: Box<Self>) -> WidgetRefMut<'w,E> where Self: 'w {
        self
    }
    #[inline]
    fn _boxed_mut<'w>(self) -> WidgetRefMut<'w,E> where Self: Sized + 'w {
        Box::new(self)
    }
}
