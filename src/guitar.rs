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

pub fn derive_string(note: Note) -> i32 {
    let note = format!("{:?}", note);
    /* XXX: This is dumb */
    (String::from("EADGBE").find(note.as_str()).unwrap() as i32) + 1
}
