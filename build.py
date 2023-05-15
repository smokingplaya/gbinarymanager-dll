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
    createDir("builded_dlls")
    createDir("builded_dlls/backups")
    startBuild(input())
    copy()
