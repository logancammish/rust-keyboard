#![windows_subsystem = "windows"]

use iced::widget::{button, container, slider, text, text_input};
use iced::{Theme, Element, widget, Length, Subscription};
use palette::num::Real;
use rodio::{self, Source};
use std::{f32, thread, time::Duration};
use threadpool::ThreadPool;



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
    fn play(self, bpm: f32) {  
        let time = NoteLength::duration_in_seconds(&self.length, bpm);
        let frequency: f32 = Self::base_frequencies(self.note) * 2_f32.powf(self.octave);
        println!("Playing: {}hz | Time: {}s", frequency, time);
        let (_stream, device) = rodio::OutputStream::try_default().unwrap();
        let source = rodio::source::SineWave::new(frequency)
        .amplify(100.0)
        .take_duration(Duration::from_secs_f32(time));

        let sink = rodio::Sink::try_new(&device).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}

fn async_play_note(notes: Vec<RealNote>, bpm: f32) {
    println!("Playing notes: {:?}", notes); 
    let pool = ThreadPool::new(notes.len());
    for note in notes { 
        pool.execute(move || {
            note.play(bpm);
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
                    0.0..=5.0,
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
                if let Ok(value) = value.parse() {
                    if NoteLength::check_bpm(value) == true  {
                        self.bpm = value;
                        self.custom_bpm = value.to_string();
                    } else {
                        self.bpm = 60.0;
                        self.custom_bpm = "60".to_string();
                    }
                } 
            }

            Message::BpmChange(value) => {
                if NoteLength::check_bpm(value) == true  {
                    self.bpm = value;
                    self.custom_bpm = value.to_string();
                } else {
                    self.bpm = 60.0;
                    self.custom_bpm = "60".to_string();
                }
            }

            Message::Play => {
                RealNote::play(RealNote{
                    note: Note::C, 
                    length: NoteLength::Whole,
                    octave: self.octave
                }, self.bpm);
                //std::thread::sleep(Duration::new(2, 0));
                RealNote::play(RealNote{
                    note: Note::E, 
                    length: NoteLength::Half,
                    octave: self.octave,
                }, self.bpm);
                RealNote::play(RealNote{
                    note: Note::G, 
                    length: NoteLength::Quarter,
                    octave: self.octave,
                }, self.bpm);

                std::thread::sleep(Duration::new(3, 0));


                let c_chord: Vec<RealNote> = Vec::from([
                    RealNote { note: Note::C, length: NoteLength::Whole, octave: self.octave },
                    RealNote { note: Note::E, length: NoteLength::Half, octave: self.octave },
                    RealNote { note: Note::G, length: NoteLength::Quarter, octave: self.octave },
                ]);

                async_play_note(c_chord, self.bpm);
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
