// use std::io::BufReader;
use std::io::stdin;
use std::process::Stdio;
use std::process::Command;
use whisper_rs::{WhisperContext, FullParams, SamplingStrategy};

// const MODEL: &[u8] = include_bytes!("../vendor/models/ggml-base.en.bin");
const MODEL_FP: &str = "./vendor/models/ggml-base.en.bin";

pub fn transcribe_audio (audio_data: &Vec<i16>) -> String {
    // let mut ctx = WhisperContext::new_from_buffer(MODEL).expect("Failed to load model.");
    let mut ctx = WhisperContext::new(MODEL_FP).expect("Failed to load model.");
    let audio = whisper_rs::convert_integer_to_float_audio(audio_data);

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1  });
    params.set_language(Some("en"));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_print_timestamps(false);

    ctx.full(params, &audio[..]).expect("Failed to run model");

    let num_segments = ctx.full_n_segments();
    let mut transcription = String::new();
    for i in 0..num_segments {
        let segment = ctx.full_get_segment_text(i).expect("Failed to get segment.");
        transcription.push_str(&segment);
    }

    String::from(transcription.trim())
}

// TODO convert to Result
pub fn listen () -> String {
    let process = Command::new("sox")
        .args(["-t", "coreaudio", "default", "-r", "16000", "-c", "1", "-b", "16", "output.wav"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    println!("Start talking now and press 'enter' to finish:");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read user line");

    let pid = process.id().to_string();
    Command::new("kill")
        .args(["-s", "TERM", &pid])
        .output()
        .expect("Failed to terminate process.");

    process.wait_with_output().unwrap();
    let audio_data = get_audio_from_file("output.wav");

    // let mut buffer = Vec::new();
    // process.stdout.unwrap().read_to_end(&mut buffer).unwrap();
    // let reader = BufReader::new(&buffer[..]);

    let transcription = transcribe_audio(&audio_data);
    println!("{}", &transcription);
    transcription
}

/** Get 16bit WAV audio from file */
fn get_audio_from_file (fp: &str) -> Vec<i16> {
    let mut wav_reader = hound::WavReader::open(fp).unwrap();
    let audio_data = wav_reader
        .samples()
        .map(|s| s.expect("invalid sample"))
        .collect::<Vec<_>>();
    audio_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transcribe_simple_audio () {
        let audio_data = get_audio_from_file("./audio/rust-test.wav");
        let transcription = transcribe_audio(&audio_data);
        assert_eq!(transcription, "This is a Rust test.")
    }
}
