# GBinaryManager DLL
The dll file that lets you create and save files in the garrysmod/lua/bin folder

This is my first experience writing code in Rust.

# May I download this module?
Yep, sure. Click on link below.

[Latest release 0.2.0](https://github.com/smokingplaya/gbinarymanager-dll/releases/tag/0.2.0)

# How to build

1. Download rustc
2. Download and unzip this repo
3. Enter to the directory, where you unzip repo.
4. Open terminal/cmd
5. Write "rustup override set nightly"
6. Open build.bat

I might have forgotten something, so in case of errors you can contact me at discord smokingplaya#4961

# Lua

To make the module work, you need to create a file in **/autorun/client** with any name and with the following content

```lua
require("gbinarymanager")
```

## What features does the module add?

```lua
gbinarymanager.SaveBinary(name, content)
gbinarymanager.RemoveBinary(name)

gbinarymanager.SaveLua(name, content)
gbinarymanager.RemoveLua(name)

gbinarymanager.HasBinary(name)
```
## Credits

[WilliamVenner aka billy](https://github.com/WilliamVenner) for [gmod-rs](https://github.com/WilliamVenner/gmod-rs/)
