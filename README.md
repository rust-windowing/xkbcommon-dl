# xkbcommon-dl

[![](http://meritbadge.herokuapp.com/xkbcommon-dl)](https://crates.io/crates/xkbcommon-dl)
[![Docs.rs](https://docs.rs/xkbcommon-dl/badge.svg)](https://docs.rs/xkbcommon-dl)
[![Build Status](https://travis-ci.org/francesca64/xkbcommon-dl.svg?branch=master)](https://travis-ci.org/francesca64/xkbcommon-dl)

Dynamically loaded xkbcommon and xkbcommon-x11 Rust bindings.

lib.rs is an almost verbatim copy of [wayland-kbd's xkbcommon module](https://github.com/Smithay/wayland-kbd/blob/master/src/ffi/mod.rs), along with [update-keysyms.sh](https://github.com/Smithay/wayland-kbd/blob/master/update-keysyms.sh).

## [Documentation](https://docs.rs/xkbcommon-dl)

Docs for the master branch are available [here](https://francesca64.github.io/xkbcommon-dl/docs/xkbcommon_dl/).

For examples of practical usage, see its use in [winit](https://github.com/tomaka/winit/blob/master/src/platform/linux/x11/xkb/mod.rs).
