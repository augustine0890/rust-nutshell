use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    let audio_names = vec!["explode", "lose", "move", "pew", "startup", "win"];
    for item in audio_names.iter() {
        audio.add(item, &format!("audio/{}.wav", item));
    }
    audio.play("startup");

    // Cleanup audio
    audio.wait();
    Ok(())
}
