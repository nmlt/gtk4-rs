// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::SelectionModel;
use glib::object::Cast;
use glib::object::IsA;
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
    pub struct SingleSelection(Object<ffi::GtkSingleSelection, ffi::GtkSingleSelectionClass>) @implements gio::ListModel, SelectionModel;

    match fn {
        type_ => || ffi::gtk_single_selection_get_type(),
    }
}

impl SingleSelection {
    #[doc(alias = "gtk_single_selection_new")]
    pub fn new<P: IsA<gio::ListModel>>(model: Option<&P>) -> SingleSelection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_single_selection_new(
                model.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`SingleSelection`]
    /// This method returns an instance of [`SingleSelectionBuilder`] which can be used to create a [`SingleSelection`].
    pub fn builder() -> SingleSelectionBuilder {
        SingleSelectionBuilder::default()
    }

    #[doc(alias = "gtk_single_selection_get_autoselect")]
    #[doc(alias = "get_autoselect")]
    pub fn is_autoselect(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_single_selection_get_autoselect(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_single_selection_get_can_unselect")]
    #[doc(alias = "get_can_unselect")]
    pub fn can_unselect(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_single_selection_get_can_unselect(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_single_selection_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> gio::ListModel {
        unsafe { from_glib_none(ffi::gtk_single_selection_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_single_selection_get_selected")]
    #[doc(alias = "get_selected")]
    pub fn selected(&self) -> u32 {
        unsafe { ffi::gtk_single_selection_get_selected(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_single_selection_get_selected_item")]
    #[doc(alias = "get_selected_item")]
    pub fn selected_item(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::gtk_single_selection_get_selected_item(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_single_selection_set_autoselect")]
    pub fn set_autoselect(&self, autoselect: bool) {
        unsafe {
            ffi::gtk_single_selection_set_autoselect(self.to_glib_none().0, autoselect.into_glib());
        }
    }

    #[doc(alias = "gtk_single_selection_set_can_unselect")]
    pub fn set_can_unselect(&self, can_unselect: bool) {
        unsafe {
            ffi::gtk_single_selection_set_can_unselect(
                self.to_glib_none().0,
                can_unselect.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_single_selection_set_model")]
    pub fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_single_selection_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_single_selection_set_selected")]
    pub fn set_selected(&self, position: u32) {
        unsafe {
            ffi::gtk_single_selection_set_selected(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "autoselect")]
    pub fn connect_autoselect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoselect_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::autoselect\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autoselect_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-unselect")]
    pub fn connect_can_unselect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_unselect_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::can-unselect\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_unselect_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected")]
    pub fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-item")]
    pub fn connect_selected_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_item_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::selected-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`SingleSelection`].
pub struct SingleSelectionBuilder {
    autoselect: Option<bool>,
    can_unselect: Option<bool>,
    model: Option<gio::ListModel>,
    selected: Option<u32>,
}

impl SingleSelectionBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SingleSelectionBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SingleSelection`].
    pub fn build(self) -> SingleSelection {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref autoselect) = self.autoselect {
            properties.push(("autoselect", autoselect));
        }
        if let Some(ref can_unselect) = self.can_unselect {
            properties.push(("can-unselect", can_unselect));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref selected) = self.selected {
            properties.push(("selected", selected));
        }
        glib::Object::new::<SingleSelection>(&properties)
            .expect("Failed to create an instance of SingleSelection")
    }

    pub fn autoselect(mut self, autoselect: bool) -> Self {
        self.autoselect = Some(autoselect);
        self
    }

    pub fn can_unselect(mut self, can_unselect: bool) -> Self {
        self.can_unselect = Some(can_unselect);
        self
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn selected(mut self, selected: u32) -> Self {
        self.selected = Some(selected);
        self
    }
}

impl fmt::Display for SingleSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SingleSelection")
    }
}
