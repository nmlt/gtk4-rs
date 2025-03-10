// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Orientable;
use crate::Scrollable;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ListBase(Object<ffi::GtkListBase, ffi::GtkListBaseClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable, Scrollable;

    match fn {
        type_ => || ffi::gtk_list_base_get_type(),
    }
}

pub const NONE_LIST_BASE: Option<&ListBase> = None;

pub trait ListBaseExt: 'static {
    #[doc(alias = "orientation")]
    fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListBase>> ListBaseExt for O {
    #[doc(alias = "orientation")]
    fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_orientation_trampoline<
            P: IsA<ListBase>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkListBase,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&ListBase::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::orientation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_orientation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ListBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListBase")
    }
}
