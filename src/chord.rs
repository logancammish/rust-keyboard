// use other files inside this project
use crate::{Chord, Note};

// use dependencies
use std::collections::HashMap;

// impliment for Chord
// functions: 
// 1. get_major_scale  -> returns the major scale of the relevant Note
impl Chord { 
    pub fn get_major_scale(note: Note) -> Vec<Note> { 
        let mut major_scales: HashMap<Note, Vec<Note>> = HashMap::new();
        major_scales.insert(Note::A, vec![
            Note::A, Note::B, Note::Csharp, Note::D, Note::E, Note::Fsharp, Note::Gsharp
        ]);
        major_scales.insert(Note::Asharp, vec![
            Note::Asharp, Note::C, Note::D, Note::Dsharp, Note::F, Note::G, Note::A
        ]);
        major_scales.insert(Note::B, vec![
            Note::B, Note::Csharp, Note::Dsharp, Note::E, Note::Fsharp, Note::Gsharp, Note::Asharp
        ]);
        major_scales.insert(Note::C, vec![
            Note::C, Note::D, Note::E, Note::F, Note::G, Note::A, Note::B
        ]);
        major_scales.insert(Note::Csharp, vec![
            Note::Csharp, Note::Dsharp, Note::F, Note::Fsharp, Note::Gsharp, Note::Asharp, Note::C
        ]);
        major_scales.insert(Note::D, vec![
            Note::D, Note::E, Note::Fsharp, Note::G, Note::A, Note::B, Note::Csharp
        ]);
        major_scales.insert(Note::Dsharp, vec![
            Note::Dsharp, Note::F, Note::G, Note::Gsharp, Note::Asharp, Note::C, Note::D
        ]);
        major_scales.insert(Note::E, vec![
            Note::E, Note::Fsharp, Note::Gsharp, Note::A, Note::B, Note::Csharp, Note::Dsharp
        ]);
        major_scales.insert(Note::F, vec![
            Note::F, Note::G, Note::A, Note::Asharp, Note::C, Note::D, Note::E
        ]);
        major_scales.insert(Note::Fsharp, vec![
            Note::Fsharp, Note::Gsharp, Note::Asharp, Note::B, Note::Csharp, Note::Dsharp, Note::F
        ]);
        major_scales.insert(Note::G, vec![
            Note::G, Note::A, Note::B, Note::C, Note::D, Note::E, Note::Fsharp
        ]);
        major_scales.insert(Note::Gsharp, vec![
            Note::Gsharp, Note::Asharp, Note::C, Note::Csharp, Note::Dsharp, Note::F, Note::G
        ]);

        major_scales.get(&note).expect("Not a valid scale").clone()
    }
}
