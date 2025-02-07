// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`CellArea`](crate::CellArea).

use crate::subclass::prelude::*;
use crate::{
    CellArea, CellAreaContext, CellRenderer, CellRendererState, DirectionType, SizeRequestMode,
    Snapshot, TreeIter, TreeModel, Widget,
};
use glib::translate::*;
use glib::{Cast, IsA, ParamSpec, Value};
use std::mem;

#[derive(Debug)]
pub struct CellCallback {
    callback: ffi::GtkCellCallback,
    user_data: glib::ffi::gpointer,
}

impl CellCallback {
    pub fn call<R: IsA<CellRenderer>>(&self, cell_renderer: &R) -> bool {
        unsafe {
            if let Some(callback) = self.callback {
                from_glib(callback(
                    cell_renderer.as_ref().to_glib_none().0,
                    self.user_data,
                ))
            } else {
                // true to stop iterating over cells
                true
            }
        }
    }
}

#[derive(Debug)]
pub struct CellCallbackAllocate {
    callback: ffi::GtkCellAllocCallback,
    user_data: glib::ffi::gpointer,
}

impl CellCallbackAllocate {
    pub fn call<R: IsA<CellRenderer>>(
        &self,
        cell_renderer: &R,
        cell_area: &gdk::Rectangle,
        cell_background: &gdk::Rectangle,
    ) -> bool {
        unsafe {
            if let Some(callback) = self.callback {
                from_glib(callback(
                    cell_renderer.as_ref().to_glib_none().0,
                    cell_area.to_glib_none().0,
                    cell_background.to_glib_none().0,
                    self.user_data,
                ))
            } else {
                // true to stop iterating over cells
                true
            }
        }
    }
}

pub trait CellAreaImpl: CellAreaImplExt + ObjectImpl {
    fn cell_properties() -> &'static [ParamSpec] {
        &[]
    }

    fn set_cell_property<R: IsA<CellRenderer>>(
        &self,
        _obj: &Self::Type,
        _renderer: &R,
        _id: usize,
        _value: &Value,
        _pspec: &ParamSpec,
    ) {
        unimplemented!()
    }

    fn cell_property<R: IsA<CellRenderer>>(
        &self,
        _obj: &Self::Type,
        _renderer: &R,
        _id: usize,
        _pspec: &ParamSpec,
    ) -> Value {
        unimplemented!()
    }

    fn activate<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        area: &gdk::Rectangle,
        flags: CellRendererState,
        edit_only: bool,
    ) -> bool {
        self.parent_activate(cell_area, context, widget, area, flags, edit_only)
    }

    fn add<R: IsA<CellRenderer>>(&self, cell_area: &Self::Type, renderer: &R) {
        self.parent_add(cell_area, renderer)
    }

    fn apply_attributes<M: IsA<TreeModel>>(
        &self,
        cell_area: &Self::Type,
        tree_model: &M,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    ) {
        self.parent_apply_attributes(cell_area, tree_model, iter, is_expander, is_expanded)
    }

    fn create_context(&self, cell_area: &Self::Type) -> Option<CellAreaContext> {
        self.parent_create_context(cell_area)
    }

    fn copy_context<P: IsA<CellAreaContext>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
    ) -> Option<CellAreaContext> {
        self.parent_copy_context(cell_area, context)
    }

    fn event<W: IsA<Widget>, P: IsA<CellAreaContext>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        event: &gdk::Event,
        area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        self.parent_event(cell_area, context, widget, event, area, flags)
    }

    fn foreach(&self, cell_area: &Self::Type, callback: &CellCallback) {
        self.parent_foreach(cell_area, callback);
    }

    fn foreach_alloc<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_type: &Self::Type,
        context: &P,
        widget: &W,
        area: &gdk::Rectangle,
        bg_area: &gdk::Rectangle,
        callback: &CellCallbackAllocate,
    ) {
        self.parent_foreach_alloc(cell_type, context, widget, area, bg_area, callback)
    }

    fn remove<R: IsA<CellRenderer>>(&self, cell_area: &Self::Type, renderer: &R) {
        self.parent_remove(cell_area, renderer)
    }

    fn is_activatable(&self, cell_area: &Self::Type) -> bool {
        self.parent_is_activatable(cell_area)
    }

    fn focus(&self, cell_area: &Self::Type, direction_type: DirectionType) -> bool {
        self.parent_focus(cell_area, direction_type)
    }

    fn request_mode(&self, cell_area: &Self::Type) -> SizeRequestMode {
        self.parent_request_mode(cell_area)
    }

    fn preferred_width<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
    ) -> (i32, i32) {
        self.parent_preferred_width(cell_area, context, widget)
    }

    fn preferred_width_for_height<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        height: i32,
    ) -> (i32, i32) {
        self.parent_preferred_width_for_height(cell_area, context, widget, height)
    }

    fn preferred_height<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
    ) -> (i32, i32) {
        self.parent_preferred_height(cell_area, context, widget)
    }

    fn preferred_height_for_width<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        width: i32,
    ) -> (i32, i32) {
        self.parent_preferred_height_for_width(cell_area, context, widget, width)
    }

    fn snapshot<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        snapshot: &Snapshot,
        widget: &W,
        background_area: &gdk::Rectangle,
        cellarea: &gdk::Rectangle,
        flags: CellRendererState,
        paint_focus: bool,
    ) {
        self.parent_snapshot(
            cell_area,
            context,
            snapshot,
            widget,
            background_area,
            cellarea,
            flags,
            paint_focus,
        );
    }
}

pub trait CellAreaImplExt: ObjectSubclass {
    fn parent_activate<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        area: &gdk::Rectangle,
        flags: CellRendererState,
        edit_only: bool,
    ) -> bool;

    fn parent_add<R: IsA<CellRenderer>>(&self, cell_area: &Self::Type, renderer: &R);

    fn parent_apply_attributes<M: IsA<TreeModel>>(
        &self,
        cell_area: &Self::Type,
        tree_model: &M,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    );
    fn parent_create_context(&self, cell_area: &Self::Type) -> Option<CellAreaContext>;

    fn parent_copy_context<P: IsA<CellAreaContext>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
    ) -> Option<CellAreaContext>;

    fn parent_event<W: IsA<Widget>, P: IsA<CellAreaContext>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        event: &gdk::Event,
        area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;

    fn parent_foreach(&self, cell_area: &Self::Type, callback: &CellCallback);

    fn parent_foreach_alloc<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_type: &Self::Type,
        context: &P,
        widget: &W,
        area: &gdk::Rectangle,
        bg_area: &gdk::Rectangle,
        callback: &CellCallbackAllocate,
    );

    fn parent_remove<R: IsA<CellRenderer>>(&self, cell_area: &Self::Type, renderer: &R);

    fn parent_is_activatable(&self, cell_area: &Self::Type) -> bool;

    fn parent_focus(&self, cell_area: &Self::Type, direction_type: DirectionType) -> bool;

    fn parent_request_mode(&self, cell_area: &Self::Type) -> SizeRequestMode;

    fn parent_preferred_width<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
    ) -> (i32, i32);

    fn parent_preferred_height<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
    ) -> (i32, i32);

    fn parent_preferred_width_for_height<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        height: i32,
    ) -> (i32, i32);

    fn parent_preferred_height_for_width<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        width: i32,
    ) -> (i32, i32);

    fn parent_snapshot<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        snapshot: &Snapshot,
        widget: &W,
        background_area: &gdk::Rectangle,
        cellarea: &gdk::Rectangle,
        flags: CellRendererState,
        paint_focus: bool,
    );
}

impl<T: CellAreaImpl> CellAreaImplExt for T {
    fn parent_activate<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        area: &gdk::Rectangle,
        flags: CellRendererState,
        edit_only: bool,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).activate {
                from_glib(f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    context.as_ref().to_glib_none().0,
                    widget.as_ref().to_glib_none().0,
                    area.to_glib_none().0,
                    flags.into_glib(),
                    edit_only.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_add<R: IsA<CellRenderer>>(&self, cell_area: &Self::Type, renderer: &R) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).add {
                f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    renderer.as_ref().to_glib_none().0,
                )
            }
        }
    }

    fn parent_apply_attributes<M: IsA<TreeModel>>(
        &self,
        cell_area: &Self::Type,
        tree_model: &M,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).apply_attributes {
                f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    tree_model.as_ref().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                    is_expander.into_glib(),
                    is_expanded.into_glib(),
                )
            }
        }
    }

    fn parent_create_context(&self, cell_area: &Self::Type) -> Option<CellAreaContext> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class)
                .create_context
                .expect("No parent class impl for \"create_context\"");

            let ret = f(cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0);
            Some(from_glib_full(ret))
        }
    }

    fn parent_copy_context<P: IsA<CellAreaContext>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
    ) -> Option<CellAreaContext> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class)
                .copy_context
                .expect("No parent class impl for \"copy_context\"");

            let ret = f(
                cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            );
            Some(from_glib_full(ret))
        }
    }

    fn parent_event<W: IsA<Widget>, P: IsA<CellAreaContext>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        event: &gdk::Event,
        area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).event {
                from_glib(f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    context.as_ref().to_glib_none().0,
                    widget.as_ref().to_glib_none().0,
                    event.to_glib_none().0,
                    area.to_glib_none().0,
                    flags.into_glib(),
                ))
            } else {
                // returns true only if the event is handled
                false
            }
        }
    }

    fn parent_foreach(&self, cell_area: &Self::Type, callback: &CellCallback) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).foreach {
                f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    callback.callback,
                    callback.user_data,
                )
            }
        }
    }

    fn parent_foreach_alloc<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        widget: &W,
        area: &gdk::Rectangle,
        bg_area: &gdk::Rectangle,
        callback: &CellCallbackAllocate,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).foreach_alloc {
                f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    context.as_ref().to_glib_none().0,
                    widget.as_ref().to_glib_none().0,
                    area.to_glib_none().0,
                    bg_area.to_glib_none().0,
                    callback.callback,
                    callback.user_data,
                )
            }
        }
    }

    fn parent_remove<R: IsA<CellRenderer>>(&self, cell_area: &Self::Type, renderer: &R) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).remove {
                f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    renderer.as_ref().to_glib_none().0,
                )
            }
        }
    }

    fn parent_is_activatable(&self, cell_area: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).is_activatable {
                from_glib(f(cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0))
            } else {
                false
            }
        }
    }

    fn parent_focus(&self, cell_area: &Self::Type, direction_type: DirectionType) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).focus {
                from_glib(f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    direction_type.into_glib(),
                ))
            } else {
                false
            }
        }
    }

    fn parent_request_mode(&self, cell_area: &Self::Type) -> SizeRequestMode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class)
                .get_request_mode
                .expect("No parent class impl for \"get_request_mode\"");
            from_glib(f(cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0))
        }
    }

    fn parent_preferred_width<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        cell_area_context: &P,
        widget: &W,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class).get_preferred_width.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                cell_area_context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_preferred_height<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        cell_area_context: &P,
        widget: &W,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class).get_preferred_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                cell_area_context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_preferred_width_for_height<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        cell_area_context: &P,
        widget: &W,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class).get_preferred_width_for_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                cell_area_context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                height,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_preferred_height_for_width<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        cell_area_context: &P,
        widget: &W,
        width: i32,
    ) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            let f = (*parent_class).get_preferred_height_for_width.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                cell_area_context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_snapshot<P: IsA<CellAreaContext>, W: IsA<Widget>>(
        &self,
        cell_area: &Self::Type,
        context: &P,
        snapshot: &Snapshot,
        widget: &W,
        background_area: &gdk::Rectangle,
        cellarea: &gdk::Rectangle,
        flags: CellRendererState,
        paint_focus: bool,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellAreaClass;
            if let Some(f) = (*parent_class).snapshot {
                f(
                    cell_area.unsafe_cast_ref::<CellArea>().to_glib_none().0,
                    context.as_ref().to_glib_none().0,
                    widget.as_ref().to_glib_none().0,
                    snapshot.to_glib_none().0,
                    background_area.to_glib_none().0,
                    cellarea.to_glib_none().0,
                    flags.into_glib(),
                    paint_focus.into_glib(),
                )
            }
        }
    }
}

unsafe impl<T: CellAreaImpl> IsSubclassable<T> for CellArea {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        let pspecs = <T as CellAreaImpl>::cell_properties();
        if !pspecs.is_empty() {
            unsafe {
                for (prop_id, pspec) in pspecs.iter().enumerate() {
                    ffi::gtk_cell_area_class_install_cell_property(
                        klass,
                        prop_id as u32,
                        pspec.to_glib_none().0,
                    );
                }
            }
        }
        klass.activate = Some(cell_area_activate::<T>);
        klass.add = Some(cell_area_add::<T>);
        klass.apply_attributes = Some(cell_area_apply_attributes::<T>);
        klass.create_context = Some(cell_area_create_context::<T>);
        klass.copy_context = Some(cell_area_copy_context::<T>);
        klass.event = Some(cell_area_event::<T>);
        klass.foreach = Some(cell_area_foreach::<T>);
        klass.foreach_alloc = Some(cell_area_foreach_alloc::<T>);
        klass.remove = Some(cell_area_remove::<T>);
        klass.is_activatable = Some(cell_area_is_activatable::<T>);
        klass.focus = Some(cell_area_focus::<T>);
        klass.get_request_mode = Some(cell_area_get_request_mode::<T>);
        klass.get_preferred_width = Some(cell_area_get_preferred_width::<T>);
        klass.get_preferred_width_for_height = Some(cell_area_get_preferred_width_for_height::<T>);
        klass.get_preferred_height = Some(cell_area_get_preferred_height::<T>);
        klass.get_preferred_height_for_width = Some(cell_area_get_preferred_height_for_width::<T>);
        klass.snapshot = Some(cell_area_snapshot::<T>);
        klass.set_cell_property = Some(cell_area_set_cell_property::<T>);
        klass.get_cell_property = Some(cell_area_get_cell_property::<T>);
    }
}

unsafe extern "C" fn cell_area_set_cell_property<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    rendererptr: *mut ffi::GtkCellRenderer,
    id: u32,
    valueptr: *mut glib::gobject_ffi::GValue,
    pspecptr: *mut glib::gobject_ffi::GParamSpec,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    imp.set_cell_property(
        from_glib_borrow::<_, CellArea>(ptr).unsafe_cast_ref(),
        &*from_glib_borrow::<_, CellRenderer>(rendererptr),
        id as usize,
        &*(valueptr as *mut Value),
        &from_glib_borrow(pspecptr),
    );
}

unsafe extern "C" fn cell_area_get_cell_property<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    rendererptr: *mut ffi::GtkCellRenderer,
    id: u32,
    valueptr: *mut glib::gobject_ffi::GValue,
    pspecptr: *mut glib::gobject_ffi::GParamSpec,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();

    let value = imp.cell_property(
        from_glib_borrow::<_, CellArea>(ptr).unsafe_cast_ref(),
        &*from_glib_borrow::<_, CellRenderer>(rendererptr),
        id as usize,
        &from_glib_borrow(pspecptr),
    );

    // See glib::subclass::ObjectImpl::property for the reasoning behind
    glib::gobject_ffi::g_value_unset(valueptr);
    let value = mem::ManuallyDrop::new(value);
    std::ptr::write(valueptr, std::ptr::read(value.to_glib_none().0));
}

unsafe extern "C" fn cell_area_add<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    rendererptr: *mut ffi::GtkCellRenderer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let renderer: Borrowed<CellRenderer> = from_glib_borrow(rendererptr);

    imp.add(wrap.unsafe_cast_ref(), &*renderer)
}

unsafe extern "C" fn cell_area_apply_attributes<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    modelptr: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    is_expander: glib::ffi::gboolean,
    is_expanded: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let model: Borrowed<TreeModel> = from_glib_borrow(modelptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.apply_attributes(
        wrap.unsafe_cast_ref(),
        &*model,
        &iter,
        from_glib(is_expander),
        from_glib(is_expanded),
    )
}

unsafe extern "C" fn cell_area_remove<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    rendererptr: *mut ffi::GtkCellRenderer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let renderer: Borrowed<CellRenderer> = from_glib_borrow(rendererptr);

    imp.remove(wrap.unsafe_cast_ref(), &*renderer)
}

unsafe extern "C" fn cell_area_is_activatable<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);

    imp.is_activatable(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn cell_area_focus<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    directionptr: ffi::GtkDirectionType,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);

    imp.focus(wrap.unsafe_cast_ref(), from_glib(directionptr))
        .into_glib()
}

unsafe extern "C" fn cell_area_get_request_mode<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
) -> ffi::GtkSizeRequestMode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);

    imp.request_mode(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn cell_area_get_preferred_height<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    wdgtptr: *mut ffi::GtkWidget,
    minptr: *mut libc::c_int,
    natptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_size, nat_size) = imp.preferred_height(wrap.unsafe_cast_ref(), &*context, &*widget);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn cell_area_get_preferred_width<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    wdgtptr: *mut ffi::GtkWidget,
    minptr: *mut libc::c_int,
    natptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_size, nat_size) = imp.preferred_width(wrap.unsafe_cast_ref(), &*context, &*widget);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn cell_area_get_preferred_width_for_height<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    wdgtptr: *mut ffi::GtkWidget,
    height: i32,
    min_width_ptr: *mut libc::c_int,
    nat_width_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_width, nat_width) =
        imp.preferred_width_for_height(wrap.unsafe_cast_ref(), &*context, &*widget, height);
    if !min_width_ptr.is_null() {
        *min_width_ptr = min_width;
    }
    if !nat_width_ptr.is_null() {
        *nat_width_ptr = nat_width;
    }
}

unsafe extern "C" fn cell_area_get_preferred_height_for_width<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    wdgtptr: *mut ffi::GtkWidget,
    width: i32,
    min_height_ptr: *mut libc::c_int,
    nat_height_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    let (min_height, nat_height) =
        imp.preferred_height_for_width(wrap.unsafe_cast_ref(), &*context, &*widget, width);
    if !min_height_ptr.is_null() {
        *min_height_ptr = min_height;
    }
    if !nat_height_ptr.is_null() {
        *nat_height_ptr = nat_height;
    }
}

unsafe extern "C" fn cell_area_activate<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    wdgtptr: *mut ffi::GtkWidget,
    cellptr: *const gdk::ffi::GdkRectangle,
    flags: ffi::GtkCellRendererState,
    edit_only: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);

    imp.activate(
        wrap.unsafe_cast_ref(),
        &*context,
        &*widget,
        &from_glib_borrow(cellptr),
        from_glib(flags),
        from_glib(edit_only),
    )
    .into_glib()
}

unsafe extern "C" fn cell_area_snapshot<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    wdgtptr: *mut ffi::GtkWidget,
    snapshotptr: *mut ffi::GtkSnapshot,
    bgptr: *const gdk::ffi::GdkRectangle,
    cellptr: *const gdk::ffi::GdkRectangle,
    flags: ffi::GtkCellRendererState,
    paint_focus: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(wdgtptr);
    let snapshot: Borrowed<Snapshot> = from_glib_borrow(snapshotptr);

    imp.snapshot(
        wrap.unsafe_cast_ref(),
        &*context,
        &snapshot,
        &*widget,
        &from_glib_borrow(bgptr),
        &from_glib_borrow(cellptr),
        from_glib(flags),
        from_glib(paint_focus),
    )
}

unsafe extern "C" fn cell_area_create_context<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
) -> *mut ffi::GtkCellAreaContext {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);

    imp.create_context(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn cell_area_copy_context<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
) -> *mut ffi::GtkCellAreaContext {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);

    imp.copy_context(wrap.unsafe_cast_ref(), &*context)
        .to_glib_full()
}

unsafe extern "C" fn cell_area_event<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    widgetptr: *mut ffi::GtkWidget,
    eventptr: *mut gdk::ffi::GdkEvent,
    rectangleptr: *const gdk::ffi::GdkRectangle,
    flags: ffi::GtkCellRendererState,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let event: Borrowed<gdk::Event> = from_glib_borrow(eventptr);
    let rectangle: Borrowed<gdk::Rectangle> = from_glib_borrow(rectangleptr);

    imp.event(
        wrap.unsafe_cast_ref(),
        &*context,
        &*widget,
        &event,
        &rectangle,
        from_glib(flags),
    )
    .into_glib()
}

unsafe extern "C" fn cell_area_foreach<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    callback: ffi::GtkCellCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);

    let callback = CellCallback {
        callback,
        user_data,
    };

    imp.foreach(wrap.unsafe_cast_ref(), &callback)
}

unsafe extern "C" fn cell_area_foreach_alloc<T: CellAreaImpl>(
    ptr: *mut ffi::GtkCellArea,
    contextptr: *mut ffi::GtkCellAreaContext,
    widgetptr: *mut ffi::GtkWidget,
    areaptr: *const gdk::ffi::GdkRectangle,
    rectangleptr: *const gdk::ffi::GdkRectangle,
    callback: ffi::GtkCellAllocCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CellArea> = from_glib_borrow(ptr);
    let context: Borrowed<CellAreaContext> = from_glib_borrow(contextptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let rectangle: Borrowed<gdk::Rectangle> = from_glib_borrow(rectangleptr);
    let area: Borrowed<gdk::Rectangle> = from_glib_borrow(areaptr);

    let callback = CellCallbackAllocate {
        callback,
        user_data,
    };

    imp.foreach_alloc(
        wrap.unsafe_cast_ref(),
        &*context,
        &*widget,
        &area,
        &rectangle,
        &callback,
    )
}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait CellAreaClassSubclassExt: ClassStruct {
    #[doc(alias = "gtk_cell_area_class_find_cell_property")]
    fn find_cell_property(&self, property_name: &str) -> Option<ParamSpec> {
        unsafe {
            let cell_area_class = self as *const _ as *mut ffi::GtkCellAreaClass;
            from_glib_none(ffi::gtk_cell_area_class_find_cell_property(
                cell_area_class,
                property_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_cell_area_class_list_cell_properties")]
    fn list_cell_properties(&self) -> Vec<ParamSpec> {
        unsafe {
            let cell_area_class = self as *const _ as *mut ffi::GtkCellAreaClass;
            let mut n_properties = std::mem::MaybeUninit::uninit();
            let props = ffi::gtk_cell_area_class_list_cell_properties(
                cell_area_class,
                n_properties.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_none_num(props, n_properties.assume_init() as usize)
        }
    }
}

unsafe impl<T: ClassStruct> CellAreaClassSubclassExt for T where T::Type: CellAreaImpl {}
