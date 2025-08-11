import PyInstaller.__main__
import os
# import platform

# Get the current directory (src-python)
current_dir = os.path.dirname(os.path.abspath(__file__))

# Build absolute paths
app_dir = os.path.join(current_dir, 'app')
sounds_dir = os.path.join(app_dir, 'sounds')
data_dir = os.path.join(app_dir, 'data')
main_py = os.path.join(app_dir, 'main.py')
dist_path = os.path.join(current_dir, '..', 'src-tauri', 'binaries')

# Create directories if they don't exist
os.makedirs(sounds_dir, exist_ok=True)
os.makedirs(data_dir, exist_ok=True)
os.makedirs(dist_path, exist_ok=True)

# # Platform-specific executable name for Tauri
# system = platform.system().lower()
# machine = platform.machine().lower()

# if system == "windows":
#     target_triple = "x86_64-pc-windows-msvc"
#     exe_name = f"lizard-backend-{target_triple}.exe"
# elif system == "darwin":
#     if machine == "arm64":
#         target_triple = "aarch64-apple-darwin"
#     else:
#         target_triple = "x86_64-apple-darwin"
#     exe_name = f"lizard-backend-{target_triple}"
# else:  # Linux
#     target_triple = "x86_64-unknown-linux-gnu"
#     exe_name = f"lizard-backend-{target_triple}"

exe_name = 'lizard-backend'

PyInstaller.__main__.run([
    main_py,
    '--onefile',
    f'--name={exe_name}',
    f'--distpath={dist_path}',
    '--workpath=build',
    '--specpath=build',
    f'--add-data={sounds_dir}{os.pathsep}sounds',
    f'--add-data={data_dir}{os.pathsep}data',
    '--hidden-import=uvicorn',
    '--hidden-import=fastapi',
    '--hidden-import=simpleaudio',
    '--hidden-import=pynput',
    '--console'
])

print(f"Built binary: {exe_name}")