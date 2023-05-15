/*
	lua gbinarymanager table:

	gbinarymanager.Version (var)
	gbinarymanager.saveDLL (function)
*/

#![feature(c_unwind)]

extern crate steamlocate;
use steamlocate::SteamDir;

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use gmod::gmcl::override_stdout;
use gmod::lua::State;

#[macro_use] extern crate gmod;

// main

unsafe fn devprint(lua:State, msg: &str) -> i32 {
    lua.get_global(lua_string!("print"));
    lua.push_string(msg);
    lua.call(1, 1);
    0
}


unsafe fn safefile(lua: State) -> i32 {
	0
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
	if lua.is_client() {
      override_stdout()
    }

	println!("ЛЮЬИИИТИТИ");

    lua.get_global(lua_string!("gbinarymanager"));
	if lua.is_nil(-1) {
		lua.pop();
		lua.new_table();
	}

	lua.set_global(lua_string!("gbinarymanager"));

    0
}

#[gmod13_close]
fn gmod13_close(lua: gmod::lua::State) -> i32 {
    0
}