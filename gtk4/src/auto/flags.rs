// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "GtkApplicationInhibitFlags")]
    pub struct ApplicationInhibitFlags: u32 {
        #[doc(alias = "GTK_APPLICATION_INHIBIT_LOGOUT")]
        const LOGOUT = 1;
        #[doc(alias = "GTK_APPLICATION_INHIBIT_SWITCH")]
        const SWITCH = 2;
        #[doc(alias = "GTK_APPLICATION_INHIBIT_SUSPEND")]
        const SUSPEND = 4;
        #[doc(alias = "GTK_APPLICATION_INHIBIT_IDLE")]
        const IDLE = 8;
    }
}

impl fmt::Display for ApplicationInhibitFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ApplicationInhibitFlags {
    type GlibType = ffi::GtkApplicationInhibitFlags;

    fn into_glib(self) -> ffi::GtkApplicationInhibitFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkApplicationInhibitFlags> for ApplicationInhibitFlags {
    unsafe fn from_glib(value: ffi::GtkApplicationInhibitFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ApplicationInhibitFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_application_inhibit_flags_get_type()) }
    }
}

impl glib::value::ValueType for ApplicationInhibitFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ApplicationInhibitFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ApplicationInhibitFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkBuilderClosureFlags")]
    pub struct BuilderClosureFlags: u32 {
        #[doc(alias = "GTK_BUILDER_CLOSURE_SWAPPED")]
        const SWAPPED = 1;
    }
}

impl fmt::Display for BuilderClosureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for BuilderClosureFlags {
    type GlibType = ffi::GtkBuilderClosureFlags;

    fn into_glib(self) -> ffi::GtkBuilderClosureFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkBuilderClosureFlags> for BuilderClosureFlags {
    unsafe fn from_glib(value: ffi::GtkBuilderClosureFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for BuilderClosureFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_builder_closure_flags_get_type()) }
    }
}

impl glib::value::ValueType for BuilderClosureFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BuilderClosureFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for BuilderClosureFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkCellRendererState")]
    pub struct CellRendererState: u32 {
        #[doc(alias = "GTK_CELL_RENDERER_SELECTED")]
        const SELECTED = 1;
        #[doc(alias = "GTK_CELL_RENDERER_PRELIT")]
        const PRELIT = 2;
        #[doc(alias = "GTK_CELL_RENDERER_INSENSITIVE")]
        const INSENSITIVE = 4;
        #[doc(alias = "GTK_CELL_RENDERER_SORTED")]
        const SORTED = 8;
        #[doc(alias = "GTK_CELL_RENDERER_FOCUSED")]
        const FOCUSED = 16;
        #[doc(alias = "GTK_CELL_RENDERER_EXPANDABLE")]
        const EXPANDABLE = 32;
        #[doc(alias = "GTK_CELL_RENDERER_EXPANDED")]
        const EXPANDED = 64;
    }
}

impl fmt::Display for CellRendererState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for CellRendererState {
    type GlibType = ffi::GtkCellRendererState;

    fn into_glib(self) -> ffi::GtkCellRendererState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkCellRendererState> for CellRendererState {
    unsafe fn from_glib(value: ffi::GtkCellRendererState) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for CellRendererState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_cell_renderer_state_get_type()) }
    }
}

impl glib::value::ValueType for CellRendererState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for CellRendererState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for CellRendererState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkDebugFlags")]
    pub struct DebugFlags: u32 {
        #[doc(alias = "GTK_DEBUG_TEXT")]
        const TEXT = 1;
        #[doc(alias = "GTK_DEBUG_TREE")]
        const TREE = 2;
        #[doc(alias = "GTK_DEBUG_KEYBINDINGS")]
        const KEYBINDINGS = 4;
        #[doc(alias = "GTK_DEBUG_MODULES")]
        const MODULES = 8;
        #[doc(alias = "GTK_DEBUG_GEOMETRY")]
        const GEOMETRY = 16;
        #[doc(alias = "GTK_DEBUG_ICONTHEME")]
        const ICONTHEME = 32;
        #[doc(alias = "GTK_DEBUG_PRINTING")]
        const PRINTING = 64;
        #[doc(alias = "GTK_DEBUG_BUILDER")]
        const BUILDER = 128;
        #[doc(alias = "GTK_DEBUG_SIZE_REQUEST")]
        const SIZE_REQUEST = 256;
        #[doc(alias = "GTK_DEBUG_NO_CSS_CACHE")]
        const NO_CSS_CACHE = 512;
        #[doc(alias = "GTK_DEBUG_INTERACTIVE")]
        const INTERACTIVE = 1024;
        #[doc(alias = "GTK_DEBUG_TOUCHSCREEN")]
        const TOUCHSCREEN = 2048;
        #[doc(alias = "GTK_DEBUG_ACTIONS")]
        const ACTIONS = 4096;
        #[doc(alias = "GTK_DEBUG_LAYOUT")]
        const LAYOUT = 8192;
        #[doc(alias = "GTK_DEBUG_SNAPSHOT")]
        const SNAPSHOT = 16384;
        #[doc(alias = "GTK_DEBUG_CONSTRAINTS")]
        const CONSTRAINTS = 32768;
        #[doc(alias = "GTK_DEBUG_BUILDER_OBJECTS")]
        const BUILDER_OBJECTS = 65536;
        #[doc(alias = "GTK_DEBUG_A11Y")]
        const A11Y = 131072;
    }
}

impl fmt::Display for DebugFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DebugFlags {
    type GlibType = ffi::GtkDebugFlags;

    fn into_glib(self) -> ffi::GtkDebugFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDebugFlags> for DebugFlags {
    unsafe fn from_glib(value: ffi::GtkDebugFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DebugFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_debug_flags_get_type()) }
    }
}

impl glib::value::ValueType for DebugFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DebugFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DebugFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkDialogFlags")]
    pub struct DialogFlags: u32 {
        #[doc(alias = "GTK_DIALOG_MODAL")]
        const MODAL = 1;
        #[doc(alias = "GTK_DIALOG_DESTROY_WITH_PARENT")]
        const DESTROY_WITH_PARENT = 2;
        #[doc(alias = "GTK_DIALOG_USE_HEADER_BAR")]
        const USE_HEADER_BAR = 4;
    }
}

impl fmt::Display for DialogFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DialogFlags {
    type GlibType = ffi::GtkDialogFlags;

    fn into_glib(self) -> ffi::GtkDialogFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDialogFlags> for DialogFlags {
    unsafe fn from_glib(value: ffi::GtkDialogFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DialogFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_dialog_flags_get_type()) }
    }
}

impl glib::value::ValueType for DialogFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DialogFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DialogFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkEventControllerScrollFlags")]
    pub struct EventControllerScrollFlags: u32 {
        #[doc(alias = "GTK_EVENT_CONTROLLER_SCROLL_NONE")]
        const NONE = 0;
        #[doc(alias = "GTK_EVENT_CONTROLLER_SCROLL_VERTICAL")]
        const VERTICAL = 1;
        #[doc(alias = "GTK_EVENT_CONTROLLER_SCROLL_HORIZONTAL")]
        const HORIZONTAL = 2;
        #[doc(alias = "GTK_EVENT_CONTROLLER_SCROLL_DISCRETE")]
        const DISCRETE = 4;
        #[doc(alias = "GTK_EVENT_CONTROLLER_SCROLL_KINETIC")]
        const KINETIC = 8;
        #[doc(alias = "GTK_EVENT_CONTROLLER_SCROLL_BOTH_AXES")]
        const BOTH_AXES = 3;
    }
}

impl fmt::Display for EventControllerScrollFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for EventControllerScrollFlags {
    type GlibType = ffi::GtkEventControllerScrollFlags;

    fn into_glib(self) -> ffi::GtkEventControllerScrollFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkEventControllerScrollFlags> for EventControllerScrollFlags {
    unsafe fn from_glib(value: ffi::GtkEventControllerScrollFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for EventControllerScrollFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_event_controller_scroll_flags_get_type()) }
    }
}

impl glib::value::ValueType for EventControllerScrollFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for EventControllerScrollFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for EventControllerScrollFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkFontChooserLevel")]
    pub struct FontChooserLevel: u32 {
        #[doc(alias = "GTK_FONT_CHOOSER_LEVEL_FAMILY")]
        const FAMILY = 0;
        #[doc(alias = "GTK_FONT_CHOOSER_LEVEL_STYLE")]
        const STYLE = 1;
        #[doc(alias = "GTK_FONT_CHOOSER_LEVEL_SIZE")]
        const SIZE = 2;
        #[doc(alias = "GTK_FONT_CHOOSER_LEVEL_VARIATIONS")]
        const VARIATIONS = 4;
        #[doc(alias = "GTK_FONT_CHOOSER_LEVEL_FEATURES")]
        const FEATURES = 8;
    }
}

impl fmt::Display for FontChooserLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FontChooserLevel {
    type GlibType = ffi::GtkFontChooserLevel;

    fn into_glib(self) -> ffi::GtkFontChooserLevel {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkFontChooserLevel> for FontChooserLevel {
    unsafe fn from_glib(value: ffi::GtkFontChooserLevel) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FontChooserLevel {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_font_chooser_level_get_type()) }
    }
}

impl glib::value::ValueType for FontChooserLevel {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FontChooserLevel {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FontChooserLevel {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkIconLookupFlags")]
    pub struct IconLookupFlags: u32 {
        #[doc(alias = "GTK_ICON_LOOKUP_FORCE_REGULAR")]
        const FORCE_REGULAR = 1;
        #[doc(alias = "GTK_ICON_LOOKUP_FORCE_SYMBOLIC")]
        const FORCE_SYMBOLIC = 2;
        #[doc(alias = "GTK_ICON_LOOKUP_PRELOAD")]
        const PRELOAD = 4;
    }
}

impl fmt::Display for IconLookupFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for IconLookupFlags {
    type GlibType = ffi::GtkIconLookupFlags;

    fn into_glib(self) -> ffi::GtkIconLookupFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkIconLookupFlags> for IconLookupFlags {
    unsafe fn from_glib(value: ffi::GtkIconLookupFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for IconLookupFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_icon_lookup_flags_get_type()) }
    }
}

impl glib::value::ValueType for IconLookupFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IconLookupFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for IconLookupFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkInputHints")]
    pub struct InputHints: u32 {
        #[doc(alias = "GTK_INPUT_HINT_NONE")]
        const NONE = 0;
        #[doc(alias = "GTK_INPUT_HINT_SPELLCHECK")]
        const SPELLCHECK = 1;
        #[doc(alias = "GTK_INPUT_HINT_NO_SPELLCHECK")]
        const NO_SPELLCHECK = 2;
        #[doc(alias = "GTK_INPUT_HINT_WORD_COMPLETION")]
        const WORD_COMPLETION = 4;
        #[doc(alias = "GTK_INPUT_HINT_LOWERCASE")]
        const LOWERCASE = 8;
        #[doc(alias = "GTK_INPUT_HINT_UPPERCASE_CHARS")]
        const UPPERCASE_CHARS = 16;
        #[doc(alias = "GTK_INPUT_HINT_UPPERCASE_WORDS")]
        const UPPERCASE_WORDS = 32;
        #[doc(alias = "GTK_INPUT_HINT_UPPERCASE_SENTENCES")]
        const UPPERCASE_SENTENCES = 64;
        #[doc(alias = "GTK_INPUT_HINT_INHIBIT_OSK")]
        const INHIBIT_OSK = 128;
        #[doc(alias = "GTK_INPUT_HINT_VERTICAL_WRITING")]
        const VERTICAL_WRITING = 256;
        #[doc(alias = "GTK_INPUT_HINT_EMOJI")]
        const EMOJI = 512;
        #[doc(alias = "GTK_INPUT_HINT_NO_EMOJI")]
        const NO_EMOJI = 1024;
        #[doc(alias = "GTK_INPUT_HINT_PRIVATE")]
        const PRIVATE = 2048;
    }
}

impl fmt::Display for InputHints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for InputHints {
    type GlibType = ffi::GtkInputHints;

    fn into_glib(self) -> ffi::GtkInputHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkInputHints> for InputHints {
    unsafe fn from_glib(value: ffi::GtkInputHints) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InputHints {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_input_hints_get_type()) }
    }
}

impl glib::value::ValueType for InputHints {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InputHints {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InputHints {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkPickFlags")]
    pub struct PickFlags: u32 {
        #[doc(alias = "GTK_PICK_DEFAULT")]
        const DEFAULT = 0;
        #[doc(alias = "GTK_PICK_INSENSITIVE")]
        const INSENSITIVE = 1;
        #[doc(alias = "GTK_PICK_NON_TARGETABLE")]
        const NON_TARGETABLE = 2;
    }
}

impl fmt::Display for PickFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PickFlags {
    type GlibType = ffi::GtkPickFlags;

    fn into_glib(self) -> ffi::GtkPickFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkPickFlags> for PickFlags {
    unsafe fn from_glib(value: ffi::GtkPickFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PickFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_pick_flags_get_type()) }
    }
}

impl glib::value::ValueType for PickFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PickFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PickFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkPopoverMenuFlags")]
    pub struct PopoverMenuFlags: u32 {
        #[doc(alias = "GTK_POPOVER_MENU_NESTED")]
        const NESTED = 1;
    }
}

impl fmt::Display for PopoverMenuFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PopoverMenuFlags {
    type GlibType = ffi::GtkPopoverMenuFlags;

    fn into_glib(self) -> ffi::GtkPopoverMenuFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkPopoverMenuFlags> for PopoverMenuFlags {
    unsafe fn from_glib(value: ffi::GtkPopoverMenuFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PopoverMenuFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_popover_menu_flags_get_type()) }
    }
}

impl glib::value::ValueType for PopoverMenuFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PopoverMenuFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PopoverMenuFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkShortcutActionFlags")]
    pub struct ShortcutActionFlags: u32 {
        #[doc(alias = "GTK_SHORTCUT_ACTION_EXCLUSIVE")]
        const EXCLUSIVE = 1;
    }
}

impl fmt::Display for ShortcutActionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for ShortcutActionFlags {
    type GlibType = ffi::GtkShortcutActionFlags;

    fn into_glib(self) -> ffi::GtkShortcutActionFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkShortcutActionFlags> for ShortcutActionFlags {
    unsafe fn from_glib(value: ffi::GtkShortcutActionFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for ShortcutActionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_shortcut_action_flags_get_type()) }
    }
}

impl glib::value::ValueType for ShortcutActionFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ShortcutActionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for ShortcutActionFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkStateFlags")]
    pub struct StateFlags: u32 {
        #[doc(alias = "GTK_STATE_FLAG_NORMAL")]
        const NORMAL = 0;
        #[doc(alias = "GTK_STATE_FLAG_ACTIVE")]
        const ACTIVE = 1;
        #[doc(alias = "GTK_STATE_FLAG_PRELIGHT")]
        const PRELIGHT = 2;
        #[doc(alias = "GTK_STATE_FLAG_SELECTED")]
        const SELECTED = 4;
        #[doc(alias = "GTK_STATE_FLAG_INSENSITIVE")]
        const INSENSITIVE = 8;
        #[doc(alias = "GTK_STATE_FLAG_INCONSISTENT")]
        const INCONSISTENT = 16;
        #[doc(alias = "GTK_STATE_FLAG_FOCUSED")]
        const FOCUSED = 32;
        #[doc(alias = "GTK_STATE_FLAG_BACKDROP")]
        const BACKDROP = 64;
        #[doc(alias = "GTK_STATE_FLAG_DIR_LTR")]
        const DIR_LTR = 128;
        #[doc(alias = "GTK_STATE_FLAG_DIR_RTL")]
        const DIR_RTL = 256;
        #[doc(alias = "GTK_STATE_FLAG_LINK")]
        const LINK = 512;
        #[doc(alias = "GTK_STATE_FLAG_VISITED")]
        const VISITED = 1024;
        #[doc(alias = "GTK_STATE_FLAG_CHECKED")]
        const CHECKED = 2048;
        #[doc(alias = "GTK_STATE_FLAG_DROP_ACTIVE")]
        const DROP_ACTIVE = 4096;
        #[doc(alias = "GTK_STATE_FLAG_FOCUS_VISIBLE")]
        const FOCUS_VISIBLE = 8192;
        #[doc(alias = "GTK_STATE_FLAG_FOCUS_WITHIN")]
        const FOCUS_WITHIN = 16384;
    }
}

impl fmt::Display for StateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for StateFlags {
    type GlibType = ffi::GtkStateFlags;

    fn into_glib(self) -> ffi::GtkStateFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStateFlags> for StateFlags {
    unsafe fn from_glib(value: ffi::GtkStateFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for StateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_state_flags_get_type()) }
    }
}

impl glib::value::ValueType for StateFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for StateFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for StateFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkStyleContextPrintFlags")]
    pub struct StyleContextPrintFlags: u32 {
        #[doc(alias = "GTK_STYLE_CONTEXT_PRINT_NONE")]
        const NONE = 0;
        #[doc(alias = "GTK_STYLE_CONTEXT_PRINT_RECURSE")]
        const RECURSE = 1;
        #[doc(alias = "GTK_STYLE_CONTEXT_PRINT_SHOW_STYLE")]
        const SHOW_STYLE = 2;
        #[doc(alias = "GTK_STYLE_CONTEXT_PRINT_SHOW_CHANGE")]
        const SHOW_CHANGE = 4;
    }
}

impl fmt::Display for StyleContextPrintFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for StyleContextPrintFlags {
    type GlibType = ffi::GtkStyleContextPrintFlags;

    fn into_glib(self) -> ffi::GtkStyleContextPrintFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStyleContextPrintFlags> for StyleContextPrintFlags {
    unsafe fn from_glib(value: ffi::GtkStyleContextPrintFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for StyleContextPrintFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_style_context_print_flags_get_type()) }
    }
}

impl glib::value::ValueType for StyleContextPrintFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for StyleContextPrintFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for StyleContextPrintFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkTextSearchFlags")]
    pub struct TextSearchFlags: u32 {
        #[doc(alias = "GTK_TEXT_SEARCH_VISIBLE_ONLY")]
        const VISIBLE_ONLY = 1;
        #[doc(alias = "GTK_TEXT_SEARCH_TEXT_ONLY")]
        const TEXT_ONLY = 2;
        #[doc(alias = "GTK_TEXT_SEARCH_CASE_INSENSITIVE")]
        const CASE_INSENSITIVE = 4;
    }
}

impl fmt::Display for TextSearchFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TextSearchFlags {
    type GlibType = ffi::GtkTextSearchFlags;

    fn into_glib(self) -> ffi::GtkTextSearchFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTextSearchFlags> for TextSearchFlags {
    unsafe fn from_glib(value: ffi::GtkTextSearchFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TextSearchFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_text_search_flags_get_type()) }
    }
}

impl glib::value::ValueType for TextSearchFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TextSearchFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TextSearchFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GtkTreeModelFlags")]
    pub struct TreeModelFlags: u32 {
        #[doc(alias = "GTK_TREE_MODEL_ITERS_PERSIST")]
        const ITERS_PERSIST = 1;
        #[doc(alias = "GTK_TREE_MODEL_LIST_ONLY")]
        const LIST_ONLY = 2;
    }
}

impl fmt::Display for TreeModelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for TreeModelFlags {
    type GlibType = ffi::GtkTreeModelFlags;

    fn into_glib(self) -> ffi::GtkTreeModelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTreeModelFlags> for TreeModelFlags {
    unsafe fn from_glib(value: ffi::GtkTreeModelFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TreeModelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gtk_tree_model_flags_get_type()) }
    }
}

impl glib::value::ValueType for TreeModelFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TreeModelFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TreeModelFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
