// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::TextTag;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct TextTagTable(Object<ffi::GtkTextTagTable>) @implements Buildable;

    match fn {
        type_ => || ffi::gtk_text_tag_table_get_type(),
    }
}

impl TextTagTable {
    #[doc(alias = "gtk_text_tag_table_new")]
    pub fn new() -> TextTagTable {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_text_tag_table_new()) }
    }

    #[doc(alias = "gtk_text_tag_table_add")]
    pub fn add<P: IsA<TextTag>>(&self, tag: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_tag_table_add(
                self.to_glib_none().0,
                tag.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_tag_table_foreach")]
    pub fn foreach<P: FnMut(&TextTag)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&TextTag)>(
            tag: *mut ffi::GtkTextTag,
            data: glib::ffi::gpointer,
        ) {
            let tag = from_glib_borrow(tag);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&tag);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_text_tag_table_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[doc(alias = "gtk_text_tag_table_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> i32 {
        unsafe { ffi::gtk_text_tag_table_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_text_tag_table_lookup")]
    pub fn lookup(&self, name: &str) -> Option<TextTag> {
        unsafe {
            from_glib_none(ffi::gtk_text_tag_table_lookup(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_text_tag_table_remove")]
    pub fn remove<P: IsA<TextTag>>(&self, tag: &P) {
        unsafe {
            ffi::gtk_text_tag_table_remove(self.to_glib_none().0, tag.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "tag-added")]
    pub fn connect_tag_added<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tag_added_trampoline<F: Fn(&TextTagTable, &TextTag) + 'static>(
            this: *mut ffi::GtkTextTagTable,
            tag: *mut ffi::GtkTextTag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tag))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tag-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tag_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tag-changed")]
    pub fn connect_tag_changed<F: Fn(&Self, &TextTag, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tag_changed_trampoline<
            F: Fn(&TextTagTable, &TextTag, bool) + 'static,
        >(
            this: *mut ffi::GtkTextTagTable,
            tag: *mut ffi::GtkTextTag,
            size_changed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(tag),
                from_glib(size_changed),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tag-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tag_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tag-removed")]
    pub fn connect_tag_removed<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tag_removed_trampoline<F: Fn(&TextTagTable, &TextTag) + 'static>(
            this: *mut ffi::GtkTextTagTable,
            tag: *mut ffi::GtkTextTag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tag))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tag-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tag_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TextTagTable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TextTagTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextTagTable")
    }
}
