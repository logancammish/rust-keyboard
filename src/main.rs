use iced::{ widget::{button, container, slider}, * };
use rodio::{self, source, Source};
use std::{collections::HashMap, time::Duration};
use std::thread;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Note { 
    A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp
}

enum NoteLength { 
    Whole, Half, Quarter, Eighth, Sixteenth
}

impl NoteLength { 
    pub fn duration_in_seconds(&self) -> f32 {
        match self {
            NoteLength::Whole => (60.0 / 120.0) * 4.0,      
            NoteLength::Half => (60.0 / 120.0) * 2.0,        
            NoteLength::Quarter => 60.0 / 120.0,            
            NoteLength::Eighth => (60.0 / 120.0) * 0.5,      
            NoteLength::Sixteenth => (60.0 / 120.0) * 0.25,  
        }
    }
}

struct RealNote { 
    note: Note, 
    length: NoteLength, 
    octave: f32, 
}


impl RealNote { 
    pub fn base_frequencies(note: Note) -> Option<f32> { 
        match note {
            Note::C => Some(16.35),
            Note::Csharp => Some(17.32),
            Note::D => Some(18.35),
            Note::Dsharp => Some(19.45),
            Note::E => Some(20.60),
            Note::F => Some(21.83),
            Note::Fsharp => Some(23.12),
            Note::G => Some(24.50),
            Note::Gsharp => Some(25.96),
            Note::A => Some(27.50),
            Note::Asharp => Some(29.14),
            Note::B => Some(30.87),
            _ => None,
        }
    }
    fn play(self) {  
        thread::spawn(move || {   
            let frequency: f32 = Self::base_frequencies(self.note).expect("Invalid Note") * (&self.octave + 1.0);
            let (_stream, device) = rodio::OutputStream::try_default().unwrap();
            let source = rodio::source::SineWave::new(frequency)
            .take_duration(Duration::new(2, 0)); 
            device.play_raw(source.convert_samples()).unwrap();
            std::thread::sleep(Duration::new(NoteLength::duration_in_seconds(&self.length) as u64, 0));
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
    slided: f32
} 

#[derive(Debug, Clone, Copy)]
enum Message { 
    Sliding(f32),
    Play
}

impl Program { 
    fn view(&self) -> Element<Message> { 
        container(widget::column![
            widget::row!(
                slider(
                    0.0..=6.0,
                    self.slided,
                    Message::Sliding
                ),
            ),

            widget::row!(
                button("Play")
                .on_press(Message::Play)
                .width(Length::Fixed(150.0)) 
                .height(Length::Fixed(50.0))  
                .padding(10),             
            ),

        ].spacing(10))
        .padding(10)
        .into()
    }
    fn update(&mut self, message: Message) { 
        match message { 
            Message::Sliding(value) => {
                self.slided = value;
            }

            Message::Play => {
                RealNote::play(RealNote{
                    note: Note::C, 
                    length: NoteLength::Whole,
                    octave: self.slided
                });
                RealNote::play(RealNote{
                    note: Note::E, 
                    length: NoteLength::Whole,
                    octave: self.slided
                });
                RealNote::play(RealNote{
                    note: Note::G, 
                    length: NoteLength::Whole,
                    octave: self.slided
                });
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
            slided: 0.0
        }
    }
}
