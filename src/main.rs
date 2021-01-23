use std::env::args;
use std::fs::File;
use std::process::exit;
use screech::{AudioBuffer, Distort, Gain, Pitch, read_wav, write_wav};

#[derive(Debug)]
enum CliError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    Arguments(String),
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<std::num::ParseIntError> for CliError {
    fn from(err: std::num::ParseIntError) -> Self {
        CliError::ParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for CliError {
    fn from(err: std::num::ParseFloatError) -> Self {
        CliError::ParseFloat(err)
    }
}

fn run<F: Fn(AudioBuffer) -> AudioBuffer>(f: F, mut audio_buffer: AudioBuffer, iterations: u32) -> AudioBuffer {
    for _ in 0..iterations {
        audio_buffer = f(audio_buffer)
    }
    audio_buffer
}

fn do_main(in_filename: &str, out_filename: &String, mut option_arguments: &[String]) -> Result<(), CliError> {
    let mut audio_buffer = read_wav(&mut File::open(in_filename)?)?;
    while !option_arguments.is_empty() {
        let iterations = match option_arguments[0].parse::<u32>() {
            Ok(i) => { 
                option_arguments = &option_arguments[1..];
                i
            }
            Err(_) => { 1 }
        };
        if "-interpolate".starts_with(&option_arguments[0]) {
            audio_buffer = run(|ab: AudioBuffer| ab.interpolate(), audio_buffer, iterations);
            option_arguments = &option_arguments[1..];
        } else if "-fractalize".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("fractalize takes an integral depth parameter")));
            }
            let depth = option_arguments[1].parse::<u32>()?;
            audio_buffer = run(|ab: AudioBuffer| ab.fractalize(depth), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "-fold".starts_with(&option_arguments[0]) {
            audio_buffer = run(|ab: AudioBuffer| ab.fold(), audio_buffer, iterations);
            option_arguments = &option_arguments[1..];
        } else if "-hardclip".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("hardclip takes a decimal threshold parameter")));
            }
            audio_buffer = audio_buffer.hard_clip(option_arguments[1].parse::<f32>()?);
            option_arguments = &option_arguments[2..];
        } else if "-softclip".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("softclip takes a decimal amount parameter")));
            }
            let amount = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| ab.soft_clip(amount), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "-decimate".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("decimate takes a decimal depth parameter")));
            }
            audio_buffer = audio_buffer.decimate(option_arguments[1].parse::<f32>()?);
            option_arguments = &option_arguments[2..];
        } else if "-delaypitch".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("delaypitch takes a decimal factor parameter")));
            }
            let factor = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| ab.delay_pitch(factor), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "-speed".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("speed takes a decimal speed parameter")));
            }
            let speed = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| ab.speed(speed), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "-expand".starts_with(&option_arguments[0]) {
            audio_buffer = audio_buffer.expand();
            option_arguments = &option_arguments[1..];
        } else if "-gain".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("gain takes a decimal gain parameter")));
            }
            let gain = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| ab.gain(gain), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "-normalize".starts_with(&option_arguments[0]) {
            audio_buffer = audio_buffer.normalize();
            option_arguments = &option_arguments[1..];
        } else {
            return Err(CliError::Arguments(format!("Unknown option {}", option_arguments[0])));
        }
    }
    write_wav(&mut File::create(out_filename)?, &audio_buffer).or_else(|e| Err(e.into()))
}

static USAGE: &str = "usage: screech input_file [[iterations] option]... output_file
available options:
  -interpolate
  -fractalize <depth>
  -fold
  -hardclip <threshold>
  -softclip <amount>
  -decimate <depth>
  -delaypitch <factor>
  -speed <speed>
  -gain <gain>
  -normalize

short versions are tried in that order";

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        eprintln!("{}", USAGE);
        exit(1);
    }

    if let Err(err) = do_main(&args[1], &args[args.len() - 1], &args[2..args.len() - 1]) {
        match err {
            CliError::Io(e) => {
                eprintln!("{}", e.to_string());
                exit(2)
            }
            CliError::ParseInt(e) => {
                eprintln!("{}", e.to_string());
                exit(3)
            }
            CliError::ParseFloat(e) => {
                eprintln!("{}", e.to_string());
                exit(3)
            }
            CliError::Arguments(s) => {
                eprintln!("{}", s);
                exit(1);
            }
        }
    }
}
