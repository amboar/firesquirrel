/* SPDX-License-Identifier: GPL-3.0-only */
/* Copyright 2019 Andrew Jeffery */
/*
pub enum Interval {
    Identity,
    Half,
    Whole,
}

pub enum Degree {
    Tonic,
    Supertonic,
    Mediant,
    Subdominant,
    Dominant,
    Submediant,
    Subtonic,
}

pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian
}

pub enum Scale { Major, Minor, }
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Note {
    C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B
}

#[derive(Debug)]
pub enum NoteError {
    OffsetError,
    NotationError,
}

pub fn derive_note(base: Note, offset: i32) -> Result<Note, NoteError> {
    match ((base as i32) + offset) % 12 {
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

pub fn normalise_note(note: String) -> Result<Note, NoteError> {
    match note.to_lowercase().as_ref() {
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
