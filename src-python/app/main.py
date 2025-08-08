from fastapi import FastAPI
from pynput import keyboard
import simpleaudio as sa
import os
import time

script_dir = os.path.dirname(os.path.abspath(__file__))
sound_path = os.path.join(script_dir, "sounds", "lizard_cleaned.wav")
sound_to_play_on_k_press = sa.WaveObject.from_wave_file(sound_path)
last_played = 0
DEBOUNCE_DELAY = 0  # seconds
 

def on_press(key):
    print("key pressed {0}".format(key))

    global last_played
    now = time.time()
    if now - last_played > DEBOUNCE_DELAY:
        if sound_to_play_on_k_press:
            sound_to_play_on_k_press.play()
        last_played = now

# play_obj.wait_done() blocking not needed as for now..


listener = keyboard.Listener(on_press=on_press)
listener.start()


app = FastAPI()


@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}
