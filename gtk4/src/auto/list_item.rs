// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Widget;
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
    pub struct ListItem(Object<ffi::GtkListItem, ffi::GtkListItemClass>);

    match fn {
        type_ => || ffi::gtk_list_item_get_type(),
    }
}

impl ListItem {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ListItem`]
    /// This method returns an instance of [`ListItemBuilder`] which can be used to create a [`ListItem`].
    pub fn builder() -> ListItemBuilder {
        ListItemBuilder::default()
    }

    #[doc(alias = "gtk_list_item_get_activatable")]
    #[doc(alias = "get_activatable")]
    pub fn is_activatable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_item_get_activatable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_item_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_list_item_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_item_get_item")]
    #[doc(alias = "get_item")]
    pub fn item(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_list_item_get_item(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_item_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> u32 {
        unsafe { ffi::gtk_list_item_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_list_item_get_selectable")]
    #[doc(alias = "get_selectable")]
    pub fn is_selectable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_item_get_selectable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_item_get_selected")]
    #[doc(alias = "get_selected")]
    pub fn is_selected(&self) -> bool {
        unsafe { from_glib(ffi::gtk_list_item_get_selected(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_list_item_set_activatable")]
    pub fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_item_set_activatable(self.to_glib_none().0, activatable.into_glib());
        }
    }

    #[doc(alias = "gtk_list_item_set_child")]
    pub fn set_child<P: IsA<Widget>>(&self, child: Option<&P>) {
        unsafe {
            ffi::gtk_list_item_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_item_set_selectable")]
    pub fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_item_set_selectable(self.to_glib_none().0, selectable.into_glib());
        }
    }

    #[doc(alias = "activatable")]
    pub fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "item")]
    pub fn connect_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selectable")]
    pub fn connect_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
                b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected")]
    pub fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&ListItem) + 'static>(
            this: *mut ffi::GtkListItem,
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
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ListItem`].
pub struct ListItemBuilder {
    activatable: Option<bool>,
    child: Option<Widget>,
    selectable: Option<bool>,
}

impl ListItemBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ListItemBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListItem`].
    pub fn build(self) -> ListItem {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activatable) = self.activatable {
            properties.push(("activatable", activatable));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref selectable) = self.selectable {
            properties.push(("selectable", selectable));
        }
        glib::Object::new::<ListItem>(&properties)
            .expect("Failed to create an instance of ListItem")
    }

    pub fn activatable(mut self, activatable: bool) -> Self {
        self.activatable = Some(activatable);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListItem")
    }
}
