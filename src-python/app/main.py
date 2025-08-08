from fastapi import FastAPI
from pynput import keyboard
import simpleaudio as sa
import os

script_dir = os.path.dirname(os.path.abspath(__file__))
sound_path = os.path.join(script_dir, "sounds", "lizard.wav")
sound_to_play_on_k_press = sa.WaveObject.from_wave_file(sound_path)


def on_press(key):
    print("key pressed {0}".format(key))
play_obj = sound_to_play_on_k_press.play()


# play_obj.wait_done() blocking not needed as for now..


listener = keyboard.Listener(on_press=on_press)
listener.start()


app = FastAPI()


@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}
