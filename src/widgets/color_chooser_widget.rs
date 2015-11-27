// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkColorChooserWidget — A widget for choosing colors

use ffi;
use FFIWidget;

struct_Widget!(ColorChooserWidget);

impl ColorChooserWidget {
    pub fn new() -> Option<ColorChooserWidget> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_color_chooser_widget_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(ColorChooserWidget);
impl_TraitWidget!(ColorChooserWidget);

impl ::ContainerTrait for ColorChooserWidget {}
impl ::BoxTrait for ColorChooserWidget {}
