use raylib::prelude::*;
use raylib::core::audio::{ Sound, RaylibAudio };



fn main() {
    let (mut rl, thread) = raylib::init()
    .size(640, 480)
    .title("Hello, World")
    .vsync()
    .build();

    let mut audio = RaylibAudio::init_audio_device();
    let bamboo = Sound::load_sound("bamboo.wav").unwrap();
    let hihat = Sound::load_sound("hihat.wav").unwrap();

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            audio.play_sound_multi(&bamboo);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            audio.play_sound(&hihat);
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_fps(12,12);
    }
}
