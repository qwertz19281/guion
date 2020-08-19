use super::*;

impl<'w,E,T,Stil> Widget<'w,E> for Pane<'w,E,T,Stil> where
    E: Env,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    T: WidgetArray<'w,E>+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        self._render_impl(l,r)
    }
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        self._event_direct_impl(l,e)
    }
    fn _size(&self, l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        self._size_impl(l,e)
    }
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()> {
        self.child_bounds_impl(l,b,e,force)
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        self.childs.childs()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        self.childs.into_childs()
    }

    fn focusable(&self) -> bool {
        false
    }

    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        self.childs.child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        self.childs.into_child(i)
    }
}
impl<'w,E,T,Stil> WidgetMut<'w,E> for Pane<'w,E,T,Stil> where 
    E: Env,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    T: WidgetArrayMut<'w,E>+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn _set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        self.childs.childs_mut()
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        self.childs.into_childs_mut()
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        self.childs.child_mut(i)
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        self.childs.into_child_mut(i)
    }
}

impl<'w,E,T,Stil> Pane<'w,E,T,Stil> where
    E: Env,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    T: WidgetArray<'w,E>+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    pub fn _render_impl(&self, mut l: Link<E>, r: &mut RenderLink<E>) where
        E: Env,
    {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdTag::BorderOuter,l.ctx);
        let sizes = l.child_sizes(r.style()).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&r.bounds().size,&sizes,self.orientation); 

        for i in 0..self.childs() {
            let l = l.for_child(i).expect("Dead Path Inside Pane");
            let mut r = r.slice(&bounds[i]);
            r.render_widget(l);
        }
        //TODO FIX viewport
    }

    pub fn _event_direct_impl(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp where
        E: Env,
    {
        let e = try_or_false!(e.filter_bounds_by_border(l.style_provider(),StdTag::BorderOuter));
        
        let sizes = l.child_sizes(&e.style).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&e.bounds.size,&sizes,self.orientation);

        let mut passed = false;

        for i in 0..self.childs() {
            let mut l = l.for_child(i).expect("Dead Path Inside Pane");
            let sliced = e.slice_bounds(&bounds[i]);
            if let Some(ee) = sliced.filter(&l) {
                passed |= l.event_direct(&ee);
            }
        }

        passed
    }

    pub fn _size_impl(&self, mut l: Link<E>, e: &ESVariant<E>) -> ESize<E> where
        E: Env,
    {
        let mut s = ESize::<E>::empty();
        l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(e), self.orientation) ).expect("Dead Path inside Pane");
        s
    }

    pub fn child_bounds_impl(&self, mut l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()> where
        E: Env,
    {
        let sizes = l.child_sizes(e).expect("Dead Path Inside Pane");
        let bounds = calc_bounds(&b.size,&sizes,self.orientation); 

        Ok(bounds)
    }
}
