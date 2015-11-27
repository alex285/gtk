// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Application chooser widget that can be embedded in other widgets

use cast::GTK_APP_CHOOSER_WIDGET;
use ffi;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::{to_bool, to_gboolean};

struct_Widget!(AppChooserWidget);

impl AppChooserWidget {
    pub fn new(content_type: &str) -> Option<AppChooserWidget> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_app_chooser_widget_new(content_type.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, AppChooserWidget)
    }

    pub fn set_show_default(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_default(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_default(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_default(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_recommended(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_recommended(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_recommended(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_recommended(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_fallback(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_fallback(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_fallback(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_fallback(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_other(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_other(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_other(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_other(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_show_all(&self, setting: bool) {
        unsafe { ffi::gtk_app_chooser_widget_set_show_all(GTK_APP_CHOOSER_WIDGET(self.pointer), to_gboolean(setting)) }
    }

    pub fn get_show_all(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_app_chooser_widget_get_show_all(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }

    pub fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(GTK_APP_CHOOSER_WIDGET(self.pointer), text.to_glib_none().0)
        }
    }

    pub fn get_default_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_app_chooser_widget_get_default_text(GTK_APP_CHOOSER_WIDGET(self.pointer)))
        }
    }
}

impl_drop!(AppChooserWidget);
impl_TraitWidget!(AppChooserWidget);

impl ::ContainerTrait for AppChooserWidget {}
impl ::BoxTrait for AppChooserWidget {}
