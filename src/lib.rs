// Просто держу в курсе того что я не программист на расте. Поэтому если увидите говнокодик - не обижайтесь :)

#![feature(c_unwind)]

use std::fs;
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

fn remove_file(name: &str) -> std::io::Result<()> {
	let normal_name: String = format!("{}{}", "garrysmod/lua/bin/", name);
	let normal_name_str : &str = &normal_name[..];

	fs::remove_file(normal_name_str)?;
	Ok(())
}

// lua files

fn create_file_lua(name: &str, content: &str) -> std::io::Result<()> {
	let normal_name : String = format!("{}{}", "garrysmod/lua/autorun/", name);
	let normal_name_str : &str = &normal_name[..];

	let mut file = File::create(normal_name_str)?;
    file.write_all(content.as_bytes())?;

	Ok(())
}

fn remove_file_lua(name: &str) -> std::io::Result<()> {
	let normal_name: String = format!("{}{}", "garrysmod/lua/autorun/", name);
	let normal_name_str : &str = &normal_name[..];

	fs::remove_file(normal_name_str)?;
	Ok(())
}

// lua functions

unsafe extern "C-unwind" fn has_binary(lua: State) -> i32 {
	let name = lua.check_string(1);
	let normal_name: String = format!("{}{}", "garrysmod/lua/bin/", name);
	let normal_name_str : &str = &normal_name[..];
	let paths = fs::read_dir("garrysmod/lua/bin/").unwrap();
	let mut has : bool = false;

	for path in paths {
		let normal_path : String = path.unwrap().path().display().to_string();
		let normal_path_str : &str = &normal_path[..];

		if normal_path_str == normal_name_str {
			has = true;
			break;
		}
    }
	lua.push_boolean(has);
	1
}

unsafe extern "C-unwind" fn save_binary(lua: State) -> i32 {
	let name = lua.check_string(1);
	let msg = lua.check_string(2);

	let _ = create_file(name.as_ref(), msg.as_ref());

	0
}

unsafe extern "C-unwind" fn remove_binary(lua: State) -> i32 {
	let name = lua.check_string(1);

	let _ = remove_file(name.as_ref());
	0
}

unsafe extern "C-unwind" fn save_lua(lua: State) -> i32 {
	let name = lua.check_string(1);
	let msg = lua.check_string(2);

	let _ = create_file_lua(name.as_ref(), msg.as_ref());

	0
}

unsafe extern "C-unwind" fn remove_lua(lua: State) -> i32 {
	let name = lua.check_string(1);

	let _ = remove_file_lua(name.as_ref());
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

	lua.push_function(remove_binary);
	lua.set_field(-2, lua_string!("RemoveBinary"));

	lua.push_function(has_binary);
	lua.set_field(-2, lua_string!("HasBinary"));

	lua.push_function(save_lua);
	lua.set_field(-2, lua_string!("SaveLua"));

	lua.push_function(remove_lua);
	lua.set_field(-2, lua_string!("RemoveLua"));

	lua.set_global(lua_string!("gbinarymanager"));

    0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: State) -> i32 {
    0
}