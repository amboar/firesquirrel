/* SPDX-License-Identifier: GPL-3.0-only */
/* Copyright 2019 Andrew Jeffery */

use std::vec::Vec;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Note { C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B }

#[derive(Debug)]
pub enum NoteError {
    OffsetError,
    NotationError,
}

impl Note {
    pub fn derive(&self, offset: i32) -> Result<Note, NoteError> {
        match ((*self as i32) + offset) % 12 {
            0 => Ok(Note::C),
            1 => Ok(Note::Db),
            2 => Ok(Note::D),
            3 => Ok(Note::Eb),
            4 => Ok(Note::E),
            5 => Ok(Note::F),
            6 => Ok(Note::Gb),
            7 => Ok(Note::G),
            8 => Ok(Note::Ab),
            9 => Ok(Note::A),
            10 => Ok(Note::Bb),
            11 => Ok(Note::B),
            _ => Err(NoteError::OffsetError),
        }
    }
}

impl FromStr for Note {
    type Err = NoteError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "c"         => Ok(Note::C),
            "c#" | "db" => Ok(Note::Db),
            "d"         => Ok(Note::D),
            "d#" | "eb" => Ok(Note::Eb),
            "e"         => Ok(Note::E),
            "f"         => Ok(Note::F),
            "f#" | "gb" => Ok(Note::Gb),
            "g"         => Ok(Note::G),
            "g#" | "ab" => Ok(Note::Ab),
            "a"         => Ok(Note::A),
            "a#" | "bb" => Ok(Note::Bb),
            "b"         => Ok(Note::B),
            _ => Err(NoteError::NotationError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interval {
    Identity,
    Half,
    Whole,
}

#[derive(Debug, Clone, Copy)]
pub enum IntervalError {
    UnrecognisedInterval,
}

impl FromStr for Interval {
    type Err = IntervalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "half" | "semitone" => Ok(Interval::Half),
            "whole" | "tone" => Ok(Interval::Whole),
            _ => Err(IntervalError::UnrecognisedInterval),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Degree {
    Tonic,
    Supertonic,
    Mediant,
    Subdominant,
    Dominant,
    Submediant,
    Subtonic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian
}

#[derive(Debug, Clone, Copy)]
pub enum ModeError {
    UnrecognisedMode,
}

impl FromStr for Mode {
    type Err = ModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "ionian" => Ok(Mode::Ionian),
            "dorian" => Ok(Mode::Dorian),
            "phrygian" => Ok(Mode::Phrygian),
            "lydian" => Ok(Mode::Lydian),
            "mixolydian" => Ok(Mode::Mixolydian),
            "aeolian" => Ok(Mode::Aeolian),
            "locrian" => Ok(Mode::Locrian),
            _ => Err(ModeError::UnrecognisedMode),
        }
    }
}

pub const DIATONIC: [Interval; 6] = [
    Interval::Whole,
    Interval::Whole,
    Interval::Half,
    Interval::Whole,
    Interval::Whole,
    Interval::Whole,
];

pub enum Class {
    Heptatonic(&'static [Interval; 6], Mode),
}

pub struct Scale {
    notes: Vec<Note>,
}

impl Scale {
    pub fn new(class: Class, key: Note) -> Result<Scale, NoteError> {
        let (intervals, mode) = match class {
            Class::Heptatonic(intervals, mode) => {
                (intervals, mode)
            }
        };

        let mut note = key;
        let mut scale = Scale {
            notes: vec!(),
        };
        scale.notes.push(key);
        for i in intervals {
            note = note.derive(*i as i32)?;
            scale.notes.push(note);
        }

        scale.notes.rotate_left(mode as usize);

        Ok(scale)
    }

    pub fn note(&self, degree: Degree) -> Note {
        *self.notes.get(degree as usize).unwrap()
    }
}
