// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

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
    pub struct EntryBuffer(Object<ffi::GtkEntryBuffer, ffi::GtkEntryBufferClass>);

    match fn {
        type_ => || ffi::gtk_entry_buffer_get_type(),
    }
}

impl EntryBuffer {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`EntryBuffer`]
    /// This method returns an instance of [`EntryBufferBuilder`] which can be used to create a [`EntryBuffer`].
    pub fn builder() -> EntryBufferBuilder {
        EntryBufferBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`EntryBuffer`].
pub struct EntryBufferBuilder {
    max_length: Option<i32>,
    text: Option<String>,
}

impl EntryBufferBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`EntryBufferBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EntryBuffer`].
    pub fn build(self) -> EntryBuffer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref max_length) = self.max_length {
            properties.push(("max-length", max_length));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        glib::Object::new::<EntryBuffer>(&properties)
            .expect("Failed to create an instance of EntryBuffer")
    }

    pub fn max_length(mut self, max_length: i32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

pub const NONE_ENTRY_BUFFER: Option<&EntryBuffer> = None;

pub trait EntryBufferExt: 'static {
    #[doc(alias = "gtk_entry_buffer_emit_deleted_text")]
    fn emit_deleted_text(&self, position: u32, n_chars: u32);

    #[doc(alias = "gtk_entry_buffer_emit_inserted_text")]
    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32);

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "max-length")]
    fn connect_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EntryBuffer>> EntryBufferExt for O {
    fn emit_deleted_text(&self, position: u32, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(
                self.as_ref().to_glib_none().0,
                position,
                n_chars,
            );
        }
    }

    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32) {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(
                self.as_ref().to_glib_none().0,
                position,
                chars.to_glib_none().0,
                n_chars,
            );
        }
    }

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P: IsA<EntryBuffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-length")]
    fn connect_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_length_trampoline<
            P: IsA<EntryBuffer>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkEntryBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P: IsA<EntryBuffer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEntryBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&EntryBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EntryBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EntryBuffer")
    }
}
