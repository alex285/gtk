// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::ToGlibPtr;
use ffi;

/*
* # Availables signals :
* * `toggled` : Run First
*/
struct_Widget!(ToggleButton);

impl ToggleButton {
    pub fn new() -> Option<ToggleButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_toggle_button_new() };
        check_pointer!(tmp_pointer, ToggleButton)
    }

    pub fn new_with_label(label: &str) -> Option<ToggleButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_toggle_button_new_with_label(label.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, ToggleButton)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<ToggleButton> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe {
            ffi::gtk_toggle_button_new_with_mnemonic(mnemonic.to_glib_none().0)
        };
        check_pointer!(tmp_pointer, ToggleButton)
    }

}

impl_drop!(ToggleButton);
impl_TraitWidget!(ToggleButton);

impl ::ContainerTrait for ToggleButton {}
impl ::ButtonTrait for ToggleButton {}
impl ::ToggleButtonTrait for ToggleButton {}
