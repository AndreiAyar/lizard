from fastapi import FastAPI
from pynput import keyboard
import simpleaudio as sa

def on_press(key):
    print("key pressed {0}".format(key))
 


listener = keyboard.Listener(on_press=on_press)
listener.start()


app = FastAPI()


@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}
