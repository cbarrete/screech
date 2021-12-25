use screech::distort::*;
use screech::gain::*;
use screech::io::*;
use screech::phase::*;
use screech::pitch::*;
use screech::pseudo_cycle::*;
use screech::types::AudioBuffer;
use std::env::args;
use std::fs::File;
use std::process::exit;

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

fn run<F: Fn(AudioBuffer) -> AudioBuffer>(
    f: F,
    mut audio_buffer: AudioBuffer,
    iterations: u32,
) -> AudioBuffer {
    for _ in 0..iterations {
        audio_buffer = f(audio_buffer)
    }
    audio_buffer
}

fn do_main(
    in_filename: &str,
    out_filename: &String,
    mut option_arguments: &[String],
) -> Result<(), CliError> {
    let mut audio_buffer = read_wav(&mut File::open(in_filename)?)?;

    while !option_arguments.is_empty() {
        let iterations = match option_arguments[0].parse::<u32>() {
            Ok(i) => {
                option_arguments = &option_arguments[1..];
                i
            }
            Err(_) => 1,
        };
        if "interpolate".starts_with(&option_arguments[0]) {
            audio_buffer = run(|ab: AudioBuffer| interpolate(&ab), audio_buffer, iterations);
            option_arguments = &option_arguments[1..];
        } else if "fractalize".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "fractalize takes an integral depth",
                )));
            }
            let depth = option_arguments[1].parse::<u32>()?;
            audio_buffer = run(
                |ab: AudioBuffer| fractalize(&ab, depth),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[2..];
        } else if "expand".starts_with(&option_arguments[0]) {
            audio_buffer = expand(audio_buffer);
            option_arguments = &option_arguments[1..];
        } else if "reversepseudocycles".starts_with(&option_arguments[0]) {
            audio_buffer = run(
                |ab: AudioBuffer| reverse_pseudo_cycles(ab),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[1..];
        } else if "fold".starts_with(&option_arguments[0]) {
            audio_buffer = run(|ab: AudioBuffer| fold(ab), audio_buffer, iterations);
            option_arguments = &option_arguments[1..];
        } else if "hardclip".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "hardclip takes a decimal threshold",
                )));
            }
            audio_buffer = hard_clip(audio_buffer, option_arguments[1].parse::<f32>()?);
            option_arguments = &option_arguments[2..];
        } else if "softclip".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "softclip takes a decimal amount",
                )));
            }
            let amount = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(
                |ab: AudioBuffer| soft_clip(ab, amount),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[2..];
        } else if "tense".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "tense takes a decimal tension",
                )));
            }
            let tension = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(
                |ab: AudioBuffer| tense(ab, tension),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[2..];
        } else if "tensepseudocycles".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "tensepseudocycles takes a decimal tension",
                )));
            }
            let tension = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(
                |ab: AudioBuffer| tense_pseudo_cycles(ab, tension),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[2..];
        } else if "decimate".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "decimate takes a decimal depth",
                )));
            }
            audio_buffer = decimate(audio_buffer, option_arguments[1].parse::<f32>()?);
            option_arguments = &option_arguments[2..];
        } else if "delaypitch".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 3 {
                return Err(CliError::Arguments(String::from(
                    "delaypitch takes a decimal factor and an integer size",
                )));
            }
            let factor = option_arguments[1].parse::<f32>()?;
            let size = option_arguments[2].parse::<u8>()?;
            audio_buffer = run(
                |ab: AudioBuffer| delay_pitch(ab, factor, size),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[3..];
        } else if "delayrotate".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 4 {
                return Err(CliError::Arguments(String::from(
                    "delayrotate takes a delay, feedback and frequency",
                )));
            }
            let delay = option_arguments[1].parse::<usize>()?;
            let feedback = option_arguments[2].parse::<f32>()?;
            let frequency = option_arguments[3].parse::<f32>()?;
            audio_buffer = run(
                |ab: AudioBuffer| delay_rotate(ab, delay, feedback, frequency),
                audio_buffer,
                iterations,
            );
            option_arguments = &option_arguments[4..];
        } else if "speed".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "speed takes a decimal speed",
                )));
            }
            let s = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| speed(ab, s), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "gain".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from(
                    "gain takes a decimal gain",
                )));
            }
            let g = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| gain(ab, g), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "dc".starts_with(&option_arguments[0]) {
            if option_arguments.len() < 2 {
                return Err(CliError::Arguments(String::from("dc takes a decimal dc")));
            }
            let dc = option_arguments[1].parse::<f32>()?;
            audio_buffer = run(|ab: AudioBuffer| add_dc(ab, dc), audio_buffer, iterations);
            option_arguments = &option_arguments[2..];
        } else if "removedc".starts_with(&option_arguments[0]) {
            audio_buffer = remove_dc(audio_buffer);
            option_arguments = &option_arguments[1..];
        } else if "normalize".starts_with(&option_arguments[0]) {
            audio_buffer = normalize(audio_buffer);
            option_arguments = &option_arguments[1..];
        } else {
            return Err(CliError::Arguments(format!(
                "Unknown option {}\n{}",
                option_arguments[0], USAGE,
            )));
        }
    }
    write_wav(&mut File::create(out_filename)?, &audio_buffer).or_else(|e| Err(e.into()))
}

static USAGE: &str = "usage: screech input_file [[iterations] option]... output_file
available options:
  interpolate
  fractalize <depth>
  expand
  reversepseudocycles
  fold
  hardclip <threshold>
  softclip <amount>
  tense <tension>
  tensepseudocycles <tension>
  decimate <depth>
  delaypitch <factor> <size>
  delayrotate <delay> <feedback> <frequency>
  speed <speed>
  gain <gain>
  dc <dc>
  removedc
  normalize

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
