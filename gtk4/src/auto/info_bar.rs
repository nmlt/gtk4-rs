// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::Button;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::MessageType;
use crate::Overflow;
use crate::ResponseType;
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
    pub struct InfoBar(Object<ffi::GtkInfoBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_info_bar_get_type(),
    }
}

impl InfoBar {
    #[doc(alias = "gtk_info_bar_new")]
    pub fn new() -> InfoBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_info_bar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`InfoBar`]
    /// This method returns an instance of [`InfoBarBuilder`] which can be used to create a [`InfoBar`].
    pub fn builder() -> InfoBarBuilder {
        InfoBarBuilder::default()
    }

    #[doc(alias = "gtk_info_bar_add_action_widget")]
    pub fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: ResponseType) {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                response_id.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_info_bar_add_button")]
    pub fn add_button(&self, button_text: &str, response_id: ResponseType) -> Button {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_add_button(
                self.to_glib_none().0,
                button_text.to_glib_none().0,
                response_id.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_info_bar_add_child")]
    pub fn add_child<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            ffi::gtk_info_bar_add_child(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_info_bar_get_message_type")]
    #[doc(alias = "get_message_type")]
    pub fn message_type(&self) -> MessageType {
        unsafe { from_glib(ffi::gtk_info_bar_get_message_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_info_bar_get_revealed")]
    #[doc(alias = "get_revealed")]
    pub fn is_revealed(&self) -> bool {
        unsafe { from_glib(ffi::gtk_info_bar_get_revealed(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_info_bar_get_show_close_button")]
    #[doc(alias = "get_show_close_button")]
    pub fn shows_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_show_close_button(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_info_bar_remove_action_widget")]
    pub fn remove_action_widget<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            ffi::gtk_info_bar_remove_action_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_info_bar_remove_child")]
    pub fn remove_child<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            ffi::gtk_info_bar_remove_child(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_info_bar_response")]
    pub fn response(&self, response_id: ResponseType) {
        unsafe {
            ffi::gtk_info_bar_response(self.to_glib_none().0, response_id.into_glib());
        }
    }

    #[doc(alias = "gtk_info_bar_set_default_response")]
    pub fn set_default_response(&self, response_id: ResponseType) {
        unsafe {
            ffi::gtk_info_bar_set_default_response(self.to_glib_none().0, response_id.into_glib());
        }
    }

    #[doc(alias = "gtk_info_bar_set_message_type")]
    pub fn set_message_type(&self, message_type: MessageType) {
        unsafe {
            ffi::gtk_info_bar_set_message_type(self.to_glib_none().0, message_type.into_glib());
        }
    }

    #[doc(alias = "gtk_info_bar_set_response_sensitive")]
    pub fn set_response_sensitive(&self, response_id: ResponseType, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_response_sensitive(
                self.to_glib_none().0,
                response_id.into_glib(),
                setting.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_info_bar_set_revealed")]
    pub fn set_revealed(&self, revealed: bool) {
        unsafe {
            ffi::gtk_info_bar_set_revealed(self.to_glib_none().0, revealed.into_glib());
        }
    }

    #[doc(alias = "gtk_info_bar_set_show_close_button")]
    pub fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_show_close_button(self.to_glib_none().0, setting.into_glib());
        }
    }

    #[doc(alias = "close")]
    pub fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<F: Fn(&InfoBar) + 'static>(
            this: *mut ffi::GtkInfoBar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_close(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("close", &[])
                .unwrap()
        };
    }

    #[doc(alias = "response")]
    pub fn connect_response<F: Fn(&Self, ResponseType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<F: Fn(&InfoBar, ResponseType) + 'static>(
            this: *mut ffi::GtkInfoBar,
            response_id: ffi::GtkResponseType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(response_id))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    response_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "message-type")]
    pub fn connect_message_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_type_trampoline<F: Fn(&InfoBar) + 'static>(
            this: *mut ffi::GtkInfoBar,
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
                b"notify::message-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "revealed")]
    pub fn connect_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_revealed_trampoline<F: Fn(&InfoBar) + 'static>(
            this: *mut ffi::GtkInfoBar,
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
                b"notify::revealed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_revealed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-close-button")]
    pub fn connect_show_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<F: Fn(&InfoBar) + 'static>(
            this: *mut ffi::GtkInfoBar,
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
                b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_close_button_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for InfoBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`InfoBar`].
pub struct InfoBarBuilder {
    message_type: Option<MessageType>,
    revealed: Option<bool>,
    show_close_button: Option<bool>,
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
}

impl InfoBarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`InfoBarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`InfoBar`].
    pub fn build(self) -> InfoBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref message_type) = self.message_type {
            properties.push(("message-type", message_type));
        }
        if let Some(ref revealed) = self.revealed {
            properties.push(("revealed", revealed));
        }
        if let Some(ref show_close_button) = self.show_close_button {
            properties.push(("show-close-button", show_close_button));
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
        glib::Object::new::<InfoBar>(&properties).expect("Failed to create an instance of InfoBar")
    }

    pub fn message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = Some(message_type);
        self
    }

    pub fn revealed(mut self, revealed: bool) -> Self {
        self.revealed = Some(revealed);
        self
    }

    pub fn show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = Some(show_close_button);
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
}

impl fmt::Display for InfoBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InfoBar")
    }
}
