#![windows_subsystem = "windows"]

use iced::widget::{button, container, slider, text, text_input, checkbox};
use iced::{Theme, Element, widget, Length, Subscription};
use rodio::{self, Source};
use std::collections::HashMap;
use std::time::Duration;
use threadpool::ThreadPool;
use num_cpus;


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp
}

impl Note {
    pub fn get_major_scale(&self) -> Vec<Note> { 
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

        major_scales.get(&self).expect("Not a valid scale").clone()
    }
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

    fn play(&self, bpm: f32) {  
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

    fn play_triad(&self, bpm: f32) { 
        let scale = Note::get_major_scale(&self.note); 
        let chord: Vec<RealNote> = Vec::from([
            RealNote { note: scale[0].clone(), length: self.length.clone(), octave: self.octave },
            RealNote { note: scale[2].clone(), length: self.length.clone(), octave: self.octave },
            RealNote { note: scale[4].clone(), length: self.length.clone(), octave: self.octave },
        ]);
        async_play_note(&chord, bpm);
    }
}

fn async_play_note(notes: &Vec<RealNote>, bpm: f32) {
    let length = notes.len().min(num_cpus::get());
    let pool = ThreadPool::new(length);
    for note in notes.clone() { 
        pool.execute(move || {
            note.play(bpm);
        });
    }
}

#[derive(Debug, Clone)]
enum Message { 
    OctaveChange(f32),
    BpmChange(f32),
    CustomBpmChange(String),
    Play(Note),
    PlayChords
}

struct Program { 
    octave: f32,
    bpm: f32,
    custom_bpm: String,
    play_chords: bool
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

    fn view(&self) -> Element<Message> { 
        container(widget::column![
            widget::row!(
                text("Octave"),
                slider(
                    0.0..=10.0,
                    self.octave,
                    Message::OctaveChange
                ),
            ).spacing(2),

            widget::row!(
                text("BPM"),
                slider(
                    10.0..=300.0,
                    self.bpm, 
                    Message::BpmChange
                ),  
                
                text_input(format!("{}", &self.bpm).as_str(), &self.custom_bpm)
                .on_input(Message::CustomBpmChange) 
                .padding(2)
                .width(Length::Fixed(50.0)),
            ).spacing(2),

            widget::row!(
                checkbox("Play triads", self.play_chords)
                    .on_toggle(|_| Message::PlayChords),
            ),

            widget::row!(
                //C NOTE BEGIN
                //FLATS ARE INDICATED BY 50
                button("")
                .on_press(Message::Play(Note::C))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10),  
                button("")
                .on_press(Message::Play(Note::Csharp))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),      
                button("")
                .on_press(Message::Play(Note::D))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10), 
                button("")
                .on_press(Message::Play(Note::Dsharp))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),      
                button("")
                .on_press(Message::Play(Note::E))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10),         
                button("")
                .on_press(Message::Play(Note::F))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10),      
                button("")
                .on_press(Message::Play(Note::Fsharp))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),  
                button("")
                .on_press(Message::Play(Note::G))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10),      
                button("")
                .on_press(Message::Play(Note::Gsharp))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),      
                button("")
                .on_press(Message::Play(Note::A))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10),     
                button("")
                .on_press(Message::Play(Note::Asharp))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),      
                button("")
                .on_press(Message::Play(Note::B))
                .width(Length::Fixed(50.0)) 
                .height(Length::Fixed(150.0))  
                .padding(10),        
            ).spacing(3),

            widget::row!(
                text(format!("Octave: {}", &self.octave))
                .size(72)
                .wrapping(text::Wrapping::Glyph),
            )

        ].spacing(10))
        .padding(10)
        .into()
    }
    
    fn update(&mut self, message: Message) { 

        match message { 
            Message::PlayChords => {
                self.play_chords = !self.play_chords;
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
                if self.play_chords == false {
                    RealNote::play(&RealNote{
                        note: note, 
                        length: NoteLength::Whole,
                        octave: self.octave
                    }, self.bpm);
                } else { 
                    RealNote::play_triad(&RealNote{
                        note: note, 
                        length: NoteLength::Whole,
                        octave: self.octave
                    }, self.bpm);
                }


                // let c_chord: Vec<RealNote> = Vec::from([
                //     RealNote { note: Note::C, length: NoteLength::Whole, octave: self.octave },
                //     RealNote { note: Note::E, length: NoteLength::Half, octave: self.octave },
                //     RealNote { note: Note::G, length: NoteLength::Quarter, octave: self.octave },
                // ]);
                // async_play_note(&c_chord, self.bpm);
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
            play_chords: false
        }
    }
}

pub fn main() -> iced::Result {
    // println!("All music is at 120bpm, 4/4 time");
    
    iced::application("namne", Program::update, Program::view) 
        .subscription(Program::subscription)
        .theme(|_| Theme::TokyoNight)
        .run()
}
