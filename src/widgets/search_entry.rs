// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! An entry which shows a search icon

use ffi;

/// SearchEntry — An entry which shows a search icon
/*
* # Signal availables:
* * `search-changed` : Run Last
*/
struct_Widget!(SearchEntry);

impl SearchEntry {
    pub fn new() -> Option<SearchEntry> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_search_entry_new() };
        check_pointer!(tmp_pointer, SearchEntry)
    }
}

impl_drop!(SearchEntry);
impl_TraitWidget!(SearchEntry);

impl ::EntryTrait for SearchEntry {}
impl ::EditableTrait for SearchEntry {}
