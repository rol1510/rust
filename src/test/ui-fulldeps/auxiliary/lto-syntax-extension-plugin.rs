// force-host

#![feature(rustc_private)]

extern crate rustc_middle;
extern crate rustc_driver;

use rustc_driver::plugin::Registry;

#[no_mangle]
fn __rustc_plugin_registrar(_reg: &mut Registry) {}
