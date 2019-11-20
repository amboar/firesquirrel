/* SPDX-License-Identifier: GPL-3.0-only */
/* Copyright 2019 Andrew Jeffery */
use crate::theory::{Note, NoteError, derive_note, normalise_note};
use crate::guitar::Guitar;

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

use std::io;
use std::num::ParseIntError;

fn choose_note() -> Note {
    let notes: Vec<Note> = vec![
            Note::C,
            Note::Db,
            Note::D,
            Note::Eb,
            Note::E,
            Note::F,
            Note::Gb,
            Note::G,
            Note::Ab,
            Note::A,
            Note::Bb,
            Note::B,
    ];

    *notes.choose(&mut thread_rng()).unwrap()
}

fn choose_string(guitar: &Guitar) -> Note {
    *guitar.strings().choose(&mut thread_rng()).unwrap()
}

fn choose_fret() -> i32 {
    thread_rng().gen_range(0, 12)
}

#[derive(Debug)]
pub enum RendererError {
    IoError(io::Error),
}

impl From<io::Error> for RendererError {
    fn from(error: io::Error) -> Self {
        RendererError::IoError(error)
    }
}

pub trait Renderer {
    fn challenge(&mut self, question: &String) -> Result<(), RendererError>;
    fn response(&mut self) -> Result<String, RendererError>;
    fn hint(&mut self, hint: String) -> Result<(), RendererError>;
    fn mark(&mut self, result: bool) -> Result<(), RendererError>;
}

pub struct IORenderer<'a> {
    din: &'a mut dyn io::BufRead,
    dout: &'a mut dyn io::Write,
}

impl<'a> IORenderer<'a> {
    pub fn new(din: &'a mut (dyn io::BufRead), dout: &'a mut (dyn io::Write)) -> IORenderer<'a> {
        IORenderer {
            din: din,
            dout: dout,
        }
    }
}

impl<'a> Renderer for IORenderer<'a> {
    fn challenge(&mut self, question: &String) -> Result<(), RendererError> {
        self.dout.write_all(question.as_bytes())?;
        self.dout.write(b"\n")?;
        Ok(())
    }

    fn response(&mut self) -> Result<String, RendererError> {
        self.dout.write_all(b"> ")?;
        self.dout.flush()?;

        let mut guess = String::new();
        self.din.read_line(&mut guess).expect("Failed to read line");
        Ok(guess.trim().to_lowercase())
    }

    fn hint(&mut self, hint: String) -> Result<(), RendererError> {
        self.dout.write_all(hint.as_bytes())?;
        Ok(())
    }

    fn mark(&mut self, result: bool) -> Result<(), RendererError> {
        self.dout.write_all(if result { b"Correct\n" } else { b"Incorrect\n" })?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChallengeType {
    Fret(i32),
    Note(Note),
    String(i32),
    Tuning(Note),
}

pub struct Challenge {
    question: String,
    answer: ChallengeType,
}

#[derive(Debug)]
pub enum ChallengeError {
    RendererError(RendererError),
    NoteError(NoteError),
    InvalidGuess(ParseIntError),
}

impl From<RendererError> for ChallengeError {
    fn from(error: RendererError) -> Self {
        ChallengeError::RendererError(error)
    }
}

impl From<NoteError> for ChallengeError {
    fn from(error: NoteError) -> Self {
        ChallengeError::NoteError(error)
    }
}

impl From<ParseIntError> for ChallengeError {
    fn from(error: ParseIntError) -> Self {
        ChallengeError::InvalidGuess(error)
    }
}

impl Challenge {
    pub fn fret(guitar: Guitar) -> Result<Challenge, ChallengeError> {
        let cs = choose_string(&guitar);
        let cn = choose_note();

        Ok(Challenge {
            question: format!("With {:?} tuning, which fret is {:?} on {:?}?", guitar.tuning, cn, cs),
            answer: ChallengeType::Fret(Guitar::derive_fret(cs, cn)),
        })
    }

    pub fn note(guitar: Guitar) -> Result<Challenge, ChallengeError> {
        let cs = choose_string(&guitar);
        let cf = choose_fret();

        Ok(Challenge {
            question: format!("With {:?} tuning, what note is fret {:?} on {:?}?", guitar.tuning, cf, cs),
            answer: ChallengeType::Note(derive_note(cs, cf)?),
        })
    }

    pub fn string(guitar: Guitar) -> Result<Challenge, ChallengeError> {
        let cs = choose_string(&guitar);

        Ok(Challenge {
            question: format!("With {:?} tuning, what string is {:?}?", guitar.tuning, cs),
            answer: ChallengeType::String(guitar.derive_string(cs)),
        })
    }

    pub fn tuning(guitar: Guitar) -> Result<Challenge, ChallengeError> {
        let cs = choose_string(&guitar);

        Ok(Challenge {
            question: format!("With {:?} tuning, what is the note of open string {}?", guitar.tuning,
                              guitar.derive_string(cs)),
            answer: ChallengeType::Tuning(cs),
        })
    }

    pub fn issue(&self, renderer: &mut dyn Renderer) -> Result<(), ChallengeError> {
        Ok(renderer.challenge(&self.question)?)
    }

    pub fn validate(&self, guess: String) -> Result<bool, ChallengeError> {
        match self.answer {
            ChallengeType::Fret(answer) => {
                let guess = guess.parse::<i32>()?;
                Ok(Guitar::normalise_fret(guess) == answer)
            },
            ChallengeType::Note(answer) => {
                let note: Note = normalise_note(guess)?;
                Ok(note == answer)
            }
            ChallengeType::String(answer) => {
                let guess = guess.parse::<i32>()?;
                Ok(guess == answer)
            }
            ChallengeType::Tuning(answer) => {
                let note: Note = normalise_note(guess)?;
                Ok(note == answer)
            }
        }
    }

    pub fn peek(&self) -> ChallengeType {
        self.answer
    }
}

pub fn issue(challenge: Challenge, renderer: &mut dyn Renderer) -> Result<(), ChallengeError> {
    challenge.issue(renderer)?;
    loop {
        let response = renderer.response()?;
        match response.as_ref() {
            "peek" => {
                renderer.hint(format!("{:?}", challenge.peek()))?;
            }
            val => {
                let result: bool = match challenge.validate(val.to_string()) {
                    Ok(correct) => correct,
                    Err(err) => match err {
                        ChallengeError::RendererError(_) => return Err(err),
                        ChallengeError::NoteError(_) | ChallengeError::InvalidGuess(_) => false,
                    }
                };
                renderer.mark(result)?;
                if result {
                    return Ok(());
                }
            }
        }
    }
}
