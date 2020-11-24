// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct X11DisplayManager(Object<ffi::GdkX11DisplayManager, ffi::GdkX11DisplayManagerClass>) @extends gdk::DisplayManager;

    match fn {
        get_type => || ffi::gdk_x11_display_manager_get_type(),
    }
}

impl X11DisplayManager {}

impl fmt::Display for X11DisplayManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11DisplayManager")
    }
}