// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::MediaStream;
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
    pub struct Video(Object<ffi::GtkVideo, ffi::GtkVideoClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_video_get_type(),
    }
}

impl Video {
    #[doc(alias = "gtk_video_new")]
    pub fn new() -> Video {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_video_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_video_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file<P: IsA<gio::File>>(file: Option<&P>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_file(
                file.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_video_new_for_filename")]
    #[doc(alias = "new_for_filename")]
    pub fn for_filename<P: AsRef<std::path::Path>>(filename: P) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_filename(
                filename.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_video_new_for_media_stream")]
    #[doc(alias = "new_for_media_stream")]
    pub fn for_media_stream<P: IsA<MediaStream>>(stream: Option<&P>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_media_stream(
                stream.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_video_new_for_resource")]
    #[doc(alias = "new_for_resource")]
    pub fn for_resource(resource_path: Option<&str>) -> Video {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_video_new_for_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Video`]
    /// This method returns an instance of [`VideoBuilder`] which can be used to create a [`Video`].
    pub fn builder() -> VideoBuilder {
        VideoBuilder::default()
    }

    #[doc(alias = "gtk_video_get_autoplay")]
    #[doc(alias = "get_autoplay")]
    pub fn is_autoplay(&self) -> bool {
        unsafe { from_glib(ffi::gtk_video_get_autoplay(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_video_get_file(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_get_loop")]
    #[doc(alias = "get_loop")]
    pub fn is_loop(&self) -> bool {
        unsafe { from_glib(ffi::gtk_video_get_loop(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_get_media_stream")]
    #[doc(alias = "get_media_stream")]
    pub fn media_stream(&self) -> Option<MediaStream> {
        unsafe { from_glib_none(ffi::gtk_video_get_media_stream(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_video_set_autoplay")]
    pub fn set_autoplay(&self, autoplay: bool) {
        unsafe {
            ffi::gtk_video_set_autoplay(self.to_glib_none().0, autoplay.into_glib());
        }
    }

    #[doc(alias = "gtk_video_set_file")]
    pub fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>) {
        unsafe {
            ffi::gtk_video_set_file(
                self.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_video_set_filename")]
    pub fn set_filename(&self, filename: Option<&str>) {
        unsafe {
            ffi::gtk_video_set_filename(self.to_glib_none().0, filename.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_video_set_loop")]
    pub fn set_loop(&self, loop_: bool) {
        unsafe {
            ffi::gtk_video_set_loop(self.to_glib_none().0, loop_.into_glib());
        }
    }

    #[doc(alias = "gtk_video_set_media_stream")]
    pub fn set_media_stream<P: IsA<MediaStream>>(&self, stream: Option<&P>) {
        unsafe {
            ffi::gtk_video_set_media_stream(
                self.to_glib_none().0,
                stream.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_video_set_resource")]
    pub fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_video_set_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[doc(alias = "autoplay")]
    pub fn connect_autoplay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoplay_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::autoplay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autoplay_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "loop")]
    pub fn connect_loop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loop_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::loop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "media-stream")]
    pub fn connect_media_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_stream_trampoline<F: Fn(&Video) + 'static>(
            this: *mut ffi::GtkVideo,
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
                b"notify::media-stream\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_media_stream_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Video {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Video`].
pub struct VideoBuilder {
    autoplay: Option<bool>,
    file: Option<gio::File>,
    loop_: Option<bool>,
    media_stream: Option<MediaStream>,
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

impl VideoBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`VideoBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Video`].
    pub fn build(self) -> Video {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref autoplay) = self.autoplay {
            properties.push(("autoplay", autoplay));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref loop_) = self.loop_ {
            properties.push(("loop", loop_));
        }
        if let Some(ref media_stream) = self.media_stream {
            properties.push(("media-stream", media_stream));
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
        glib::Object::new::<Video>(&properties).expect("Failed to create an instance of Video")
    }

    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = Some(autoplay);
        self
    }

    pub fn file<P: IsA<gio::File>>(mut self, file: &P) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn loop_(mut self, loop_: bool) -> Self {
        self.loop_ = Some(loop_);
        self
    }

    pub fn media_stream<P: IsA<MediaStream>>(mut self, media_stream: &P) -> Self {
        self.media_stream = Some(media_stream.clone().upcast());
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

impl fmt::Display for Video {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Video")
    }
}
