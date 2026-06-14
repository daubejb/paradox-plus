import wave
import struct
import math
import os

def make_beep(filename, frequency, duration=0.1, volume=0.5, sample_rate=44100):
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    with wave.open(filename, 'w') as w:
        w.setnchannels(1) # mono
        w.setsampwidth(2) # 16-bit
        w.setframerate(sample_rate)
        
        num_samples = int(sample_rate * duration)
        for i in range(num_samples):
            # Sine wave sample
            value = int(volume * 32767.0 * math.sin(2.0 * math.pi * frequency * (i / sample_rate)))
            data = struct.pack('<h', value)
            w.writeframesraw(data)

def main():
    sounds_dir = "assets/sounds"
    print(f"Generating test audio assets in {sounds_dir}...")
    
    sounds = {
        "ui_hover.wav": (600, 0.05),
        "ui_click.wav": (800, 0.08),
        "dice_roll.wav": (300, 0.25),
        "wager_draft.wav": (450, 0.15),
        "wager_place.wav": (500, 0.15),
        "land_fairway.wav": (350, 0.12),
        "land_rough.wav": (200, 0.15),
        "land_bunker.wav": (150, 0.15),
        "land_water.wav": (100, 0.2),
        "land_ob.wav": (80, 0.25),
        "hole_complete.wav": (700, 0.4),
        "match_complete.wav": (900, 0.6),
    }

    for name, (freq, dur) in sounds.items():
        filepath = os.path.join(sounds_dir, name)
        make_beep(filepath, freq, duration=dur)
        print(f"  Generated {name} (Freq: {freq}Hz, Dur: {dur}s)")
        
    print("Done!")

if __name__ == "__main__":
    main()
