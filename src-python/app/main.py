from fastapi import FastAPI
from pynput import keyboard
import simpleaudio as sa
import os
import time
import json
from fastapi.middleware.cors import CORSMiddleware


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
            # return
            sound_to_play_on_k_press.play()
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


def save_settings(settings):
    with open(settings_file, "w") as f:
        json.dump(settings, f)


def update_settings(new_settings: dict):
    global settings_data, DEBOUNCE_DELAY
    if new_settings is None or not isinstance(new_settings, dict):
        return {"error": "No settings provided."}
    debounce = new_settings.get("debounce_delay")
    if debounce is not None:
        try:
            debounce = float(debounce)
        except ValueError:
            return {"error": "Invalid debounce delay value. Must be a number."}
        if debounce < 0:
            return {"error": "Debounce delay must be a non-negative number."}
        settings_data["debounce_delay"] = debounce
        save_settings(settings_data)
        DEBOUNCE_DELAY = debounce
    return {"message": "Settings updated"}


settings_data = load_settings()
DEBOUNCE_DELAY = settings_data.get("debounce_delay", 0.1)

app = FastAPI()

origins = [
    "http://localhost",
    "http://localhost:8080",
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}


@app.get("/settings")
def get_settings():
    return settings_data


@app.post("/settings")
def post_settings(new_settings: dict):
    return update_settings(new_settings)
