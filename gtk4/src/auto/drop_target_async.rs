// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DropTargetAsync(Object<ffi::GtkDropTargetAsync, ffi::GtkDropTargetAsyncClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_drop_target_async_get_type(),
    }
}

impl DropTargetAsync {
    #[doc(alias = "gtk_drop_target_async_new")]
    pub fn new(formats: Option<&gdk::ContentFormats>, actions: gdk::DragAction) -> DropTargetAsync {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_drop_target_async_new(
                formats.to_glib_full(),
                actions.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`DropTargetAsync`]
    /// This method returns an instance of [`DropTargetAsyncBuilder`] which can be used to create a [`DropTargetAsync`].
    pub fn builder() -> DropTargetAsyncBuilder {
        DropTargetAsyncBuilder::default()
    }

    #[doc(alias = "gtk_drop_target_async_get_actions")]
    #[doc(alias = "get_actions")]
    pub fn actions(&self) -> gdk::DragAction {
        unsafe {
            from_glib(ffi::gtk_drop_target_async_get_actions(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_drop_target_async_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> Option<gdk::ContentFormats> {
        unsafe {
            from_glib_full(ffi::gtk_drop_target_async_get_formats(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_drop_target_async_reject_drop")]
    pub fn reject_drop(&self, drop: &gdk::Drop) {
        unsafe {
            ffi::gtk_drop_target_async_reject_drop(self.to_glib_none().0, drop.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_drop_target_async_set_actions")]
    pub fn set_actions(&self, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_drop_target_async_set_actions(self.to_glib_none().0, actions.into_glib());
        }
    }

    #[doc(alias = "gtk_drop_target_async_set_formats")]
    pub fn set_formats(&self, formats: Option<&gdk::ContentFormats>) {
        unsafe {
            ffi::gtk_drop_target_async_set_formats(self.to_glib_none().0, formats.to_glib_none().0);
        }
    }

    #[doc(alias = "accept")]
    pub fn connect_accept<F: Fn(&Self, &gdk::Drop) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop) -> bool + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drag-enter")]
    pub fn connect_drag_enter<F: Fn(&Self, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_enter_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-enter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drag-leave")]
    pub fn connect_drag_leave<F: Fn(&Self, &gdk::Drop) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_leave_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop) + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-leave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drag-motion")]
    pub fn connect_drag_motion<F: Fn(&Self, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_motion_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drop")]
    pub fn connect_drop<F: Fn(&Self, &gdk::Drop, f64, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop, f64, f64) -> bool + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "actions")]
    pub fn connect_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&DropTargetAsync) + 'static>(
            this: *mut ffi::GtkDropTargetAsync,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::actions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "formats")]
    pub fn connect_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&DropTargetAsync) + 'static>(
            this: *mut ffi::GtkDropTargetAsync,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`DropTargetAsync`].
pub struct DropTargetAsyncBuilder {
    actions: Option<gdk::DragAction>,
    formats: Option<gdk::ContentFormats>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl DropTargetAsyncBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`DropTargetAsyncBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DropTargetAsync`].
    pub fn build(self) -> DropTargetAsync {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref actions) = self.actions {
            properties.push(("actions", actions));
        }
        if let Some(ref formats) = self.formats {
            properties.push(("formats", formats));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        glib::Object::new::<DropTargetAsync>(&properties)
            .expect("Failed to create an instance of DropTargetAsync")
    }

    pub fn actions(mut self, actions: gdk::DragAction) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn formats(mut self, formats: &gdk::ContentFormats) -> Self {
        self.formats = Some(formats.clone());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for DropTargetAsync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DropTargetAsync")
    }
}
