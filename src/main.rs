#![windows_subsystem = "windows"]
// use other files inside this project
mod gui;
mod chord;
mod midi;

// use dependencies     
use iced::{Theme, Element, Subscription};
use rodio::{self, Source};
use std::time::Duration;
use threadpool::ThreadPool;
use num_cpus;

// playable trait to implement polymorphism
// for structs RealNote and Chord
trait Playable {
    fn play(&self, bpm: f32);
}

// Note enum defines all notes in Western music
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp
}

// NoteLength enum defines the length of a note
// to be calculated according to beats per minute
#[derive(Debug, Clone)]
enum NoteLength { 
    Whole, Half, Quarter, Eighth, Sixteenth
}

// implement the NoteLength enum
// functions: 
//
// 1. duration_in_seconds -> calculates the time 
//                           in seconds that a note should last
// 2. check_bpm           -> checks if the bpm is valid
//                           (not below of equal to 0 or above 300)
impl NoteLength { 
    pub fn duration_in_seconds(&self, bpm: f32) -> f32 {
        match self {
            NoteLength::Whole => (60.0 / bpm) * 4.0,      
            NoteLength::Half => (60.0 / bpm) * 2.0,        
            NoteLength::Quarter => 60.0 / bpm,            
            NoteLength::Eighth => (60.0 / bpm) * 0.5,      
            NoteLength::Sixteenth => (60.0 / bpm) * 0.25,  
        }
    }
    pub fn check_bpm(bpm: f32) -> bool { 
        if (bpm <= 0.0) || (bpm > 300.0) {
            return false
        }

        true
    }
}

// RealNote struct, used for playing sounds according
// to their length and octave
// fields: 
// 1. note           -> Relevant Note enum 
// 2. length         -> Relevant NoteLength enum
// 3. octave         -> The octave that music should be played at
#[derive(Debug, Clone)]
struct RealNote { 
    note: Note, 
    length: NoteLength, 
    octave: f32,
}

// implement the RealNote struct
// functions:
// 1. base_frequencies     -> Determine the octave 0 frequency for the relevant Note
// 2. play_sound           -> Plays the note sound in the current thread
// 3. play_async           -> Plays the note sound in another thread asynchronously
// also implements:
// 1. play (from Playable) -> The same as play_sound
impl RealNote { 
    pub fn base_frequencies(note: Note) -> f32 { 
        match note {
            Note::C => 16.35,
            Note::Csharp => 17.32,
            Note::D => 18.35,
            Note::Dsharp => 19.45,
            Note::E => 20.60,
            Note::F => 21.83,
            Note::Fsharp => 23.12,
            Note::G => 24.50,
            Note::Gsharp => 25.96,
            Note::A => 27.50,   
            Note::Asharp => 29.14,
            Note::B => 30.87,
        }
    }

    fn play_sound(&self, bpm: f32) {  
        let time = NoteLength::duration_in_seconds(&self.length, bpm);
        let frequency: f32 = Self::base_frequencies(self.note.clone()) * 2_f32.powf(self.octave);
        // println!("Playing: {}hz | Time: {}s", frequency, time);
        let (_stream, device) = rodio::OutputStream::try_default()
            .expect("Failed to get output device");
        let source = rodio::source::SineWave::new(frequency)
            .amplify(0.1)
            .take_duration(Duration::from_secs_f32(time));
        let sink = rodio::Sink::try_new(&device)
            .expect("Failed to create sink with device");
        sink.append(source);
        sink.sleep_until_end();
    }

    fn play_async(&self, bpm: f32) { 
        let notes = vec![self.clone()];
        async_play_note(&notes, bpm);
    }
}

// implement Playable trait for RealNote 
impl Playable for RealNote { 
    fn play(&self, bpm: f32) {
        self.play_sound(bpm);
    }
}

// Chord struct, which is used to play multiple notes at once
// and calculations musically relevant to this concept
struct Chord { 
    notes: Vec<RealNote>
}

// implement the Chord struct
// functions:
// 1. triad_from_note   -> Calculates the major triad 
//                         (the 1st, 3rd and 5th notes of the major scale)
//                         and returns it as a function
// also implements:
// 1. play (from Playable) -> The same as plays the chord asynchronously using
//                            async_play_note
impl Chord {
    fn triad_from_note(note: &RealNote) -> Chord {
        let scale = Self::get_major_scale(note.note.clone());
        return Chord{
            notes: vec![
                RealNote { note: scale[0].clone(), length: note.length.clone(), octave: note.octave },
                RealNote { note: scale[2].clone(), length: note.length.clone(), octave: note.octave },
                RealNote { note: scale[4].clone(), length: note.length.clone(), octave: note.octave }
            ]
        }
    }
}

// implement Playable trait for Chord
impl Playable for Chord { 
    fn play(&self, bpm: f32) {
        async_play_note(&self.notes, bpm);
    }
}

// async_play_note function, which can be used at any point in the program 
// asynchronously plays notes   
fn async_play_note(notes: &Vec<RealNote>, bpm: f32) {
    let length = notes.len().min(num_cpus::get());
    let pool = ThreadPool::new(length);
    for note in notes.clone() { 
        pool.execute(move || {
            note.play_sound(bpm);
        });
    }
}

// Message enum, which is used to communicate changes to the GUI
#[derive(Debug, Clone)]
enum Message { 
    OctaveChange(f32),
    BpmChange(f32),
    CustomBpmChange(String),
    Play(Note),
    PlayChords,
    PlayAsync
}

// Program struct, which stores the current information the program may need
// fields:
// 1. octave        -> The current octave the program is using
// 2. bpm           -> The current beats per minute the program is using
// 3. custom_bpm    -> String representation of the bpm, required for iced
// 4. play_chords   -> Whether or not the play triad button is selected
// 5. play_async    -> Whether or not to play notes asynchronously 
struct Program { 
    octave: f32,
    bpm: f32,
    custom_bpm: String,
    play_chords: bool,
    play_async: bool
} 

// implement the Program struct
// functions: 
// 1. update_bpm    -> check and update the bpm
// 2. view          -> display gui
// 3. update        -> update Program
// 4. subscription  -> sets the iced subscription
impl Program { 
    pub fn update_bpm(&mut self, value: f32) {
        if NoteLength::check_bpm(value) {
            self.bpm = value;
            self.custom_bpm = value.to_string();
        } else {
            self.bpm = 60.0;
            self.custom_bpm = "60".to_string();
        }
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        Self::get_ui_information(self).into()
    }    
    
    fn update(&mut self, message: Message) { 
        match message { 
            Message::PlayChords => {
                self.play_chords = !self.play_chords;
            }

            Message::PlayAsync => {
                self.play_async = !self.play_async;
            }

            Message::OctaveChange(value) => {
                self.octave = value;
            }

            Message::CustomBpmChange(value) => {
                if let Ok(value) = value.parse::<f32>() {
                    Self::update_bpm(self, value);
                } 
            }

            Message::BpmChange(value) => {
                Self::update_bpm(self, value);
            }

            Message::Play(note) => {
                if (self.play_chords == false) && (self.play_async == false) {  
                    RealNote::play(&RealNote{
                        note: note, 
                        length: NoteLength::Whole,
                        octave: self.octave
                    }, self.bpm);
                } else if self.play_chords == true { 
                    Chord::play(&Chord::triad_from_note(&RealNote {
                        note: note, 
                        length: NoteLength::Whole,
                        octave: self.octave
                    }), self.bpm);
                } else if self.play_async == true {               
                    RealNote::play_async(&RealNote{ 
                        note: note, 
                        length: NoteLength::Whole,
                        octave: self.octave
                    }, self.bpm);
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

// changing Default for Program
impl Default for Program { 
    fn default() -> Self {
        Self {
            octave: 2.0,
            bpm: 120.0,
            custom_bpm: "120".to_string(),
            play_chords: false,
            play_async: false
        }
    }
}

// main function
pub fn main() -> iced::Result {
    iced::application("namne", Program::update, Program::view) 
        .subscription(Program::subscription)
        .theme(|_| Theme::TokyoNight)
        .run()
} 
