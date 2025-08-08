from fastapi import FastAPI
from pynput import keyvoard

app = FastAPI()

@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!"}