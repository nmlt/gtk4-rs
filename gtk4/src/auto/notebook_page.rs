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
    pub struct NotebookPage(Object<ffi::GtkNotebookPage>);

    match fn {
        type_ => || ffi::gtk_notebook_page_get_type(),
    }
}

impl NotebookPage {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`NotebookPage`]
    /// This method returns an instance of [`NotebookPageBuilder`] which can be used to create a [`NotebookPage`].
    pub fn builder() -> NotebookPageBuilder {
        NotebookPageBuilder::default()
    }

    #[doc(alias = "gtk_notebook_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_notebook_page_get_child(self.to_glib_none().0)) }
    }

    pub fn is_detachable(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"detachable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `detachable` getter")
        }
    }

    pub fn set_detachable(&self, detachable: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"detachable\0".as_ptr() as *const _,
                detachable.to_value().to_glib_none().0,
            );
        }
    }

    pub fn menu(&self) -> Option<Widget> {
        unsafe {
            let mut value = glib::Value::from_type(<Widget as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"menu\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `menu` getter")
        }
    }

    #[doc(alias = "menu-label")]
    pub fn menu_label(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"menu-label\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `menu-label` getter")
        }
    }

    #[doc(alias = "menu-label")]
    pub fn set_menu_label(&self, menu_label: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"menu-label\0".as_ptr() as *const _,
                menu_label.to_value().to_glib_none().0,
            );
        }
    }

    pub fn position(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `position` getter")
        }
    }

    pub fn set_position(&self, position: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"position\0".as_ptr() as *const _,
                position.to_value().to_glib_none().0,
            );
        }
    }

    pub fn is_reorderable(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"reorderable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `reorderable` getter")
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"reorderable\0".as_ptr() as *const _,
                reorderable.to_value().to_glib_none().0,
            );
        }
    }

    pub fn tab(&self) -> Option<Widget> {
        unsafe {
            let mut value = glib::Value::from_type(<Widget as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `tab` getter")
        }
    }

    #[doc(alias = "tab-expand")]
    pub fn is_tab_expand(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab-expand\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tab-expand` getter")
        }
    }

    #[doc(alias = "tab-expand")]
    pub fn set_tab_expand(&self, tab_expand: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab-expand\0".as_ptr() as *const _,
                tab_expand.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "tab-fill")]
    pub fn is_tab_fill(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab-fill\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tab-fill` getter")
        }
    }

    #[doc(alias = "tab-fill")]
    pub fn set_tab_fill(&self, tab_fill: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab-fill\0".as_ptr() as *const _,
                tab_fill.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "tab-label")]
    pub fn tab_label(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab-label\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tab-label` getter")
        }
    }

    #[doc(alias = "tab-label")]
    pub fn set_tab_label(&self, tab_label: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tab-label\0".as_ptr() as *const _,
                tab_label.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "detachable")]
    pub fn connect_detachable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detachable_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                b"notify::detachable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_detachable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-label")]
    pub fn connect_menu_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_label_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                b"notify::menu-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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

    #[doc(alias = "reorderable")]
    pub fn connect_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reorderable_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                b"notify::reorderable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reorderable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-expand")]
    pub fn connect_tab_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_expand_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                b"notify::tab-expand\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tab_expand_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-fill")]
    pub fn connect_tab_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_fill_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                b"notify::tab-fill\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tab_fill_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tab-label")]
    pub fn connect_tab_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_label_trampoline<F: Fn(&NotebookPage) + 'static>(
            this: *mut ffi::GtkNotebookPage,
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
                b"notify::tab-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tab_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`NotebookPage`].
pub struct NotebookPageBuilder {
    child: Option<Widget>,
    detachable: Option<bool>,
    menu: Option<Widget>,
    menu_label: Option<String>,
    position: Option<i32>,
    reorderable: Option<bool>,
    tab: Option<Widget>,
    tab_expand: Option<bool>,
    tab_fill: Option<bool>,
    tab_label: Option<String>,
}

impl NotebookPageBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`NotebookPageBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`NotebookPage`].
    pub fn build(self) -> NotebookPage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref detachable) = self.detachable {
            properties.push(("detachable", detachable));
        }
        if let Some(ref menu) = self.menu {
            properties.push(("menu", menu));
        }
        if let Some(ref menu_label) = self.menu_label {
            properties.push(("menu-label", menu_label));
        }
        if let Some(ref position) = self.position {
            properties.push(("position", position));
        }
        if let Some(ref reorderable) = self.reorderable {
            properties.push(("reorderable", reorderable));
        }
        if let Some(ref tab) = self.tab {
            properties.push(("tab", tab));
        }
        if let Some(ref tab_expand) = self.tab_expand {
            properties.push(("tab-expand", tab_expand));
        }
        if let Some(ref tab_fill) = self.tab_fill {
            properties.push(("tab-fill", tab_fill));
        }
        if let Some(ref tab_label) = self.tab_label {
            properties.push(("tab-label", tab_label));
        }
        glib::Object::new::<NotebookPage>(&properties)
            .expect("Failed to create an instance of NotebookPage")
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn detachable(mut self, detachable: bool) -> Self {
        self.detachable = Some(detachable);
        self
    }

    pub fn menu<P: IsA<Widget>>(mut self, menu: &P) -> Self {
        self.menu = Some(menu.clone().upcast());
        self
    }

    pub fn menu_label(mut self, menu_label: &str) -> Self {
        self.menu_label = Some(menu_label.to_string());
        self
    }

    pub fn position(mut self, position: i32) -> Self {
        self.position = Some(position);
        self
    }

    pub fn reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = Some(reorderable);
        self
    }

    pub fn tab<P: IsA<Widget>>(mut self, tab: &P) -> Self {
        self.tab = Some(tab.clone().upcast());
        self
    }

    pub fn tab_expand(mut self, tab_expand: bool) -> Self {
        self.tab_expand = Some(tab_expand);
        self
    }

    pub fn tab_fill(mut self, tab_fill: bool) -> Self {
        self.tab_fill = Some(tab_fill);
        self
    }

    pub fn tab_label(mut self, tab_label: &str) -> Self {
        self.tab_label = Some(tab_label.to_string());
        self
    }
}

impl fmt::Display for NotebookPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NotebookPage")
    }
}
