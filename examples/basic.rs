use std::{sync::Arc, thread, time::Duration};

use lyre::{Context, MusicData, SfxData};
use oddio::Frames;

fn load_wav(bytes: &[u8]) -> Arc<Frames<[f32; 2]>> {
    let mut reader = hound::WavReader::new(bytes).expect("Failed to read WAV file");
    let hound::WavSpec {
        sample_rate: source_sample_rate,
        sample_format,
        bits_per_sample,
        channels,
        ..
    } = reader.spec();

    // convert the WAV data to floating point samples
    // e.g. i8 data is converted from [-128, 127] to [-1.0, 1.0]
    let samples_result: Result<Vec<f32>, _> = match sample_format {
        hound::SampleFormat::Int => {
            let max_value = 2_u32.pow(bits_per_sample as u32 - 1) - 1;
            reader
                .samples::<i32>()
                .map(|sample| sample.map(|sample| sample as f32 / max_value as f32))
                .collect()
        }
        hound::SampleFormat::Float => reader.samples::<f32>().collect(),
    };

    let mut samples = samples_result.unwrap();

    match channels {
        2 => {
            // channels are interleaved, so we put them together in stereo
            let samples_stereo: Vec<_> = oddio::frame_stereo(&mut samples).to_vec();
            oddio::Frames::from_iter(source_sample_rate, samples_stereo)
        }
        1 => {
            // `lyre` currently only supports stereo sounds.
            // To support mono sounds here, we duplicate the data.
            // If this is a common use case, `oddio::MonoToStereo` should be used instead.
            let samples_stereo = samples
                .iter()
                .zip(samples.iter())
                .clone()
                .map(|(a, b)| [*a, *b]);
            oddio::Frames::from_iter(source_sample_rate, samples_stereo)
        }
        _ => {
            panic!("Unsupported number of channels: {}", channels);
        }
    }
}

fn main() {
    let mut context = Context::default();

    let music_frames =
        load_wav(include_bytes!("../examples/resources/rain_and_thunder.wav").as_ref());
    let music_data = MusicData::from(music_frames);
    let mut music = context.play_music(&music_data);

    let sfx_frames = load_wav(include_bytes!("../examples/resources/pickupCoin.wav").as_ref());
    let sfx_data = SfxData::from(sfx_frames);

    thread::sleep(Duration::from_secs_f32(1.0));
    {
        let gain = music.gain();
        music.set_gain(gain + 5.0);
    }

    thread::sleep(Duration::from_secs_f32(1.5));
    println!("Pausing background music, playing sfx.");
    {
        // Pause background music; when resumed, it will continue playing from where we left off.
        music.pause();
        let _sfx = context.play_sfx(&sfx_data);
    }

    thread::sleep(Duration::from_secs_f32(0.5));
    println!("Resuming background music.");
    {
        let gain = music.gain();
        music.set_gain(gain - 10.0);
        music.resume();
    }

    println!("Stopping background music.");
    thread::sleep(Duration::from_secs_f32(2.5));
    {
        // Stop music for good; we will start a new one to restart from the beginning.
        music.stop();
    }

    thread::sleep(Duration::from_secs_f32(1.0));
    println!("Restarting background music.");

    // Restarting gives us a new Music instance.
    // Since we already stopped the music, this is equivalent to calling `Context::play_music` again.
    let _new_music = context.restart(music, &music_data);

    // Let the music play for a little longer.
    thread::sleep(Duration::from_secs_f32(8.0));
}
