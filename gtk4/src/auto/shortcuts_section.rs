// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::BaselinePosition;
use crate::Box;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Orientable;
use crate::Orientation;
use crate::Overflow;
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
    pub struct ShortcutsSection(Object<ffi::GtkShortcutsSection, ffi::GtkShortcutsSectionClass>) @extends Box, Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_shortcuts_section_get_type(),
    }
}

impl ShortcutsSection {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ShortcutsSection`]
    /// This method returns an instance of [`ShortcutsSectionBuilder`] which can be used to create a [`ShortcutsSection`].
    pub fn builder() -> ShortcutsSectionBuilder {
        ShortcutsSectionBuilder::default()
    }

    #[doc(alias = "max-height")]
    pub fn max_height(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-height` getter")
        }
    }

    #[doc(alias = "max-height")]
    pub fn set_max_height(&self, max_height: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-height\0".as_ptr() as *const _,
                max_height.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "section-name")]
    pub fn section_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"section-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `section-name` getter")
        }
    }

    #[doc(alias = "section-name")]
    pub fn set_section_name(&self, section_name: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"section-name\0".as_ptr() as *const _,
                section_name.to_value().to_glib_none().0,
            );
        }
    }

    pub fn title(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `title` getter")
        }
    }

    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                title.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "view-name")]
    pub fn view_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"view-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `view-name` getter")
        }
    }

    #[doc(alias = "view-name")]
    pub fn set_view_name(&self, view_name: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"view-name\0".as_ptr() as *const _,
                view_name.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "max-height")]
    pub fn connect_max_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_height_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                b"notify::max-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "section-name")]
    pub fn connect_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_section_name_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                b"notify::section-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_section_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "view-name")]
    pub fn connect_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_name_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                b"notify::view-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_view_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ShortcutsSection`].
pub struct ShortcutsSectionBuilder {
    max_height: Option<u32>,
    section_name: Option<String>,
    title: Option<String>,
    view_name: Option<String>,
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
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
    orientation: Option<Orientation>,
}

impl ShortcutsSectionBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ShortcutsSectionBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ShortcutsSection`].
    pub fn build(self) -> ShortcutsSection {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref max_height) = self.max_height {
            properties.push(("max-height", max_height));
        }
        if let Some(ref section_name) = self.section_name {
            properties.push(("section-name", section_name));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref view_name) = self.view_name {
            properties.push(("view-name", view_name));
        }
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
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
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<ShortcutsSection>(&properties)
            .expect("Failed to create an instance of ShortcutsSection")
    }

    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn section_name(mut self, section_name: &str) -> Self {
        self.section_name = Some(section_name.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn view_name(mut self, view_name: &str) -> Self {
        self.view_name = Some(view_name.to_string());
        self
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
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

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for ShortcutsSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShortcutsSection")
    }
}
