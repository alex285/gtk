// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::{from_glib_none, ToGlibPtr};
use cast::GTK_LINKBUTTON;
use ffi;
use glib::{to_bool, to_gboolean};

/*
* # Availables signals :
* * `activate-link` : Run Last
*/
struct_Widget!(LinkButton);

impl LinkButton {
    pub fn new(uri: &str) -> Option<LinkButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_link_button_new(uri.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, LinkButton)
    }

    pub fn new_with_label(uri: &str, label: &str) -> Option<LinkButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_link_button_new_with_label(uri.to_glib_none().0, label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, LinkButton)
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(GTK_LINKBUTTON(self.pointer)))
        }
    }

    pub fn set_uri(&self, uri: &str) -> () {
        unsafe {
            ffi::gtk_link_button_set_uri(GTK_LINKBUTTON(self.pointer), uri.to_glib_none().0)
        }
    }

    pub fn set_visited(&self, visited: bool) -> () {
        unsafe { ffi::gtk_link_button_set_visited(GTK_LINKBUTTON(self.pointer), to_gboolean(visited)); }
    }

    pub fn get_visited(&self) -> bool {
        unsafe { to_bool(ffi::gtk_link_button_get_visited(GTK_LINKBUTTON(self.pointer))) }
    }
}

impl_drop!(LinkButton);
impl_TraitWidget!(LinkButton);

impl ::ContainerTrait for LinkButton {}
impl ::ButtonTrait for LinkButton {}
