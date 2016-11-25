use std::os::raw::{c_void, c_char};

#[repr(C)]
pub struct GUDockWidgetFuncs {
    pub is_floating: extern "C" fn(dock_widget: *const GUDockWidget) -> bool,
    pub set_floating: extern "C" fn(dock_widget: *const GUDockWidget, floating: bool),
    pub set_widget: extern "C" fn(dock_widget: *const GUDockWidget, widget: *const GUWidget),
}

#[repr(C)]
pub struct GUObjectFuncs {
    pub connect: extern "C" fn(sender: *const c_void, id: *const c_char, reciver: *const c_void, func: *const c_void) -> i32,
}

#[repr(C)]
pub struct GUWidgetFuncs {
    pub set_size: extern "C" fn(widget: *const GUWidget, width: i32, height: i32),
}

#[repr(C)]
pub struct GUWindowFuncs {
    pub set_title_foo: extern "C" fn(name: *const c_char),
}

#[repr(C)]
pub struct GUMainWindowFuncs {
    pub add_dock_widget: extern "C" fn(win: *const GUMainWindow, area: u32, widget: *const GUDockWidget),
}

#[repr(C)]
pub struct GUPushButtonFuncs {
    pub set_default: extern "C" fn(button: *const GUPushButton, state: i32),
}

#[repr(C)]
pub struct GUApplicationFuncs {
    pub run: extern "C" fn(p: *const c_void) -> i32,
}

#[repr(C)]
pub struct GUObject {
    pub p: *const c_void,
}

#[repr(C)]
pub struct GUWidget {
    pub object: *const GUObject,
}

#[repr(C)]
pub struct GUDockWidget {
    pub base: *const GUWidget,
    pub privd: *const c_void,
}

#[repr(C)]
pub struct GUWindow {
    pub base: *const GUWidget,
}

#[repr(C)]
pub struct GUPushButton {
    pub base: *const GUWidget,
}

#[repr(C)]
pub struct GUMainWindow {
    pub base: *const GUWidget,
}

#[repr(C)]
pub struct GUApplication {
    pub p: *const c_void,
}

#[repr(C)]
pub struct Wrui {
    pub api_version: u64,
    pub application_create: extern "C" fn() -> *const GUApplication,
    pub window_create: extern "C" fn(parent: *const GUWidget) -> *const GUWindow,
    pub push_button_create: extern "C" fn(parent: *const GUWidget) -> *const GUPushButton,
    pub main_window_create: extern "C" fn(parent: *const GUWidget) -> *const GUMainWindow,
    pub dock_widget_create: extern "C" fn(parent: *const GUWidget) -> *const GUDockWidget,
    pub object_funcs: *const GUObjectFuncs,
    pub widget_funcs: *const GUWidgetFuncs,
    pub main_window_funcs: *const GUMainWindowFuncs,
    pub push_button_funcs: *const GUPushButtonFuncs,
    pub application_funcs: *const GUApplicationFuncs,
}

extern "C" {
    pub fn wrui_get() -> *mut Wrui;
}
