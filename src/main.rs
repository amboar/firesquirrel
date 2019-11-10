/* SPDX-License-Identifier: GPL-3.0-only */
/* Copyright 2019 Andrew Jeffery */
mod theory;
mod guitar;
mod challenge;

use crate::challenge::{Challenge, ChallengeError, IORenderer, issue};

use std::io;
use std::env;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn ask_stdio(challenge: Challenge) -> Result<(), ChallengeError> {
    issue(challenge, &mut IORenderer::new(&mut io::stdin().lock(), &mut io::stdout().lock()))
}

fn ask_frets() -> Result<(), ChallengeError> {
    ask_stdio(Challenge::fret()?)
}

fn ask_notes() -> Result<(), ChallengeError> {
    ask_stdio(Challenge::note()?)
}

fn ask_strings() -> Result<(), ChallengeError> {
    ask_stdio(Challenge::string()?)
}

fn ask_tunings() -> Result<(), ChallengeError> {
    ask_stdio(Challenge::tuning()?)
}

fn ask_whatever() -> Result<(), ChallengeError> {
    let quizzes: Vec<& dyn Fn() -> Result<(), ChallengeError>> =
        vec![&ask_frets, &ask_notes, &ask_strings, &ask_tunings];
    let mut rng = thread_rng();
    quizzes.choose(&mut rng).unwrap()()
}

fn ask_forever(f: &(dyn Fn() -> Result<(), ChallengeError>)) -> Result<(), ChallengeError> {
    loop {
        f()?;
        println!();
    }
}

fn main() -> Result<(), ChallengeError> {
    let args: Vec<String> = env::args().collect();

    let mode: &(dyn Fn() -> Result<(), ChallengeError>) = match args.len() {
        1 => &ask_whatever,
        _ => {
            let mode = &args[1];

            match mode.as_ref() {
                "frets" => &ask_frets,
                "notes" => &ask_notes,
                "strings" => &ask_strings,
                "tunings" => &ask_tunings,
                mode => {
                    println!("Unrecognised mode: {}", mode);
                    panic!();
                },
            }
        },
    };

    ask_forever(mode)
}
