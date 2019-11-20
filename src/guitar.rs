/* SPDX-License-Identifier: GPL-3.0-only */
/* Copyright 2019 Andrew Jeffery */
use crate::theory::Note;

/*
pub enum Position {
    Fret(i32),
    Note(Note),
    String(i32),
}
*/

#[derive(Debug)]
pub enum Tuning {
    EADGBE,
    DADGBE,
    CGCFAD,
}

pub struct Guitar {
    pub tuning: Tuning
}

impl Guitar {
    pub fn new(tuning: Tuning) -> Guitar {
        Guitar {
            tuning: tuning,
        }
    }

    pub fn derive_fret(string: Note, want: Note) -> i32 {
        let sn = string as i32;
        let wn = want as i32;

        if wn < sn {
            12 - ((sn - wn) % 12)
        } else {
            (wn - sn) % 12
        }
    }

    pub fn normalise_fret(fret: i32) -> i32 {
        fret % 12
    }

    pub fn derive_string(&self, note: Note) -> i32 {
        let note = format!("{:?}", note);
        /* XXX: This is dumb */
        (format!("{:?}", self.tuning).find(note.as_str()).unwrap() as i32) + 1
    }

    pub fn strings(&self) -> Vec<Note> {
        match self.tuning {
            Tuning::EADGBE => vec![ Note::E, Note::A, Note::D, Note::G, Note::B, Note::E, ],
            Tuning::DADGBE => vec![ Note::D, Note::A, Note::D, Note::G, Note::B, Note::E, ],
            Tuning::CGCFAD => vec![ Note::C, Note::G, Note::C, Note::F, Note::A, Note::D, ],
        }
    }
}
