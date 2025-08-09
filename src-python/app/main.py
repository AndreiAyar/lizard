from fastapi import FastAPI
import sys
import logging
import os
import time
import json
import uvicorn
from fastapi.middleware.cors import CORSMiddleware
import signal

# Set up minimal logging initially
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")
logger = logging.getLogger(__name__)

main_dir = os.path.dirname(os.path.abspath(__file__))
sound_path = os.path.join(main_dir, "sounds", "lizard_cleaned.wav")
settings_file = os.path.join(main_dir, "data", "settings.json")
app_status = "on"

# Lazy-loaded globals
sound_to_play_on_k_press = None
listener = None
last_played = 0
backend_ready = False

from contextlib import asynccontextmanager

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    logger.info("=== LIZARD BACKEND STARTING ===")
    
    # Start background initialization
    import asyncio
    asyncio.create_task(initialize_backend())
    
    yield
    # Shutdown
    global listener
    if listener:
        listener.stop()
    logger.info("=== LIZARD BACKEND SHUTTING DOWN ===")

async def initialize_backend():
    """Initialize heavy components asynchronously"""
    global sound_to_play_on_k_press, listener, backend_ready
    
    try:
        # Load audio file
        import simpleaudio as sa
        sound_to_play_on_k_press = sa.WaveObject.from_wave_file(sound_path)
        
        # Start keyboard listener
        from pynput import keyboard
        listener = keyboard.Listener(on_press=on_press)
        listener.start()
        
        backend_ready = True
        logger.info("Backend initialization complete")
        
    except Exception as e:
        logger.error(f"Backend initialization failed: {e}")

app = FastAPI(lifespan=lifespan)

def on_press(key):
    if app_status == "off" or not backend_ready:
        return
    global last_played
    now = time.time()
    if now - last_played > DEBOUNCE_DELAY:
        if sound_to_play_on_k_press:
            sound_to_play_on_k_press.play()
        last_played = now


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


origins = ["*"]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/")
def read_root():
    return {"message": "Hello from Python backend!", "ready": backend_ready}


@app.get("/health")
def health_check():
    return {"status": "running", "ready": backend_ready}


@app.post("/toggle")
def toggle_app():
    global app_status
    if app_status == "on":
        app_status = "off"
    else:
        app_status = "on"
    return {"app_status": app_status}


@app.get("/settings")
def get_settings():
    return load_settings()


@app.post("/settings")
def post_settings(new_settings: dict):
    return update_settings(new_settings)

def signal_handler(signum, frame):
    """Handle termination signals"""
    logger.info(f"Received signal {signum}, shutting down...")
    global listener
    if listener:
        listener.stop()
    sys.exit(0)

# Register signal handlers
signal.signal(signal.SIGTERM, signal_handler)
signal.signal(signal.SIGINT, signal_handler)

# START THE SERVER
if __name__ == "__main__":
    logger.info("=== STARTING UVICORN SERVER ===")
    try:
        uvicorn.run(
            app,
            host="0.0.0.0",
            port=8000,
            log_level="warning"
        )
    except KeyboardInterrupt:
        logger.info("Received keyboard interrupt, shutting down...")
    except Exception as e:
        logger.error(f"Failed to start server: {e}")
        raise
    finally:
        if listener:
            listener.stop()