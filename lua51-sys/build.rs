//extern crate pkg_config;
extern crate cc;

use std::env;

fn main() {
//    match pkg_config::find_library("lua5.1") {
//        Ok(_) => return,
//        Err(..) => {}
//    };

    let mut build = cc::Build::new();

    if env::var("CARGO_CFG_TARGET_OS") == Ok("linux".to_string()) {
        // Enable `io.popen` support
        build.define("LUA_USE_LINUX", None);
    }

    build
        .file("lua/lapi.c")
        .file("lua/lauxlib.c")
        .file("lua/lbaselib.c")
        .file("lua/lcode.c")
        .file("lua/ldblib.c")
        .file("lua/ldebug.c")
        .file("lua/ldo.c")
        .file("lua/ldump.c")
        .file("lua/lfunc.c")
        .file("lua/lgc.c")
        .file("lua/linit.c")
        .file("lua/liolib.c")
        .file("lua/llex.c")
        .file("lua/lmathlib.c")
        .file("lua/lmem.c")
        .file("lua/loadlib.c")
        .file("lua/lobject.c")
        .file("lua/lopcodes.c")
        .file("lua/loslib.c")
        .file("lua/lparser.c")
        .file("lua/lstate.c")
        .file("lua/lstring.c")
        .file("lua/lstrlib.c")
        .file("lua/ltable.c")
        .file("lua/ltablib.c")
        .file("lua/ltm.c")
//        .file("lua/lua.c")
//        .file("lua/luac.c")
        .file("lua/lundump.c")
        .file("lua/lvm.c")
        .file("lua/lzio.c")
        .file("lua/print.c")
        .file("lua/compat-5.2.c")

        .include("lua")
        .compile("liblua.a");
}
