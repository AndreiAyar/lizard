from fastapi import FastAPI
from pynput import keyboard

app = FastAPI()

@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}