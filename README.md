# Testing manually

## sound init but no sound played
+ cargo run
+ press ESC

program exits without error

## play sound using `RaylibAudio.play_sound`
+ cargo run
+ press keyboard down one or more times
+ press ESC

program exits without error

## play sound using `RaylibAudio.play_sound_multi`
+ cargo run
+ press keyboard up one or more times
+ press ESC

program exits with STATUS_HEAP_CORRUPTION or STATUS_ACCESS_VIOLATION

# Logs

```
INFO: Initializing raylib 3.0
INFO: DISPLAY: Device initialized successfully
INFO:     > Display size: 1920 x 1200
INFO:     > Render size:  640 x 480
INFO:     > Screen size:  640 x 480
INFO:     > Viewport offsets: 0, 0
INFO: GLAD: OpenGL extensions loaded successfully
INFO: GL: OpenGL 3.3 Core profile supported
INFO: DISPLAY: Trying to enable VSYNC
INFO: GL: OpenGL device information:
INFO:     > Vendor:   Intel
INFO:     > Renderer: Intel(R) UHD Graphics 630
INFO:     > Version:  3.3.0 - Build 27.20.100.7990
INFO:     > GLSL:     3.30 - Build 27.20.100.7990
INFO: GL: Supported extensions count: 230
INFO: GL: DXT compressed textures supported
INFO: GL: ETC2/EAC compressed textures supported
INFO: GL: ASTC compressed textures supported
INFO: GL: Anisotropic textures filtering supported (max: 16X)
INFO: TEXTURE: [ID 1] Texture created successfully (1x1 - 1 mipmaps)
INFO: TEXTURE: [ID 1] Default texture loaded successfully
INFO: SHADER: [ID 1] Compiled successfully
INFO: SHADER: [ID 2] Compiled successfully
INFO: SHADER: [ID 3] Program loaded successfully
INFO: SHADER: [ID 3] Default shader loaded successfully
INFO: RLGL: Internal vertex buffers initialized successfully in RAM (CPU)
INFO: RLGL: Internal vertex buffers uploaded successfully to VRAM (GPU)
INFO: RLGL: Default state initialized successfully
INFO: TEXTURE: [ID 2] Texture created successfully (128x128 - 1 mipmaps)
INFO: FONT: Default font loaded successfully
INFO: AUDIO: Device initialized successfully
INFO:     > Backend:      miniaudio / WASAPI
INFO:     > Format:       32-bit IEEE Floating Point -> 32-bit IEEE Floating Point
INFO:     > Channels:     2 -> 2
INFO:     > Sample rate:  44100 -> 44100
INFO:     > Periods size: 1320
INFO: AUDIO: Multichannel pool size: 16
INFO: WAVE: [bamboo.wav] File loaded successfully (44100 Hz, 16 bit, Stereo)
INFO: WAVE: Unloaded wave data from RAM
INFO: WAVE: [hihat.wav] File loaded successfully (44100 Hz, 16 bit, Stereo)
INFO: WAVE: Unloaded wave data from RAM
INFO: WAVE: Unloaded sound data from RAM
INFO: WAVE: Unloaded sound data from RAM
error: process didn't exit successfully: `target\debug\raylib-example.exe` (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
```

# Machine info

+ Windows 10 64 bit
+ Asus gaming laptop TUF GAMING FX504GE_FX80GE
+ Realtek High Definition Audio RTKVHD64.sys


