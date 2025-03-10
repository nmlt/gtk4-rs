// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Border;
use crate::StateFlags;
use crate::StyleContextPrintFlags;
use crate::StyleProvider;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct StyleContext(Object<ffi::GtkStyleContext, ffi::GtkStyleContextClass>);

    match fn {
        type_ => || ffi::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    #[doc(alias = "gtk_style_context_add_provider_for_display")]
    pub fn add_provider_for_display<P: IsA<gdk::Display>, Q: IsA<StyleProvider>>(
        display: &P,
        provider: &Q,
        priority: u32,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_add_provider_for_display(
                display.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    #[doc(alias = "gtk_style_context_remove_provider_for_display")]
    pub fn remove_provider_for_display<P: IsA<gdk::Display>, Q: IsA<StyleProvider>>(
        display: &P,
        provider: &Q,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_remove_provider_for_display(
                display.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }
}

pub const NONE_STYLE_CONTEXT: Option<&StyleContext> = None;

pub trait StyleContextExt: 'static {
    #[doc(alias = "gtk_style_context_add_class")]
    fn add_class(&self, class_name: &str);

    #[doc(alias = "gtk_style_context_add_provider")]
    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32);

    #[doc(alias = "gtk_style_context_get_border")]
    #[doc(alias = "get_border")]
    fn border(&self) -> Border;

    #[doc(alias = "gtk_style_context_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> gdk::RGBA;

    #[doc(alias = "gtk_style_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> gdk::Display;

    #[doc(alias = "gtk_style_context_get_margin")]
    #[doc(alias = "get_margin")]
    fn margin(&self) -> Border;

    #[doc(alias = "gtk_style_context_get_padding")]
    #[doc(alias = "get_padding")]
    fn padding(&self) -> Border;

    #[doc(alias = "gtk_style_context_get_scale")]
    #[doc(alias = "get_scale")]
    fn scale(&self) -> i32;

    #[doc(alias = "gtk_style_context_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> StateFlags;

    #[doc(alias = "gtk_style_context_has_class")]
    fn has_class(&self, class_name: &str) -> bool;

    #[doc(alias = "gtk_style_context_lookup_color")]
    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA>;

    #[doc(alias = "gtk_style_context_remove_class")]
    fn remove_class(&self, class_name: &str);

    #[doc(alias = "gtk_style_context_remove_provider")]
    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P);

    #[doc(alias = "gtk_style_context_restore")]
    fn restore(&self);

    #[doc(alias = "gtk_style_context_save")]
    fn save(&self);

    #[doc(alias = "gtk_style_context_set_display")]
    fn set_display<P: IsA<gdk::Display>>(&self, display: &P);

    #[doc(alias = "gtk_style_context_set_scale")]
    fn set_scale(&self, scale: i32);

    #[doc(alias = "gtk_style_context_set_state")]
    fn set_state(&self, flags: StateFlags);

    #[doc(alias = "gtk_style_context_to_string")]
    fn to_string(&self, flags: StyleContextPrintFlags) -> glib::GString;

    #[doc(alias = "display")]
    fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleContext>> StyleContextExt for O {
    fn add_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_add_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32) {
        unsafe {
            ffi::gtk_style_context_add_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    fn border(&self) -> Border {
        unsafe {
            let mut border = Border::uninitialized();
            ffi::gtk_style_context_get_border(
                self.as_ref().to_glib_none().0,
                border.to_glib_none_mut().0,
            );
            border
        }
    }

    fn color(&self) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_color(
                self.as_ref().to_glib_none().0,
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn display(&self) -> gdk::Display {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn margin(&self) -> Border {
        unsafe {
            let mut margin = Border::uninitialized();
            ffi::gtk_style_context_get_margin(
                self.as_ref().to_glib_none().0,
                margin.to_glib_none_mut().0,
            );
            margin
        }
    }

    fn padding(&self) -> Border {
        unsafe {
            let mut padding = Border::uninitialized();
            ffi::gtk_style_context_get_padding(
                self.as_ref().to_glib_none().0,
                padding.to_glib_none_mut().0,
            );
            padding
        }
    }

    fn scale(&self) -> i32 {
        unsafe { ffi::gtk_style_context_get_scale(self.as_ref().to_glib_none().0) }
    }

    fn state(&self) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_style_context_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_style_context_has_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            ))
        }
    }

    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_lookup_color(
                self.as_ref().to_glib_none().0,
                color_name.to_glib_none().0,
                color.to_glib_none_mut().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    fn remove_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P) {
        unsafe {
            ffi::gtk_style_context_remove_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    fn restore(&self) {
        unsafe {
            ffi::gtk_style_context_restore(self.as_ref().to_glib_none().0);
        }
    }

    fn save(&self) {
        unsafe {
            ffi::gtk_style_context_save(self.as_ref().to_glib_none().0);
        }
    }

    fn set_display<P: IsA<gdk::Display>>(&self, display: &P) {
        unsafe {
            ffi::gtk_style_context_set_display(
                self.as_ref().to_glib_none().0,
                display.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_scale(&self, scale: i32) {
        unsafe {
            ffi::gtk_style_context_set_scale(self.as_ref().to_glib_none().0, scale);
        }
    }

    fn set_state(&self, flags: StateFlags) {
        unsafe {
            ffi::gtk_style_context_set_state(self.as_ref().to_glib_none().0, flags.into_glib());
        }
    }

    fn to_string(&self, flags: StyleContextPrintFlags) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_style_context_to_string(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "display")]
    fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<
            P: IsA<StyleContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStyleContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleContext")
    }
}
