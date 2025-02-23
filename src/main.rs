#![windows_subsystem = "windows"]

mod gui;
mod chord;

use iced::{Theme, Element, Subscription};
use rodio::{self, Source};
use std::time::Duration;
use threadpool::ThreadPool;
use num_cpus;

trait Playable {
    fn play(&self, bpm: f32);
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp
}

#[derive(Debug, Clone)]
enum NoteLength { 
    Whole, Half, Quarter, Eighth, Sixteenth
}

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

#[derive(Debug, Clone)]
struct RealNote { 
    note: Note, 
    length: NoteLength, 
    octave: f32,
}

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

impl Playable for RealNote { 
    fn play(&self, bpm: f32) {
        self.play_sound(bpm);
    }
}

struct Chord { 
    notes: Vec<RealNote>
}
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

impl Playable for Chord { 
    fn play(&self, bpm: f32) {
        async_play_note(&self.notes, bpm);
    }
}

fn async_play_note(notes: &Vec<RealNote>, bpm: f32) {
    let length = notes.len().min(num_cpus::get());
    let pool = ThreadPool::new(length);
    for note in notes.clone() { 
        pool.execute(move || {
            note.play_sound(bpm);
        });
    }
}

#[derive(Debug, Clone)]
enum Message { 
    OctaveChange(f32),
    BpmChange(f32),
    CustomBpmChange(String),
    Play(Note),
    PlayChords,
    PlayAsync
}

struct Program { 
    octave: f32,
    bpm: f32,
    custom_bpm: String,
    play_chords: bool,
    play_async: bool
} 

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

pub fn main() -> iced::Result {
    iced::application("namne", Program::update, Program::view) 
        .subscription(Program::subscription)
        .theme(|_| Theme::TokyoNight)
        .run()
} 
