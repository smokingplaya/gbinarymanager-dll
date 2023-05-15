// Просто держу в курсе того что я не программист на расте. Поэтому если увидите говнокодик - не обижайтесь :)

#![feature(c_unwind)]

use std::fs::File;
use std::io::prelude::*;

use gmod::gmcl::override_stdout;
use gmod::lua::State;

#[macro_use] extern crate gmod;

fn create_file(name: &str, content: &str) -> std::io::Result<()> {
	let normal_name : String = format!("{}{}", "garrysmod/lua/bin/", name);
	let normal_name_str : &str = &normal_name[..];

	let mut file = File::create(normal_name_str)?;
    file.write_all(content.as_bytes())?;

	Ok(())
}

unsafe extern "C-unwind" fn save_binary(lua: State) -> i32 {
	let name = lua.check_string(1);
	let msg = lua.check_string(2);

	let _ = create_file(name.as_ref(), msg.as_ref());

	0
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
	if lua.is_client() {
      override_stdout()
    }

    lua.get_global(lua_string!("gbinarymanager"));
	if lua.is_nil(-1) {
		lua.pop();
		lua.new_table();
	}

	lua.push_string(env!("CARGO_PKG_VERSION"));
	lua.set_field(-2, lua_string!("version"));

	lua.push_function(save_binary);
	lua.set_field(-2, lua_string!("SaveBinary"));

	lua.set_global(lua_string!("gbinarymanager"));

    0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: State) -> i32 {
    0
}
