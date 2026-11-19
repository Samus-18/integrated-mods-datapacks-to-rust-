#!/usr/bin/env python3
"""Generate original five-class/three-stage positional magic SFX.

No sampled, copyrighted or external material. Deterministic standard-library
oscillator/noise synthesis then FFmpeg Vorbis encoding.
Re-run before `cargo horizon-mod build --wasm`. No paid generator required.
"""
from pathlib import Path
import math
import random
import struct
import subprocess
import shutil
import tempfile
import wave
import json
import argparse

RATE=24000
KINDS=("phoenix","glacier","worldroot","thunder","astral")
STAGES=("charge","cast","impact")
BASE=(95.0,220.0,108.0,62.0,52.0)
OUT=Path(__file__).resolve().parents[1]/"assets"/"aetheria_preview"
TWO_PI=2*math.pi

def synthesize(kind:int,stage:int):
    rng=random.Random(10871+kind*237+stage*413)
    seconds=(0.80,0.48,1.18)[stage]
    count=int(seconds*RATE)
    phase=phase_hi=0.0
    low_noise=0.0
    values=[]
    base=BASE[kind]
    for i in range(count):
        t=i/RATE
        p=t/seconds
        # A fast ramp, with natural release; never hard-clip a sound at its end.
        fadein=min(1.0,t/0.025)
        release=max(0.0,1.0-p)**(1.5 if stage!=1 else 2.5)
        env=fadein*release
        n=rng.uniform(-1.0,1.0)
        low_noise=0.96*low_noise+0.04*n
        high=n-low_noise
        if stage==0:  # charge: growing drone with increasing harmonic tension
            f=base*(1.0+0.35*p+0.035*math.sin(16*t))
            brightness=0.3+0.45*p
            phase+=TWO_PI*f/RATE
            phase_hi+=TWO_PI*f*2.01/RATE
            wave0=(math.sin(phase)*0.48+
                    math.sin(phase_hi+0.1*math.sin(7*t))*brightness*0.25+
                    low_noise*(0.20 if kind==2 else 0.08))
            if kind==1: wave0+=0.1*math.sin(TWO_PI*(840*t+75*t*t))
            if kind==3: wave0+=0.08*high*(0.5+0.5*math.sin(TWO_PI*23*t))
        elif stage==1:  # cast: an acoustic attack, whoosh/chime/lightning burst
            f=base*(3.0+5.0*(1.0-p)**2)
            phase+=TWO_PI*f/RATE
            phase_hi+=TWO_PI*f*1.49/RATE
            wave0=0.20*math.sin(phase)+0.15*math.sin(phase_hi)+high*0.27
            if kind==0: wave0+=low_noise*1.8
            if kind==1: wave0+=0.20*math.sin(TWO_PI*(1400*t-560*t*t))
            if kind==2: wave0+=0.21*math.sin(TWO_PI*(430*t+190*t*t))
            if kind==3: wave0+=0.44*high*abs(math.sin(TWO_PI*37*t))
            if kind==4: wave0+=0.19*math.sin(TWO_PI*(270*t+250*t*t))
        else:  # impact: weight, brittle echoes and natural decays
            f=base*(0.86+0.06*math.exp(-t*6))
            phase+=TWO_PI*f/RATE
            wave0=0.31*math.sin(phase)*(math.exp(-t*4)+0.25)
            wave0+=0.30*low_noise*math.exp(-t*3.5)
            for transient in range(4):
                onset=0.055+0.085*transient
                delay=t-onset
                if delay>=0:
                    envelope=math.exp(-delay*(15+transient*2))
                    tone=base*(5.2+transient*0.7+kind*0.15)
                    wave0+=0.12*envelope*math.sin(TWO_PI*tone*delay)
                    if kind in (1,3):
                        wave0+=0.11*envelope*high
            if kind==0: wave0+=0.18*low_noise*math.exp(-t*6)
            if kind==2: wave0+=0.15*math.sin(TWO_PI*(174*t+12*t*t))*math.exp(-t*5)
            if kind==4: wave0+=0.07*math.sin(TWO_PI*(73*t+11*t*t))
        values.append(wave0*env)
    peak=max(max(abs(x) for x in values),0.001)
    factor=0.73/peak
    pcm=[max(-32767,min(32767,int(v*factor*32767))) for v in values]
    return struct.pack("<"+str(len(pcm))+"h",*pcm)

def generate():
    ffmpeg=shutil.which("ffmpeg")
    if ffmpeg is None:
        import imageio_ffmpeg
        ffmpeg=imageio_ffmpeg.get_ffmpeg_exe()
    sound_dir=OUT/"sounds"
    sound_dir.mkdir(parents=True,exist_ok=True)
    manifest={}
    with tempfile.TemporaryDirectory() as temp:
        for ci,kind in enumerate(KINDS):
            for si,stage in enumerate(STAGES):
                name=f"{kind}_{stage}"
                wav=Path(temp)/(name+".wav")
                with wave.open(str(wav),"wb") as w:
                    w.setnchannels(1);w.setsampwidth(2);w.setframerate(RATE)
                    w.writeframes(synthesize(ci,si))
                dest=sound_dir/(name+".ogg")
                subprocess.run([
                    ffmpeg,"-hide_banner","-loglevel","error","-y","-i",str(wav),
                    "-c:a","libvorbis","-qscale:a","4",str(dest)
                ],check=True)
                if dest.stat().st_size<800:
                    raise RuntimeError(f"invalid audio output {dest}")
                manifest[name]={"sounds":[{
                    "name":f"aetheria_preview:{name}",
                    "volume":0.8
                }],"subtitle":f"subtitles.aetheria_preview.{name}"}
    (OUT/"sounds.json").write_text(json.dumps(manifest,indent=2)+"\n",encoding="utf-8")
    print(f"Generated {len(manifest)} original 3-stage audio stems: {sound_dir}")

if __name__=="__main__":generate()
