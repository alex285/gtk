// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use Container;
use Widget;

pub type Fixed = Object<ffi::GtkFixed>;

unsafe impl Upcast<Widget> for Fixed { }
unsafe impl Upcast<Container> for Fixed { }
unsafe impl Upcast<Buildable> for Fixed { }

impl Fixed {
    pub fn new() -> Fixed {
        unsafe {
            Widget::from_glib_none(ffi::gtk_fixed_new()).downcast_unchecked()
        }
    }

    pub fn move_<T: Upcast<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_move(self.to_glib_none().0, widget.upcast().to_glib_none().0, x, y);
        }
    }

    pub fn put<T: Upcast<Widget>>(&self, widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_fixed_put(self.to_glib_none().0, widget.upcast().to_glib_none().0, x, y);
        }
    }

}

impl types::StaticType for Fixed {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_fixed_get_type()) }
    }
}
