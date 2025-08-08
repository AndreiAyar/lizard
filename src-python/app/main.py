from fastapi import FastAPI
from pynput import keyboard
import simpleaudio as sa
import os
import time
import json

main_dir = os.path.dirname(os.path.abspath(__file__))
sound_path = os.path.join(main_dir, "sounds", "lizard_cleaned.wav")
settings_file = os.path.join(main_dir, "data", "settings.json")

sound_to_play_on_k_press = sa.WaveObject.from_wave_file(sound_path)

last_played = 0


def on_press(key):
    print("key pressed {0}".format(key))

    global last_played
    now = time.time()
    if now - last_played > DEBOUNCE_DELAY:
        if sound_to_play_on_k_press:
            return
            # sound_to_play_on_k_press.play()
        last_played = now


# play_obj.wait_done() blocking not needed as for now..


listener = keyboard.Listener(on_press=on_press)
listener.start()


def load_settings():
    if os.path.isfile(settings_file):
        with open(settings_file, "r") as f:
            return json.load(f)
    else:
        return {"debounce_delay": 0.3}


settings_data = load_settings()
DEBOUNCE_DELAY = settings_data.get("debounce_delay", 0.1)

app = FastAPI()


@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}


@app.get("/settings")
def get_settings():
    return settings_data
