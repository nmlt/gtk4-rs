// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::CellRenderer;
use crate::CellRendererAccelMode;
use crate::CellRendererMode;
use crate::CellRendererText;
use crate::TreePath;
use glib::object::Cast;
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
    pub struct CellRendererAccel(Object<ffi::GtkCellRendererAccel>) @extends CellRendererText, CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    #[doc(alias = "gtk_cell_renderer_accel_new")]
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_accel_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`CellRendererAccel`]
    /// This method returns an instance of [`CellRendererAccelBuilder`] which can be used to create a [`CellRendererAccel`].
    pub fn builder() -> CellRendererAccelBuilder {
        CellRendererAccelBuilder::default()
    }

    #[doc(alias = "accel-key")]
    pub fn accel_key(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-key\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accel-key` getter")
        }
    }

    #[doc(alias = "accel-key")]
    pub fn set_accel_key(&self, accel_key: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-key\0".as_ptr() as *const _,
                accel_key.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accel-mode")]
    pub fn accel_mode(&self) -> CellRendererAccelMode {
        unsafe {
            let mut value =
                glib::Value::from_type(<CellRendererAccelMode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accel-mode` getter")
        }
    }

    #[doc(alias = "accel-mode")]
    pub fn set_accel_mode(&self, accel_mode: CellRendererAccelMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-mode\0".as_ptr() as *const _,
                accel_mode.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accel-mods")]
    pub fn accel_mods(&self) -> gdk::ModifierType {
        unsafe {
            let mut value =
                glib::Value::from_type(<gdk::ModifierType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-mods\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `accel-mods` getter")
        }
    }

    #[doc(alias = "accel-mods")]
    pub fn set_accel_mods(&self, accel_mods: gdk::ModifierType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accel-mods\0".as_ptr() as *const _,
                accel_mods.to_value().to_glib_none().0,
            );
        }
    }

    pub fn keycode(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"keycode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `keycode` getter")
        }
    }

    pub fn set_keycode(&self, keycode: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"keycode\0".as_ptr() as *const _,
                keycode.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accel-cleared")]
    pub fn connect_accel_cleared<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accel_cleared_trampoline<
            F: Fn(&CellRendererAccel, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellRendererAccel,
            path_string: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path_string));
            f(&from_glib_borrow(this), path)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accel-cleared\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accel_cleared_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-edited")]
    pub fn connect_accel_edited<F: Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accel_edited_trampoline<
            F: Fn(&CellRendererAccel, TreePath, u32, gdk::ModifierType, u32) + 'static,
        >(
            this: *mut ffi::GtkCellRendererAccel,
            path_string: *mut libc::c_char,
            accel_key: libc::c_uint,
            accel_mods: gdk::ffi::GdkModifierType,
            hardware_keycode: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path_string));
            f(
                &from_glib_borrow(this),
                path,
                accel_key,
                from_glib(accel_mods),
                hardware_keycode,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accel-edited\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accel_edited_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-key")]
    pub fn connect_accel_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_key_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::accel-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_key_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-mode")]
    pub fn connect_accel_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_mode_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::accel-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-mods")]
    pub fn connect_accel_mods_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_mods_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::accel-mods\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accel_mods_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "keycode")]
    pub fn connect_keycode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_keycode_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::keycode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_keycode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererAccel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`CellRendererAccel`].
pub struct CellRendererAccelBuilder {
    accel_key: Option<u32>,
    accel_mode: Option<CellRendererAccelMode>,
    accel_mods: Option<gdk::ModifierType>,
    keycode: Option<u32>,
    align_set: Option<bool>,
    alignment: Option<pango::Alignment>,
    attributes: Option<pango::AttrList>,
    background: Option<String>,
    background_rgba: Option<gdk::RGBA>,
    background_set: Option<bool>,
    editable: Option<bool>,
    editable_set: Option<bool>,
    ellipsize: Option<pango::EllipsizeMode>,
    ellipsize_set: Option<bool>,
    family: Option<String>,
    family_set: Option<bool>,
    font: Option<String>,
    font_desc: Option<pango::FontDescription>,
    foreground: Option<String>,
    foreground_rgba: Option<gdk::RGBA>,
    foreground_set: Option<bool>,
    language: Option<String>,
    language_set: Option<bool>,
    markup: Option<String>,
    max_width_chars: Option<i32>,
    placeholder_text: Option<String>,
    rise: Option<i32>,
    rise_set: Option<bool>,
    scale: Option<f64>,
    scale_set: Option<bool>,
    single_paragraph_mode: Option<bool>,
    size: Option<i32>,
    size_points: Option<f64>,
    size_set: Option<bool>,
    stretch: Option<pango::Stretch>,
    stretch_set: Option<bool>,
    strikethrough: Option<bool>,
    strikethrough_set: Option<bool>,
    style: Option<pango::Style>,
    style_set: Option<bool>,
    text: Option<String>,
    underline: Option<pango::Underline>,
    underline_set: Option<bool>,
    variant: Option<pango::Variant>,
    variant_set: Option<bool>,
    weight: Option<i32>,
    weight_set: Option<bool>,
    width_chars: Option<i32>,
    wrap_mode: Option<pango::WrapMode>,
    wrap_width: Option<i32>,
    cell_background: Option<String>,
    cell_background_rgba: Option<gdk::RGBA>,
    cell_background_set: Option<bool>,
    height: Option<i32>,
    is_expanded: Option<bool>,
    is_expander: Option<bool>,
    mode: Option<CellRendererMode>,
    sensitive: Option<bool>,
    visible: Option<bool>,
    width: Option<i32>,
    xalign: Option<f32>,
    xpad: Option<u32>,
    yalign: Option<f32>,
    ypad: Option<u32>,
}

impl CellRendererAccelBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CellRendererAccelBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererAccel`].
    pub fn build(self) -> CellRendererAccel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accel_key) = self.accel_key {
            properties.push(("accel-key", accel_key));
        }
        if let Some(ref accel_mode) = self.accel_mode {
            properties.push(("accel-mode", accel_mode));
        }
        if let Some(ref accel_mods) = self.accel_mods {
            properties.push(("accel-mods", accel_mods));
        }
        if let Some(ref keycode) = self.keycode {
            properties.push(("keycode", keycode));
        }
        if let Some(ref align_set) = self.align_set {
            properties.push(("align-set", align_set));
        }
        if let Some(ref alignment) = self.alignment {
            properties.push(("alignment", alignment));
        }
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref background) = self.background {
            properties.push(("background", background));
        }
        if let Some(ref background_rgba) = self.background_rgba {
            properties.push(("background-rgba", background_rgba));
        }
        if let Some(ref background_set) = self.background_set {
            properties.push(("background-set", background_set));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref editable_set) = self.editable_set {
            properties.push(("editable-set", editable_set));
        }
        if let Some(ref ellipsize) = self.ellipsize {
            properties.push(("ellipsize", ellipsize));
        }
        if let Some(ref ellipsize_set) = self.ellipsize_set {
            properties.push(("ellipsize-set", ellipsize_set));
        }
        if let Some(ref family) = self.family {
            properties.push(("family", family));
        }
        if let Some(ref family_set) = self.family_set {
            properties.push(("family-set", family_set));
        }
        if let Some(ref font) = self.font {
            properties.push(("font", font));
        }
        if let Some(ref font_desc) = self.font_desc {
            properties.push(("font-desc", font_desc));
        }
        if let Some(ref foreground) = self.foreground {
            properties.push(("foreground", foreground));
        }
        if let Some(ref foreground_rgba) = self.foreground_rgba {
            properties.push(("foreground-rgba", foreground_rgba));
        }
        if let Some(ref foreground_set) = self.foreground_set {
            properties.push(("foreground-set", foreground_set));
        }
        if let Some(ref language) = self.language {
            properties.push(("language", language));
        }
        if let Some(ref language_set) = self.language_set {
            properties.push(("language-set", language_set));
        }
        if let Some(ref markup) = self.markup {
            properties.push(("markup", markup));
        }
        if let Some(ref max_width_chars) = self.max_width_chars {
            properties.push(("max-width-chars", max_width_chars));
        }
        if let Some(ref placeholder_text) = self.placeholder_text {
            properties.push(("placeholder-text", placeholder_text));
        }
        if let Some(ref rise) = self.rise {
            properties.push(("rise", rise));
        }
        if let Some(ref rise_set) = self.rise_set {
            properties.push(("rise-set", rise_set));
        }
        if let Some(ref scale) = self.scale {
            properties.push(("scale", scale));
        }
        if let Some(ref scale_set) = self.scale_set {
            properties.push(("scale-set", scale_set));
        }
        if let Some(ref single_paragraph_mode) = self.single_paragraph_mode {
            properties.push(("single-paragraph-mode", single_paragraph_mode));
        }
        if let Some(ref size) = self.size {
            properties.push(("size", size));
        }
        if let Some(ref size_points) = self.size_points {
            properties.push(("size-points", size_points));
        }
        if let Some(ref size_set) = self.size_set {
            properties.push(("size-set", size_set));
        }
        if let Some(ref stretch) = self.stretch {
            properties.push(("stretch", stretch));
        }
        if let Some(ref stretch_set) = self.stretch_set {
            properties.push(("stretch-set", stretch_set));
        }
        if let Some(ref strikethrough) = self.strikethrough {
            properties.push(("strikethrough", strikethrough));
        }
        if let Some(ref strikethrough_set) = self.strikethrough_set {
            properties.push(("strikethrough-set", strikethrough_set));
        }
        if let Some(ref style) = self.style {
            properties.push(("style", style));
        }
        if let Some(ref style_set) = self.style_set {
            properties.push(("style-set", style_set));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref underline) = self.underline {
            properties.push(("underline", underline));
        }
        if let Some(ref underline_set) = self.underline_set {
            properties.push(("underline-set", underline_set));
        }
        if let Some(ref variant) = self.variant {
            properties.push(("variant", variant));
        }
        if let Some(ref variant_set) = self.variant_set {
            properties.push(("variant-set", variant_set));
        }
        if let Some(ref weight) = self.weight {
            properties.push(("weight", weight));
        }
        if let Some(ref weight_set) = self.weight_set {
            properties.push(("weight-set", weight_set));
        }
        if let Some(ref width_chars) = self.width_chars {
            properties.push(("width-chars", width_chars));
        }
        if let Some(ref wrap_mode) = self.wrap_mode {
            properties.push(("wrap-mode", wrap_mode));
        }
        if let Some(ref wrap_width) = self.wrap_width {
            properties.push(("wrap-width", wrap_width));
        }
        if let Some(ref cell_background) = self.cell_background {
            properties.push(("cell-background", cell_background));
        }
        if let Some(ref cell_background_rgba) = self.cell_background_rgba {
            properties.push(("cell-background-rgba", cell_background_rgba));
        }
        if let Some(ref cell_background_set) = self.cell_background_set {
            properties.push(("cell-background-set", cell_background_set));
        }
        if let Some(ref height) = self.height {
            properties.push(("height", height));
        }
        if let Some(ref is_expanded) = self.is_expanded {
            properties.push(("is-expanded", is_expanded));
        }
        if let Some(ref is_expander) = self.is_expander {
            properties.push(("is-expander", is_expander));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        glib::Object::new::<CellRendererAccel>(&properties)
            .expect("Failed to create an instance of CellRendererAccel")
    }

    pub fn accel_key(mut self, accel_key: u32) -> Self {
        self.accel_key = Some(accel_key);
        self
    }

    pub fn accel_mode(mut self, accel_mode: CellRendererAccelMode) -> Self {
        self.accel_mode = Some(accel_mode);
        self
    }

    pub fn accel_mods(mut self, accel_mods: gdk::ModifierType) -> Self {
        self.accel_mods = Some(accel_mods);
        self
    }

    pub fn keycode(mut self, keycode: u32) -> Self {
        self.keycode = Some(keycode);
        self
    }

    pub fn align_set(mut self, align_set: bool) -> Self {
        self.align_set = Some(align_set);
        self
    }

    pub fn alignment(mut self, alignment: pango::Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn attributes(mut self, attributes: &pango::AttrList) -> Self {
        self.attributes = Some(attributes.clone());
        self
    }

    pub fn background(mut self, background: &str) -> Self {
        self.background = Some(background.to_string());
        self
    }

    pub fn background_rgba(mut self, background_rgba: &gdk::RGBA) -> Self {
        self.background_rgba = Some(background_rgba.clone());
        self
    }

    pub fn background_set(mut self, background_set: bool) -> Self {
        self.background_set = Some(background_set);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn editable_set(mut self, editable_set: bool) -> Self {
        self.editable_set = Some(editable_set);
        self
    }

    pub fn ellipsize(mut self, ellipsize: pango::EllipsizeMode) -> Self {
        self.ellipsize = Some(ellipsize);
        self
    }

    pub fn ellipsize_set(mut self, ellipsize_set: bool) -> Self {
        self.ellipsize_set = Some(ellipsize_set);
        self
    }

    pub fn family(mut self, family: &str) -> Self {
        self.family = Some(family.to_string());
        self
    }

    pub fn family_set(mut self, family_set: bool) -> Self {
        self.family_set = Some(family_set);
        self
    }

    pub fn font(mut self, font: &str) -> Self {
        self.font = Some(font.to_string());
        self
    }

    pub fn font_desc(mut self, font_desc: &pango::FontDescription) -> Self {
        self.font_desc = Some(font_desc.clone());
        self
    }

    pub fn foreground(mut self, foreground: &str) -> Self {
        self.foreground = Some(foreground.to_string());
        self
    }

    pub fn foreground_rgba(mut self, foreground_rgba: &gdk::RGBA) -> Self {
        self.foreground_rgba = Some(foreground_rgba.clone());
        self
    }

    pub fn foreground_set(mut self, foreground_set: bool) -> Self {
        self.foreground_set = Some(foreground_set);
        self
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }

    pub fn language_set(mut self, language_set: bool) -> Self {
        self.language_set = Some(language_set);
        self
    }

    pub fn markup(mut self, markup: &str) -> Self {
        self.markup = Some(markup.to_string());
        self
    }

    pub fn max_width_chars(mut self, max_width_chars: i32) -> Self {
        self.max_width_chars = Some(max_width_chars);
        self
    }

    pub fn placeholder_text(mut self, placeholder_text: &str) -> Self {
        self.placeholder_text = Some(placeholder_text.to_string());
        self
    }

    pub fn rise(mut self, rise: i32) -> Self {
        self.rise = Some(rise);
        self
    }

    pub fn rise_set(mut self, rise_set: bool) -> Self {
        self.rise_set = Some(rise_set);
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn scale_set(mut self, scale_set: bool) -> Self {
        self.scale_set = Some(scale_set);
        self
    }

    pub fn single_paragraph_mode(mut self, single_paragraph_mode: bool) -> Self {
        self.single_paragraph_mode = Some(single_paragraph_mode);
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn size_points(mut self, size_points: f64) -> Self {
        self.size_points = Some(size_points);
        self
    }

    pub fn size_set(mut self, size_set: bool) -> Self {
        self.size_set = Some(size_set);
        self
    }

    pub fn stretch(mut self, stretch: pango::Stretch) -> Self {
        self.stretch = Some(stretch);
        self
    }

    pub fn stretch_set(mut self, stretch_set: bool) -> Self {
        self.stretch_set = Some(stretch_set);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn strikethrough_set(mut self, strikethrough_set: bool) -> Self {
        self.strikethrough_set = Some(strikethrough_set);
        self
    }

    pub fn style(mut self, style: pango::Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn style_set(mut self, style_set: bool) -> Self {
        self.style_set = Some(style_set);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn underline(mut self, underline: pango::Underline) -> Self {
        self.underline = Some(underline);
        self
    }

    pub fn underline_set(mut self, underline_set: bool) -> Self {
        self.underline_set = Some(underline_set);
        self
    }

    pub fn variant(mut self, variant: pango::Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn variant_set(mut self, variant_set: bool) -> Self {
        self.variant_set = Some(variant_set);
        self
    }

    pub fn weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn weight_set(mut self, weight_set: bool) -> Self {
        self.weight_set = Some(weight_set);
        self
    }

    pub fn width_chars(mut self, width_chars: i32) -> Self {
        self.width_chars = Some(width_chars);
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: pango::WrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    pub fn wrap_width(mut self, wrap_width: i32) -> Self {
        self.wrap_width = Some(wrap_width);
        self
    }

    pub fn cell_background(mut self, cell_background: &str) -> Self {
        self.cell_background = Some(cell_background.to_string());
        self
    }

    pub fn cell_background_rgba(mut self, cell_background_rgba: &gdk::RGBA) -> Self {
        self.cell_background_rgba = Some(cell_background_rgba.clone());
        self
    }

    pub fn cell_background_set(mut self, cell_background_set: bool) -> Self {
        self.cell_background_set = Some(cell_background_set);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn is_expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = Some(is_expanded);
        self
    }

    pub fn is_expander(mut self, is_expander: bool) -> Self {
        self.is_expander = Some(is_expander);
        self
    }

    pub fn mode(mut self, mode: CellRendererMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: u32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: u32) -> Self {
        self.ypad = Some(ypad);
        self
    }
}

impl fmt::Display for CellRendererAccel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererAccel")
    }
}
