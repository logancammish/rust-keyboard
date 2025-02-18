#![windows_subsystem = "windows"]

use iced::{ widget::{button, container, slider, text, text_input}, * };
use rodio::{self, Source};
use std::{f32, ops::Mul, thread, time::Duration};


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp
}

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
}

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
    fn play(self, bpm: f32) {  

        thread::spawn(move || {   
            let time = NoteLength::duration_in_seconds(&self.length, bpm) as u64;
            let frequency: f32 = Self::base_frequencies(self.note).powf((&self.octave + 1.0));
            let (_stream, device) = rodio::OutputStream::try_default().unwrap();
            let source = rodio::source::SineWave::new(frequency)
            .amplify(100.0)
            .take_duration(Duration::new(time, 0));
            device.play_raw(source.convert_samples()).unwrap();
            std::thread::sleep(Duration::new(time, 0));
        });
    }
}


pub fn main() -> iced::Result {
    println!("All music is at 120bpm, 4/4 time");
    
    iced::application("namne", Program::update, Program::view) 
    .subscription(Program::subscription)
    .theme(|_| Theme::TokyoNight)
    .run()

}

struct Program { 
    octave: f32,
    bpm: f32,
    custom_bpm: String
} 

#[derive(Debug, Clone)]
enum Message { 
    OctaveChange(f32),
    BpmChange(f32),
    CustomBpmChange(String),
    Play
}

impl Program { 
    fn view(&self) -> Element<Message> { 
        container(widget::column![
            widget::row!(
                text("Octave"),
                slider(
                    0.0..=10.0,
                    self.octave,
                    Message::OctaveChange
                ),
            ),

            widget::row!(
                text("BPM"),
                slider(
                    10.0..=300.0,
                    self.bpm, 
                    Message::BpmChange
                ),
                text_input(format!("{}", &self.bpm).as_str(), &self.custom_bpm)
                .on_input(Message::CustomBpmChange) 
                .width(Length::Fixed(50.0)),
            ),

            widget::row!(
                button("Play")
                .on_press(Message::Play)
                .width(Length::Fixed(150.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),             
            ),

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
            Message::OctaveChange(value) => {
                self.octave = value;
            }

            Message::CustomBpmChange(value) => {
                self.bpm = value.parse().expect("Failed to convert");
            }

            Message::BpmChange(value) => {
                self.bpm = value;
            }

            Message::Play => {
                RealNote::play(RealNote{
                    note: Note::C, 
                    length: NoteLength::Whole,
                    octave: self.octave
                }, self.bpm);
                std::thread::sleep(Duration::new(3, 0));
                RealNote::play(RealNote{
                    note: Note::E, 
                    length: NoteLength::Whole,
                    octave: self.octave,
                }, self.bpm);
                std::thread::sleep(Duration::new(3, 0));
                RealNote::play(RealNote{
                    note: Note::G, 
                    length: NoteLength::Whole,
                    octave: self.octave,
                }, self.bpm);
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
            octave: 0.0,
            bpm: 120.0,
            custom_bpm: "120".to_string()
        }
    }
}
