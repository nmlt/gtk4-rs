// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for blanket imports.

pub use crate::auto::traits::*;

pub use crate::accessible::AccessibleExtManual;
pub use crate::actionable::ActionableExtManual;
pub use crate::cell_area::CellAreaExtManual;
pub use crate::cell_editable::CellEditableExtManual;
pub use crate::cell_layout::CellLayoutExtManual;
pub use crate::cell_renderer::CellRendererExtManual;
pub use crate::color_chooser::ColorChooserExtManual;
pub use crate::combo_box::ComboBoxExtManual;
pub use crate::constraint_guide::ConstraintGuideExtManual;
pub use crate::dialog::DialogExtManual;
pub use crate::drawing_area::DrawingAreaExtManual;
pub use crate::editable::EditableExtManual;
pub use crate::entry::EntryExtManual;
pub use crate::entry_buffer::EntryBufferExtManual;
pub use crate::entry_completion::EntryCompletionExtManual;
pub use crate::font_chooser::FontChooserExtManual;
pub use crate::im_context::IMContextExtManual;
pub use crate::im_context_simple::IMContextSimpleExtManual;
pub use crate::list_store::GtkListStoreExtManual;
pub use crate::media_stream::MediaStreamExtManual;
pub use crate::native_dialog::NativeDialogExtManual;
pub use crate::overlay::OverlayExtManual;
pub use crate::scale::ScaleExtManual;
pub use crate::shortcut_trigger::ShortcutTriggerExtManual;
pub use crate::spin_button::SpinButtonExtManual;
pub use crate::text_buffer::TextBufferExtManual;
pub use crate::text_view::TextViewExtManual;
pub use crate::tree_model::TreeModelExtManual;
pub use crate::tree_model_filter::TreeModelFilterExtManual;
pub use crate::tree_sortable::TreeSortableExtManual;
pub use crate::tree_store::TreeStoreExtManual;
pub use crate::tree_view::TreeViewExtManual;
pub use crate::widget::{InitializingWidgetExt, WidgetExtManual};

#[doc(hidden)]
pub use gdk::prelude::*;
#[doc(hidden)]
pub use gdk_pixbuf::prelude::*;
#[doc(hidden)]
pub use gio::prelude::*;
#[doc(hidden)]
pub use glib::prelude::*;
#[doc(hidden)]
pub use graphene::prelude::*;
#[doc(hidden)]
pub use gsk::prelude::*;
#[doc(hidden)]
pub use pango::prelude::*;
