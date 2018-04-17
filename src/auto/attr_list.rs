// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use AttrIterator;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct AttrList(Shared<ffi::PangoAttrList>);

    match fn {
        ref => |ptr| ffi::pango_attr_list_ref(ptr),
        unref => |ptr| ffi::pango_attr_list_unref(ptr),
        get_type => || ffi::pango_attr_list_get_type(),
    }
}

impl AttrList {
    pub fn new() -> AttrList {
        unsafe {
            from_glib_full(ffi::pango_attr_list_new())
        }
    }

    pub fn copy(&self) -> Option<AttrList> {
        unsafe {
            from_glib_full(ffi::pango_attr_list_copy(self.to_glib_none().0))
        }
    }

    //pub fn filter<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/AttrFilterFunc, data: P) -> Option<AttrList> {
    //    unsafe { TODO: call ffi::pango_attr_list_filter() }
    //}

    pub fn get_iterator(&self) -> Option<AttrIterator> {
        unsafe {
            from_glib_full(ffi::pango_attr_list_get_iterator(self.to_glib_none().0))
        }
    }

    pub fn splice(&self, other: &AttrList, pos: i32, len: i32) {
        unsafe {
            ffi::pango_attr_list_splice(self.to_glib_none().0, other.to_glib_none().0, pos, len);
        }
    }
}

impl Default for AttrList {
    fn default() -> Self {
        Self::new()
    }
}
