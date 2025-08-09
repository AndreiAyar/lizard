import PyInstaller.__main__
import os

# Get the current directory (src-python)
current_dir = os.path.dirname(os.path.abspath(__file__))

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
    '--hidden-import=simpleaudio',
    '--hidden-import=pynput',
    '--console'
])