// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FileChooserAction;
use crate::FileFilter;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct FileChooser(Interface<ffi::GtkFileChooser>);

    match fn {
        type_ => || ffi::gtk_file_chooser_get_type(),
    }
}

pub const NONE_FILE_CHOOSER: Option<&FileChooser> = None;

pub trait FileChooserExt: 'static {
    #[doc(alias = "gtk_file_chooser_add_filter")]
    fn add_filter(&self, filter: &FileFilter);

    #[doc(alias = "gtk_file_chooser_add_shortcut_folder")]
    fn add_shortcut_folder<P: IsA<gio::File>>(&self, folder: &P) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_file_chooser_get_action")]
    #[doc(alias = "get_action")]
    fn action(&self) -> FileChooserAction;

    #[doc(alias = "gtk_file_chooser_get_choice")]
    #[doc(alias = "get_choice")]
    fn choice(&self, id: &str) -> Option<glib::GString>;

    #[doc(alias = "gtk_file_chooser_get_create_folders")]
    #[doc(alias = "get_create_folders")]
    fn creates_folders(&self) -> bool;

    #[doc(alias = "gtk_file_chooser_get_current_folder")]
    #[doc(alias = "get_current_folder")]
    fn current_folder(&self) -> Option<gio::File>;

    #[doc(alias = "gtk_file_chooser_get_current_name")]
    #[doc(alias = "get_current_name")]
    fn current_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_file_chooser_get_file")]
    #[doc(alias = "get_file")]
    fn file(&self) -> Option<gio::File>;

    #[doc(alias = "gtk_file_chooser_get_files")]
    #[doc(alias = "get_files")]
    fn files(&self) -> Option<gio::ListModel>;

    #[doc(alias = "gtk_file_chooser_get_filter")]
    #[doc(alias = "get_filter")]
    fn filter(&self) -> Option<FileFilter>;

    #[doc(alias = "gtk_file_chooser_get_filters")]
    #[doc(alias = "get_filters")]
    fn filters(&self) -> Option<gio::ListModel>;

    #[doc(alias = "gtk_file_chooser_get_select_multiple")]
    #[doc(alias = "get_select_multiple")]
    fn selects_multiple(&self) -> bool;

    #[doc(alias = "gtk_file_chooser_get_shortcut_folders")]
    #[doc(alias = "get_shortcut_folders")]
    fn shortcut_folders(&self) -> Option<gio::ListModel>;

    #[doc(alias = "gtk_file_chooser_remove_choice")]
    fn remove_choice(&self, id: &str);

    #[doc(alias = "gtk_file_chooser_remove_filter")]
    fn remove_filter(&self, filter: &FileFilter);

    #[doc(alias = "gtk_file_chooser_remove_shortcut_folder")]
    fn remove_shortcut_folder<P: IsA<gio::File>>(&self, folder: &P) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_file_chooser_set_action")]
    fn set_action(&self, action: FileChooserAction);

    #[doc(alias = "gtk_file_chooser_set_choice")]
    fn set_choice(&self, id: &str, option: &str);

    #[doc(alias = "gtk_file_chooser_set_create_folders")]
    fn set_create_folders(&self, create_folders: bool);

    #[doc(alias = "gtk_file_chooser_set_current_folder")]
    fn set_current_folder<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_file_chooser_set_current_name")]
    fn set_current_name(&self, name: &str);

    #[doc(alias = "gtk_file_chooser_set_file")]
    fn set_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_file_chooser_set_filter")]
    fn set_filter(&self, filter: &FileFilter);

    #[doc(alias = "gtk_file_chooser_set_select_multiple")]
    fn set_select_multiple(&self, select_multiple: bool);

    #[doc(alias = "action")]
    fn connect_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "create-folders")]
    fn connect_create_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "filter")]
    fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "filters")]
    fn connect_filters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "select-multiple")]
    fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "shortcut-folders")]
    fn connect_shortcut_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooser>> FileChooserExt for O {
    fn add_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_add_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn add_shortcut_folder<P: IsA<gio::File>>(&self, folder: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_add_shortcut_folder(
                self.as_ref().to_glib_none().0,
                folder.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn action(&self) -> FileChooserAction {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn choice(&self, id: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    fn creates_folders(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_create_folders(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_folder(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_folder(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_current_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn files(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_files(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn filter(&self) -> Option<FileFilter> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_get_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn filters(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_filters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn selects_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_get_select_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shortcut_folders(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(ffi::gtk_file_chooser_get_shortcut_folders(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_choice(&self, id: &str) {
        unsafe {
            ffi::gtk_file_chooser_remove_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            );
        }
    }

    fn remove_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_remove_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn remove_shortcut_folder<P: IsA<gio::File>>(&self, folder: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_remove_shortcut_folder(
                self.as_ref().to_glib_none().0,
                folder.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_action(&self, action: FileChooserAction) {
        unsafe {
            ffi::gtk_file_chooser_set_action(self.as_ref().to_glib_none().0, action.into_glib());
        }
    }

    fn set_choice(&self, id: &str, option: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                option.to_glib_none().0,
            );
        }
    }

    fn set_create_folders(&self, create_folders: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_create_folders(
                self.as_ref().to_glib_none().0,
                create_folders.into_glib(),
            );
        }
    }

    fn set_current_folder<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_set_current_folder(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_current_name(&self, name: &str) {
        unsafe {
            ffi::gtk_file_chooser_set_current_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn set_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_file_chooser_set_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_filter(&self, filter: &FileFilter) {
        unsafe {
            ffi::gtk_file_chooser_set_filter(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_file_chooser_set_select_multiple(
                self.as_ref().to_glib_none().0,
                select_multiple.into_glib(),
            );
        }
    }

    #[doc(alias = "action")]
    fn connect_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<P: IsA<FileChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "create-folders")]
    fn connect_create_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_create_folders_trampoline<
            P: IsA<FileChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::create-folders\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_create_folders_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "filter")]
    fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<P: IsA<FileChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "filters")]
    fn connect_filters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filters_trampoline<P: IsA<FileChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filters\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filters_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "select-multiple")]
    fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_multiple_trampoline<
            P: IsA<FileChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::select-multiple\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_select_multiple_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shortcut-folders")]
    fn connect_shortcut_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcut_folders_trampoline<
            P: IsA<FileChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkFileChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&FileChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shortcut-folders\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcut_folders_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooser")
    }
}
