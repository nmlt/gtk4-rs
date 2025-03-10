// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Editable;
use crate::LayoutManager;
use crate::Overflow;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Editable;

    match fn {
        type_ => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[doc(alias = "gtk_search_entry_new")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_search_entry_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`SearchEntry`]
    /// This method returns an instance of [`SearchEntryBuilder`] which can be used to create a [`SearchEntry`].
    pub fn builder() -> SearchEntryBuilder {
        SearchEntryBuilder::default()
    }

    #[doc(alias = "gtk_search_entry_get_key_capture_widget")]
    #[doc(alias = "get_key_capture_widget")]
    pub fn key_capture_widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_search_entry_get_key_capture_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_search_entry_set_key_capture_widget")]
    pub fn set_key_capture_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            ffi::gtk_search_entry_set_key_capture_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "activates-default")]
    pub fn activates_default(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"activates-default\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `activates-default` getter")
        }
    }

    #[doc(alias = "activates-default")]
    pub fn set_activates_default(&self, activates_default: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"activates-default\0".as_ptr() as *const _,
                activates_default.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "placeholder-text")]
    pub fn placeholder_text(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"placeholder-text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `placeholder-text` getter")
        }
    }

    #[doc(alias = "placeholder-text")]
    pub fn set_placeholder_text(&self, placeholder_text: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"placeholder-text\0".as_ptr() as *const _,
                placeholder_text.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("activate", &[])
                .unwrap()
        };
    }

    #[doc(alias = "next-match")]
    pub fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_match_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-match\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_next_match(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("next-match", &[])
                .unwrap()
        };
    }

    #[doc(alias = "previous-match")]
    pub fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn previous_match_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"previous-match\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    previous_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_previous_match(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("previous-match", &[])
                .unwrap()
        };
    }

    #[doc(alias = "search-changed")]
    pub fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_changed_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    search_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "search-started")]
    pub fn connect_search_started<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_started_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    search_started_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stop-search")]
    pub fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_search_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop-search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_stop_search(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("stop-search", &[])
                .unwrap()
        };
    }

    #[doc(alias = "activates-default")]
    pub fn connect_activates_default_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activates_default_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                b"notify::activates-default\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activates_default_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "placeholder-text")]
    pub fn connect_placeholder_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_placeholder_text_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                b"notify::placeholder-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_placeholder_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`SearchEntry`].
pub struct SearchEntryBuilder {
    activates_default: Option<bool>,
    placeholder_text: Option<String>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    editable: Option<bool>,
    enable_undo: Option<bool>,
    max_width_chars: Option<i32>,
    text: Option<String>,
    width_chars: Option<i32>,
    xalign: Option<f32>,
}

impl SearchEntryBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SearchEntryBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchEntry`].
    pub fn build(self) -> SearchEntry {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activates_default) = self.activates_default {
            properties.push(("activates-default", activates_default));
        }
        if let Some(ref placeholder_text) = self.placeholder_text {
            properties.push(("placeholder-text", placeholder_text));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref enable_undo) = self.enable_undo {
            properties.push(("enable-undo", enable_undo));
        }
        if let Some(ref max_width_chars) = self.max_width_chars {
            properties.push(("max-width-chars", max_width_chars));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref width_chars) = self.width_chars {
            properties.push(("width-chars", width_chars));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        glib::Object::new::<SearchEntry>(&properties)
            .expect("Failed to create an instance of SearchEntry")
    }

    pub fn activates_default(mut self, activates_default: bool) -> Self {
        self.activates_default = Some(activates_default);
        self
    }

    pub fn placeholder_text(mut self, placeholder_text: &str) -> Self {
        self.placeholder_text = Some(placeholder_text.to_string());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn enable_undo(mut self, enable_undo: bool) -> Self {
        self.enable_undo = Some(enable_undo);
        self
    }

    pub fn max_width_chars(mut self, max_width_chars: i32) -> Self {
        self.max_width_chars = Some(max_width_chars);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn width_chars(mut self, width_chars: i32) -> Self {
        self.width_chars = Some(width_chars);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }
}

impl fmt::Display for SearchEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SearchEntry")
    }
}
