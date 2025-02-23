use iced::{widget, Length};
use iced::widget::{button, checkbox, container, slider, text, text_input, Container};

use crate::{Message, Note, Program};

impl Program { 
    pub fn get_ui_information(&self) -> Container<'static, Message> {
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
                checkbox("Play aynchronously", self.play_async)
                    .on_toggle(|_| Message::PlayAsync),
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
    }
}
