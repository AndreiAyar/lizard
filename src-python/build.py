import PyInstaller.__main__
import os

PyInstaller.__main__.run([
    'app/main.py',
    '--onefile',
    '--name=lizard-backend',
    '--distpath=../src-tauri/binaries',
    '--workpath=build',
    '--specpath=build',
    '--add-data=app/sounds:sounds',
    '--add-data=app/data:data',
    '--hidden-import=uvicorn',
    '--hidden-import=fastapi',
    '--console'
])