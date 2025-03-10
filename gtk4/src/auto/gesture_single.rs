// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct GestureSingle(Object<ffi::GtkGestureSingle, ffi::GtkGestureSingleClass>) @extends Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_single_get_type(),
    }
}

impl GestureSingle {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`GestureSingle`]
    /// This method returns an instance of [`GestureSingleBuilder`] which can be used to create a [`GestureSingle`].
    pub fn builder() -> GestureSingleBuilder {
        GestureSingleBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`GestureSingle`].
pub struct GestureSingleBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl GestureSingleBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GestureSingleBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureSingle`].
    pub fn build(self) -> GestureSingle {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
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
        glib::Object::new::<GestureSingle>(&properties)
            .expect("Failed to create an instance of GestureSingle")
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
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

pub const NONE_GESTURE_SINGLE: Option<&GestureSingle> = None;

pub trait GestureSingleExt: 'static {
    #[doc(alias = "gtk_gesture_single_get_button")]
    #[doc(alias = "get_button")]
    fn button(&self) -> u32;

    #[doc(alias = "gtk_gesture_single_get_current_button")]
    #[doc(alias = "get_current_button")]
    fn current_button(&self) -> u32;

    #[doc(alias = "gtk_gesture_single_get_current_sequence")]
    #[doc(alias = "get_current_sequence")]
    fn current_sequence(&self) -> Option<gdk::EventSequence>;

    #[doc(alias = "gtk_gesture_single_get_exclusive")]
    #[doc(alias = "get_exclusive")]
    fn is_exclusive(&self) -> bool;

    #[doc(alias = "gtk_gesture_single_get_touch_only")]
    #[doc(alias = "get_touch_only")]
    fn is_touch_only(&self) -> bool;

    #[doc(alias = "gtk_gesture_single_set_button")]
    fn set_button(&self, button: u32);

    #[doc(alias = "gtk_gesture_single_set_exclusive")]
    fn set_exclusive(&self, exclusive: bool);

    #[doc(alias = "gtk_gesture_single_set_touch_only")]
    fn set_touch_only(&self, touch_only: bool);

    #[doc(alias = "button")]
    fn connect_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "exclusive")]
    fn connect_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "touch-only")]
    fn connect_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureSingle>> GestureSingleExt for O {
    fn button(&self) -> u32 {
        unsafe { ffi::gtk_gesture_single_get_button(self.as_ref().to_glib_none().0) }
    }

    fn current_button(&self) -> u32 {
        unsafe { ffi::gtk_gesture_single_get_current_button(self.as_ref().to_glib_none().0) }
    }

    fn current_sequence(&self) -> Option<gdk::EventSequence> {
        unsafe {
            from_glib_full(ffi::gtk_gesture_single_get_current_sequence(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_exclusive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_exclusive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_touch_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_single_get_touch_only(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_button(&self, button: u32) {
        unsafe {
            ffi::gtk_gesture_single_set_button(self.as_ref().to_glib_none().0, button);
        }
    }

    fn set_exclusive(&self, exclusive: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_exclusive(
                self.as_ref().to_glib_none().0,
                exclusive.into_glib(),
            );
        }
    }

    fn set_touch_only(&self, touch_only: bool) {
        unsafe {
            ffi::gtk_gesture_single_set_touch_only(
                self.as_ref().to_glib_none().0,
                touch_only.into_glib(),
            );
        }
    }

    #[doc(alias = "button")]
    fn connect_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_trampoline<
            P: IsA<GestureSingle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGestureSingle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "exclusive")]
    fn connect_exclusive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_exclusive_trampoline<
            P: IsA<GestureSingle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGestureSingle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::exclusive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_exclusive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "touch-only")]
    fn connect_touch_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_touch_only_trampoline<
            P: IsA<GestureSingle>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGestureSingle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&GestureSingle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::touch-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_touch_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GestureSingle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureSingle")
    }
}
