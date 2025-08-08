from fastapi import FastAPI
from pynput import keyboard
import simpleaudio as sa


sound_to_play_on_k_press = sa.WaveObject.from_wave_file("./sounds/lizard.mp3")


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
