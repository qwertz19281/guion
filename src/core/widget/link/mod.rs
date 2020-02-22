use crate::core::ctx::widgets::Widgets;
use std::ops::DerefMut;
use std::ops::Deref;
use super::*;

pub mod imp;
use imp::*;

/// holds a immutable reference to the current widget and the widget tree, also a mutable reference to the context
pub struct Link<'c,E> where E: Env {
    pub ctx: &'c mut E::Context,
    pub widget: Resolved<'c,E>,
}

impl<'c,E> Link<'c,E> where E: Env {
    /// enqueue mutable access to this widget
    #[inline] 
    pub fn mutate(&mut self, f: fn(&mut E::DynWidget), invalidate: bool) {
        self.ctx.queue_mut().enqueue_widget_mut(self.widget.path.slice(),f,invalidate)
    }
    /// enqueue mutable access to this widget
    #[inline] 
    pub fn mutate_closure(&mut self, f: impl FnOnce(&mut E::DynWidget), invalidate: bool) {
        self.ctx.queue_mut().enqueue_widget_mut_closure(self.widget.path.slice(),Box::new(f),invalidate)
    }
    /// enqueue immutable access to this widget
    #[inline] 
    pub fn later(&mut self, f: fn(&E::DynWidget)) {
        self.ctx.queue_mut().enqueue_widget(self.widget.path.slice(),f)
    }
    /// enqueue immutable access to this widget
    #[inline] 
    pub fn later_closure(&mut self, f: impl FnOnce(&E::DynWidget)) {
        self.ctx.queue_mut().enqueue_widget_closure(self.widget.path.slice(),Box::new(f))
    }
    #[inline]
    pub fn enqueue_invalidate(&mut self) {
        let s = self.widget.path.slice();
        self.ctx.queue_mut().enqueue_widget_invalidate(s)
    }
    /// mark the current widget as validated
    /// this should and should only be called from widget's render fn
    #[inline]
    pub fn enqueue_validate(&mut self) {
        let s = self.widget.path.slice();
        self.ctx.queue_mut().enqueue_widget_validate(s)
    }
    #[inline]
    pub fn w(&self) -> &E::DynWidget {
        &self.widget
    }

    #[inline]
    pub fn widget(&self) -> Resolved<'c,E> {
        self.widget.clone()
    }

    #[inline]
    pub fn id(&self) -> E::WidgetID {
        self.widget.id()
    }

    /*#[inline]
    pub fn for_child<'s>(&'s self, child: &'s E::DynWidget) -> Link<'s> where 'c: 's {
        
    }*/

    #[inline]
    pub fn render(&mut self, r: &mut RenderLink<E>) -> bool {
        self.ctx.render(self.widget(),r)
    }
    #[inline]
    pub fn event(&mut self, e: (EEvent<E>,&Bounds)) {
        self.ctx.event(self.widget(),e)
    }
    #[inline]
    pub fn size(&mut self) -> ESize<E> {
        self.ctx.size(self.widget())
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _render(&mut self, r: &mut RenderLink<E>) -> bool {
        let w = self.ctx.link(self.widget.clone());
        self.widget.wref.widget().render(w,r)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _event(&mut self, e: (EEvent<E>,&Bounds)) {
        let w = self.ctx.link(self.widget.clone());
        self.widget.wref.widget().event(w,e)
    }
    /// bypasses Context and Handler(s)
    #[inline]
    pub fn _size(&mut self) -> ESize<E> {
        let w = self.ctx.link(self.widget.clone());
        self.widget.wref.widget().size(w)
    }

    #[inline]
    pub fn is_hovered(&self) -> bool where ECHandler<E>: AsHandlerStateful<E> {
        self.ctx.state().is_hovered(&self.id())
    }
    #[inline]
    pub fn is_selected(&self) -> bool where ECHandler<E>: AsHandlerStateful<E> {
        self.ctx.state().is_selected(&self.id())
    }

    #[inline]
    pub fn child_paths(&self) -> Vec<E::WidgetPath> {
        self.widget.child_paths()
    }

    //THIS IS AN ULTRA HACK
    //(as shortening teh lifetime even more aggresse we MAY can put an iterator on it)
    #[inline]
    pub fn for_childs<'s>(&'s mut self, f: &'s mut impl FnMut(Link<E>)) -> Result<(),()> where 'c: 's {
        let wref = self.widget.wref.refc();
        let path = self.widget.path.refc();
        let ch = self.widget.childs();
        let stor = self.widget.stor;
        for c in ch {
            let w = c.resolve_widget(stor)?;
            let w = Resolved{
                wref: w,
                path: wref.widget().self_in_parent(path.slice()).into(),
                stor,
            };
            let l = Link{
                widget: short_resolved(w),
                ctx: self.ctx,
            };
            f(l);
        }
        Ok(())
    }

    /*
    /// iterate over childs
    #[inline]
    pub fn childs(&'c self, predicate: impl Fn(WPSlice<'c,E>)->bool + 'c ) -> impl Iterator<Item=&'c E::DynWidget> + 'c {
        self.ctx.widget(self.path).unwrap()
            .child_paths(self.path)
            .into_iter()
            .filter(#[inline] move |s| predicate(s.slice()) )
            .map(move |e| {
                (
                    self.ctx.widget(e.slice()).expect("Lost Child")
                )
            })
    }
    /// iterate over childs mut
    #[inline]
    pub fn childs_mut(&'c mut self, mut f: impl FnMut(&mut E::DynWidget), mut predicate: impl FnMut(&E::WidgetPath)->bool) {
        let childs: Vec<E::WidgetPath> = self.ctx.widget(self.path).unwrap().child_paths(self.path);

        for e in childs {
            if predicate(&e) {
                f(
                    self.ctx.widget_mut(e.slice()).expect("Lost Child")
                );
            }
        }
    }*/
    /// iterate from current up to the root element
    #[inline]
    pub fn parents(&'c self) -> Parents<'c,E> {
        Parents{
            stor: self.widget.stor,
            next: Some(self.widget.path.slice()),
        }
    }
    
    pub fn with_ctx<F: Env<WidgetPath=E::WidgetPath,Storage=E::Storage>>(self, ctx: &'c mut F::Context) -> Link<'c,F> where E::WidgetPath: WidgetPath<F,SubPath=EWPSub<E>,RcPath=EWPRc<E>>, EWPSub<E>: SubPath<F>, E::Storage: Widgets<F> {
        Link{
            widget: self.widget.with_env::<F>(),
            ctx,
        }
    }
    #[inline]
    pub fn enqueue<I>(&'c mut self, i: I) where ECQueue<E>: Enqueue<E,I> {
        self.ctx.queue_mut().enqueue(i)
    }
}

impl<'a,E> Deref for Link<'a,E> where E: Env {
    type Target = E::Context;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
impl<'a,E> DerefMut for Link<'a,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}