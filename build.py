# Маленькая утилита, которая запускает компиляюцию Rust-библиотеки, а затем копирует готовый DLL-файл в папку builded_dlls
# Увы я не кодер на питоне, и это мой первый опыт написания скриптов на нём.

import os
import shutil

dllname = "gbinarymanager"

def createDir(dir):
    if not os.path.exists(dir):
        os.makedirs(dir)

def startBuild(flag):
    os.system("cargo build --target i686-pc-windows-msvc" + (flag and "--" + flag or ""))

def copy():
    dirFiles = [f for f in os.listdir("builded_dlls/backups") if os.path.isfile(os.path.join("builded_dlls/backups", f))]
    
    folder = "builded_dlls/"

    oldPath = folder + dllname + ".dll"
    if os.path.isfile(oldPath):
        newName = dllname + "_" + str(len(dirFiles)+1) + ".dll"
        newPath = folder + newName
        os.rename(oldPath, newPath)
        shutil.move(newPath, folder + "backups/" + newName)

    oldPath = "target/i686-pc-windows-msvc/debug/" + dllname + ".dll"
    if os.path.isfile(oldPath):
        shutil.move(oldPath, folder)

if __name__ == "__main__":
    print("check for folders")
    createDir("builded_dlls")
    createDir("builded_dlls/backups")
    print("start build...")
    print("enter the build flag, or just press enter")
    startBuild(input())
    print("try to copy...")
    copy()
    print("script end")