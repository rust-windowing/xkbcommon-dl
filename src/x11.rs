use std::os::raw::c_int;

use x11_dl::xlib_xcb::xcb_connection_t;

use super::*;

pub const XKB_X11_MIN_MAJOR_XKB_VERSION: u16 = 1;
pub const XKB_X11_MIN_MINOR_XKB_VERSION: u16 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_x11_setup_xkb_extension_flags {
    /// Do not apply any flags.
    XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS = 0,
}

dlopen_external_library!(XkbCommonX11,
functions:
    fn xkb_x11_setup_xkb_extension(
        *mut xcb_connection_t,
        u16,
        u16,
        xkb_x11_setup_xkb_extension_flags,
        *mut u16,
        *mut u16,
        *mut u8,
        *mut u8
    ) -> c_int,
    fn xkb_x11_get_core_keyboard_device_id(*mut xcb_connection_t) -> i32,
    fn xkb_x11_keymap_new_from_device(
        *mut xkb_context,
        *mut xcb_connection_t,
        i32,
        xkb_keymap_compile_flags
    ) -> *mut xkb_keymap,
    fn xkb_x11_state_new_from_device(
        *mut xkb_keymap,
        *mut xcb_connection_t,
        i32
    ) -> *mut xkb_state,
);

lazy_static!(
    pub static ref XKBCOMMON_X11_OPTION: Option<XkbCommonX11> = {
        XkbCommonX11::open("libxkbcommon-x11.so").ok()
    };
    pub static ref XKBCOMMON_X11_HANDLE: &'static XkbCommonX11 = {
        XKBCOMMON_X11_OPTION.as_ref().expect("Library libxkbcommon-x11.so could not be loaded.")
    };
);
