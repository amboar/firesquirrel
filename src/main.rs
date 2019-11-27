/* SPDX-License-Identifier: GPL-3.0-only */
/* Copyright 2019 Andrew Jeffery */
mod theory;
mod guitar;
mod challenge;

use crate::challenge::{Challenge, ChallengeError, IORenderer, issue};
use crate::guitar::{Guitar, Tuning};

use std::io;
use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn ask_stdio(challenge: Challenge) -> Result<(), ChallengeError> {
    issue(challenge, &mut IORenderer::new(&mut io::stdin().lock(), &mut io::stdout().lock()))
}

fn ask_frets(guitar: Guitar) -> Result<(), ChallengeError> {
    ask_stdio(Challenge::fret(guitar)?)
}

fn ask_notes(guitar: Guitar) -> Result<(), ChallengeError> {
    ask_stdio(Challenge::note(guitar)?)
}

fn ask_strings(guitar: Guitar) -> Result<(), ChallengeError> {
    ask_stdio(Challenge::string(guitar)?)
}

fn ask_tunings(guitar: Guitar) -> Result<(), ChallengeError> {
    ask_stdio(Challenge::tuning(guitar)?)
}

fn ask_modes(_: Guitar) -> Result<(), ChallengeError> {
    ask_stdio(Challenge::mode()?)
}

fn ask_whatever(guitar: Guitar) -> Result<(), ChallengeError> {
    let quizzes: Vec<& dyn Fn(Guitar) -> Result<(), ChallengeError>> =
        vec![&ask_frets, &ask_notes, &ask_strings, &ask_tunings, &ask_modes];
    let mut rng = thread_rng();
    quizzes.choose(&mut rng).unwrap()(guitar)
}

fn ask_forever(f: &(dyn Fn(Guitar) -> Result<(), ChallengeError>)) -> Result<(), ChallengeError> {
    loop {
        f(Guitar::new(Tuning::EADGBE))?;
        println!();
    }
}

fn main() -> Result<(), ChallengeError> {
    let args: Vec<String> = env::args().collect();

    let mode: &(dyn Fn(Guitar) -> Result<(), ChallengeError>) = match args.len() {
        1 => &ask_whatever,
        _ => {
            let mode = &args[1];

            match mode.as_ref() {
                "frets" => &ask_frets,
                "notes" => &ask_notes,
                "strings" => &ask_strings,
                "tunings" => &ask_tunings,
                "modes" => &ask_modes,
                mode => {
                    println!("Unrecognised mode: {}", mode);
                    panic!();
                },
            }
        },
    };

    ask_forever(mode)
}
